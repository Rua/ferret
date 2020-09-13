use derivative::Derivative;
use downcast_rs::{impl_downcast, DowncastSync};
use fnv::FnvHashMap;
use std::{
	any::{Any, TypeId},
	clone::Clone,
	marker::PhantomData,
	sync::{Arc, Weak},
};

pub trait Asset: Send + Sync + 'static {
	type Data: Send + Sync + 'static;
	const NAME: &'static str;
	const NEEDS_PROCESSING: bool;

	fn import(name: &str, asset_storage: &mut AssetStorage) -> anyhow::Result<Box<dyn ImportData>>;
}

pub trait ImportData: DowncastSync {}
impl_downcast!(sync ImportData);
impl<T: DowncastSync> ImportData for T {}

pub struct AssetStorage {
	source: Box<dyn DataSource>,
	storages: FnvHashMap<TypeId, Box<dyn Any + Send + Sync>>,
	handle_allocator: HandleAllocator,
}

impl AssetStorage {
	#[inline]
	pub fn new(source: impl DataSource) -> AssetStorage {
		AssetStorage {
			source: Box::new(source),
			storages: FnvHashMap::default(),
			handle_allocator: HandleAllocator::default(),
		}
	}

	#[inline]
	pub fn source(&self) -> &dyn DataSource {
		&*self.source
	}

	#[inline]
	pub fn add_storage<A: Asset>(&mut self) {
		self.storages.insert(
			TypeId::of::<A>(),
			Box::new(AssetStorageTyped::<A>::default()),
		);
	}

	#[inline]
	pub fn get<A: Asset>(&self, handle: &AssetHandle<A>) -> Option<&A::Data> {
		let storage = storage::<A>(&self.storages);
		storage.assets.get(&handle.id())
	}

	#[inline]
	pub fn iter<A: Asset>(&self) -> impl Iterator<Item = (&AssetHandle<A>, &A::Data)> {
		let storage = storage::<A>(&self.storages);
		storage
			.handles
			.iter()
			.map(move |handle| (handle, storage.assets.get(&handle.id()).unwrap()))
	}

	#[inline]
	pub fn handle_for<A: Asset>(&self, name: &str) -> Option<AssetHandle<A>> {
		let storage = storage::<A>(&self.storages);
		storage.names.get(name).and_then(WeakHandle::upgrade)
	}

	#[inline]
	pub fn get_by_name<A: Asset>(&self, name: &str) -> Option<&A::Data> {
		let storage = storage::<A>(&self.storages);
		storage
			.names
			.get(name)
			.and_then(WeakHandle::upgrade)
			.and_then(|handle| storage.assets.get(&handle.id()))
	}

	#[inline]
	pub fn insert<A: Asset>(&mut self, asset: A::Data) -> AssetHandle<A> {
		let handle = self.handle_allocator.allocate();
		let storage = storage_mut::<A>(&mut self.storages);
		storage.assets.insert(handle.id(), asset);
		storage.handles.push(handle.clone());
		handle
	}

	#[inline]
	pub fn insert_with_name<A: Asset>(&mut self, name: &str, asset: A::Data) -> AssetHandle<A> {
		let storage = storage_mut::<A>(&mut self.storages);
		match storage.names.get(name).and_then(WeakHandle::upgrade) {
			Some(handle) => {
				storage.assets.insert(handle.id(), asset);
				handle
			}
			None => {
				let handle = {
					let handle = self.handle_allocator.allocate();
					storage.assets.insert(handle.id(), asset);
					storage.handles.push(handle.clone());
					handle
				};
				storage.names.insert(name.to_owned(), handle.downgrade());
				handle
			}
		}
	}

	#[inline]
	pub fn load<A: Asset>(&mut self, name: &str) -> AssetHandle<A> {
		match storage_mut::<A>(&mut self.storages)
			.names
			.get(name)
			.and_then(WeakHandle::upgrade)
		{
			Some(handle) => handle,
			None => {
				let handle = self.handle_allocator.allocate();
				let import_result = A::import(name, self);

				let storage = storage_mut::<A>(&mut self.storages);
				storage.names.insert(name.to_owned(), handle.downgrade());

				if A::NEEDS_PROCESSING {
					storage
						.unprocessed
						.push((handle.clone(), import_result, name.to_owned()));
				} else {
					let data = import_result.unwrap();
					let asset: A::Data = *data.downcast().ok().unwrap();
					storage.assets.insert(handle.id(), asset);
				}

				handle
			}
		}
	}

