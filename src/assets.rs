use derivative::Derivative;
use fnv::FnvHashMap;
use std::{
	any::{Any, TypeId},
	clone::Clone,
	marker::PhantomData,
	sync::{Arc, Weak},
};

pub trait Asset: Send + Sync + 'static {
	type Data: Send + Sync + 'static;
	type Intermediate: Send + Sync + 'static;
	const NAME: &'static str;

	fn import(name: &str, source: &impl DataSource) -> anyhow::Result<Self::Intermediate>;
}

#[derive(Default)]
pub struct AssetStorage {
	storages: FnvHashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl AssetStorage {
	#[inline]
	pub fn get<A: Asset>(&self, handle: &AssetHandle<A>) -> Option<&A::Data> {
		self.storages.get(&TypeId::of::<A>()).and_then(|entry| {
			entry
				.downcast_ref::<AssetStorageTyped<A>>()
				.unwrap()
				.get(handle)
		})
	}

	#[inline]
	pub fn handle_for<A: Asset>(&self, name: &str) -> Option<AssetHandle<A>> {
		self.storages.get(&TypeId::of::<A>()).and_then(|entry| {
			entry
				.downcast_ref::<AssetStorageTyped<A>>()
				.unwrap()
				.handle_for(name)
		})
	}

	#[inline]
	pub fn insert<A: Asset>(&mut self, data: A::Data) -> AssetHandle<A> {
		self.storages
			.entry(TypeId::of::<A>())
			.or_insert_with(|| Box::new(AssetStorageTyped::<A>::default()))
			.downcast_mut::<AssetStorageTyped<A>>()
			.unwrap()
			.insert(data)
	}

	#[inline]
	pub fn load<A: Asset>(&mut self, name: &str, source: &mut impl DataSource) -> AssetHandle<A> {
		self.storages
			.entry(TypeId::of::<A>())
			.or_insert_with(|| Box::new(AssetStorageTyped::<A>::default()))
			.downcast_mut::<AssetStorageTyped<A>>()
			.unwrap()
			.load(name, source)
	}

	#[inline]
	pub fn build_waiting<
		A: Asset,
		F: FnMut(A::Intermediate, &mut AssetStorage) -> anyhow::Result<A::Data>,
	>(
		&mut self,
		mut build_func: F,
	) {
		let unbuilt = if let Some(entry) = self.storages.get_mut(&TypeId::of::<A>()) {
			let storage = entry.downcast_mut::<AssetStorageTyped<A>>().unwrap();
			std::mem::replace(&mut storage.unbuilt, Vec::new())
		} else {
			return;
		};

		for (handle, data, name) in unbuilt {
			// Build the asset
			let asset = match data.and_then(|d| build_func(d, self)) {
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
				let storage = self
					.storages
					.get_mut(&TypeId::of::<A>())
					.unwrap()
					.downcast_mut::<AssetStorageTyped<A>>()
					.unwrap();
				storage.assets.insert(handle.id(), asset);
				storage.handles.push(handle);
			}
		}
	}
}

#[derive(Derivative)]
#[derivative(Default(bound = ""))]
struct AssetStorageTyped<A: Asset> {
	assets: FnvHashMap<u32, A::Data>,
	handles: Vec<AssetHandle<A>>,
	highest_id: u32,
	names: FnvHashMap<String, WeakHandle<A>>,
	unbuilt: Vec<(AssetHandle<A>, anyhow::Result<A::Intermediate>, String)>,
	unused_ids: Vec<u32>,
}

impl<A: Asset> AssetStorageTyped<A> {
	#[inline]
	fn get(&self, handle: &AssetHandle<A>) -> Option<&A::Data> {
		self.assets.get(&handle.id())
	}

	#[inline]
	fn handle_for(&self, name: &str) -> Option<AssetHandle<A>> {
		self.names.get(name).and_then(WeakHandle::upgrade)
	}

	#[inline]
	fn allocate_handle(&mut self) -> AssetHandle<A> {
		let id = self.unused_ids.pop().unwrap_or_else(|| {
			self.highest_id += 1;
			self.highest_id
		});

		AssetHandle {
			id: Arc::new(id),
			marker: PhantomData,
		}
	}

	#[inline]
	fn insert(&mut self, data: A::Data) -> AssetHandle<A> {
		let handle = self.allocate_handle();
		self.assets.insert(handle.id(), data);
		self.handles.push(handle.clone());
		handle
	}

	fn load(&mut self, name: &str, source: &mut impl DataSource) -> AssetHandle<A> {
		self.handle_for(name).unwrap_or_else(|| {
			let handle = self.allocate_handle();
			self.names.insert(name.to_owned(), handle.downgrade());
			let intermediate = A::import(name, source);
			self.unbuilt
				.push((handle.clone(), intermediate, name.to_owned()));

			handle
		})
	}

	/*fn clear_unused(&mut self) {
		let assets = &mut self.assets;
		let unused_ids = &mut self.unused_ids;
		let old_len = self.handles.len();

		self.handles.retain(|handle| {
			if handle.is_unique() {
				assets.remove(&handle.id());
				unused_ids.push(handle.id());
				false
			} else {
				true
			}
		});

		let count = old_len - self.handles.len();

		if count > 0 {
			log::trace!("Freed {} {} assets", count, A::NAME);
		}
	}*/
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
	id: Arc<u32>,
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

	fn id(&self) -> u32 {
		*self.id.as_ref()
	}

	/*fn is_unique(&self) -> bool {
		Arc::strong_count(&self.id) == 1
	}*/
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct WeakHandle<A> {
	id: Weak<u32>,
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

pub trait AssetFormat: Clone {
	type Asset;

	fn import(&self, name: &str, source: &impl DataSource) -> anyhow::Result<Self::Asset>;
}

pub trait DataSource {
	fn load(&self, path: &str) -> anyhow::Result<Vec<u8>>;
	fn names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a>;
}
