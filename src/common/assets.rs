use derivative::Derivative;
use downcast_rs::{impl_downcast, DowncastSync};
use fnv::FnvHashMap;
use relative_path::RelativePath;
use std::{
	any::{Any, TypeId},
	clone::Clone,
	marker::PhantomData,
	sync::{Arc, Weak},
};

pub trait Asset: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Asset for T {}

pub trait ImportData: DowncastSync {}
impl_downcast!(sync ImportData);
impl<T: DowncastSync> ImportData for T {}

pub struct AssetStorage {
	importer: fn(
		path: &RelativePath,
		asset_storage: &mut AssetStorage,
	) -> anyhow::Result<Box<dyn ImportData>>,
	source: Box<dyn DataSource>,
	storages: FnvHashMap<TypeId, Box<dyn Any + Send + Sync>>,
	handle_allocator: HandleAllocator,
}

impl AssetStorage {
	#[inline]
	pub fn new(
		importer: fn(
			path: &RelativePath,
			asset_storage: &mut AssetStorage,
		) -> anyhow::Result<Box<dyn ImportData>>,
		source: impl DataSource,
	) -> AssetStorage {
		AssetStorage {
			importer,
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
	pub fn add_storage<A: Asset>(&mut self, needs_processing: bool) {
		let mut storage = AssetStorageTyped::<A>::default();

		if needs_processing {
			storage.unprocessed = Some(Vec::new());
		}

		self.storages.insert(TypeId::of::<A>(), Box::new(storage));
	}

	#[inline]
	pub fn get<A: Asset>(&self, handle: &AssetHandle<A>) -> Option<&A> {
		let storage = storage::<A>(&self.storages);
		storage.assets.get(&handle.id())
	}

	#[inline]
	pub fn iter<A: Asset>(&self) -> impl Iterator<Item = (&AssetHandle<A>, &A)> {
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
	pub fn get_by_name<A: Asset>(&self, name: &str) -> Option<&A> {
		let storage = storage::<A>(&self.storages);
		storage
			.names
			.get(name)
			.and_then(WeakHandle::upgrade)
			.and_then(|handle| storage.assets.get(&handle.id()))
	}

	#[inline]
	pub fn insert<A: Asset>(&mut self, asset: A) -> AssetHandle<A> {
		let handle = self.handle_allocator.allocate();
		let storage = storage_mut::<A>(&mut self.storages);
		storage.assets.insert(handle.id(), asset);
		storage.handles.push(handle.clone());
		handle
	}

	#[inline]
	pub fn insert_with_name<A: Asset>(&mut self, name: &str, asset: A) -> AssetHandle<A> {
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
				let import_result = (self.importer)(RelativePath::new(name), self);

				let storage = storage_mut::<A>(&mut self.storages);
				storage.names.insert(name.to_owned(), handle.downgrade());

				if let Some(unprocessed) = &mut storage.unprocessed {
					unprocessed.push((handle.clone(), import_result, name.to_owned()));
				} else {
					match import_result {
						Ok(data) => {
							log::trace!("Loaded '{}'", name);
							let asset = *data.downcast().ok().unwrap();
							storage.assets.insert(handle.id(), asset);
						}
						Err(e) => {
							log::error!("'{}' could not be loaded: {}", name, e);
						}
					};
				}

				handle
			}
		}
	}

	#[inline]
	pub fn process<
		A: Asset,
		F: FnMut(Box<dyn ImportData>, &mut AssetStorage) -> anyhow::Result<A>,
	>(
		&mut self,
		mut process_func: F,
	) {
		let unprocessed =
			if let Some(unprocessed) = &mut storage_mut::<A>(&mut self.storages).unprocessed {
				std::mem::replace(unprocessed, Vec::new())
			} else {
				return;
			};

		for (handle, data, name) in unprocessed {
			// Build the asset
			let asset = match data.and_then(|d| process_func(d, self)) {
				Ok(asset) => {
					log::trace!("Loaded '{}'", name);
					asset
				}
				Err(e) => {
					log::error!("'{}' could not be loaded: {}", name, e);
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
	assets: FnvHashMap<u64, A>,
	handles: Vec<AssetHandle<A>>,
	names: FnvHashMap<String, WeakHandle<A>>,
	unprocessed: Option<Vec<(AssetHandle<A>, anyhow::Result<Box<dyn ImportData>>, String)>>,
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
	fn load(&self, path: &RelativePath) -> anyhow::Result<Vec<u8>>;
	fn names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a>;
}