	#[inline]
	pub fn process<
		A: Asset,
		F: FnMut(Box<dyn ImportData>, &mut AssetStorage) -> anyhow::Result<A::Data>,
	>(
		&mut self,
		mut process_func: F,
	) {
		assert!(A::NEEDS_PROCESSING);

		let unprocessed = if let Some(entry) = self.storages.get_mut(&TypeId::of::<A>()) {
			let storage = entry.downcast_mut::<AssetStorageTyped<A>>().unwrap();
			std::mem::replace(&mut storage.unprocessed, Vec::new())
		} else {
			return;
		};

		for (handle, data, name) in unprocessed {
			// Build the asset
			let asset = match data.and_then(|d| process_func(d, self)) {
				Ok(asset) => {
					log::trace!("{} '{}' loaded", A::NAME, name);
					asset
				}
				Err(e) => {
					log::error!("{} '{}' could not be loaded: {}", A::NAME, name, e);
					continue;
				}
			};

			// Insert it into the storage
			{
				let storage = storage_mut::<A>(&mut self.storages);
				storage.assets.insert(handle.id(), asset);
				storage.handles.push(handle);
			}
		}
	}
}

#[inline]
fn storage<A: Asset>(
	storages: &FnvHashMap<TypeId, Box<dyn Any + Send + Sync>>,
) -> &AssetStorageTyped<A> {
	storages
		.get(&TypeId::of::<A>())
		.expect("unknown asset type")
		.downcast_ref::<AssetStorageTyped<A>>()
		.expect("failed to downcast")
}

#[inline]
fn storage_mut<A: Asset>(
	storages: &mut FnvHashMap<TypeId, Box<dyn Any + Send + Sync>>,
) -> &mut AssetStorageTyped<A> {
	storages
		.get_mut(&TypeId::of::<A>())
		.expect("unknown asset type")
		.downcast_mut::<AssetStorageTyped<A>>()
		.expect("failed to downcast")
}

#[derive(Derivative)]
#[derivative(Default(bound = ""))]
struct AssetStorageTyped<A: Asset> {
	assets: FnvHashMap<u64, A::Data>,
	handles: Vec<AssetHandle<A>>,
	names: FnvHashMap<String, WeakHandle<A>>,
	unprocessed: Vec<(AssetHandle<A>, anyhow::Result<Box<dyn ImportData>>, String)>,
}

#[derive(Derivative)]
#[derivative(
	Clone(bound = ""),
	Eq(bound = ""),
	Hash(bound = ""),
	PartialEq(bound = ""),
	Debug(bound = "")
)]
pub struct AssetHandle<A: ?Sized> {
	id: Arc<u64>,
	marker: PhantomData<A>,
}

impl<A> AssetHandle<A> {
	pub fn downgrade(&self) -> WeakHandle<A> {
		let id = Arc::downgrade(&self.id);

		WeakHandle {
			id,
			marker: PhantomData,
		}
	}

	fn id(&self) -> u64 {
		*self.id.as_ref()
	}

	/*fn is_unique(&self) -> bool {
		Arc::strong_count(&self.id) == 1
	}*/
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct WeakHandle<A> {
	id: Weak<u64>,
	marker: PhantomData<A>,
}

impl<A> WeakHandle<A> {
	pub fn upgrade(&self) -> Option<AssetHandle<A>> {
		self.id.upgrade().map(|id| AssetHandle {
			id,
			marker: PhantomData,
		})
	}
}

#[derive(Clone, Debug, Default)]
struct HandleAllocator {
	highest_id: u64,
	unused_ids: Vec<u64>,
}

impl HandleAllocator {
	#[inline]
	fn allocate<A: Asset>(&mut self) -> AssetHandle<A> {
		let id = self.unused_ids.pop().unwrap_or_else(|| {
			self.highest_id += 1;
			self.highest_id
		});

		AssetHandle {
			id: Arc::new(id),
			marker: PhantomData,
		}
	}
}

pub trait DataSource: Send + Sync + 'static {
	fn load(&self, path: &str) -> anyhow::Result<Vec<u8>>;
	fn names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a>;
}
