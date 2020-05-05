use derivative::Derivative;
use std::{
	clone::Clone,
	collections::{HashMap, VecDeque},
	marker::PhantomData,
	sync::{Arc, Weak},
};

pub trait Asset: Send + Sync + 'static {
	type Data: Send + Sync + 'static;
	type Intermediate: Send + Sync + 'static;
	const NAME: &'static str;

	fn import(name: &str, source: &impl DataSource) -> anyhow::Result<Self::Intermediate>;
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

/*#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct AssetCache<A> {
	map: HashMap<String, WeakHandle<A>>,
}

impl<A> AssetCache<A> {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn insert<K: Into<String>>(
		&mut self,
		key: K,
		asset: &AssetHandle<A>,
	) -> Option<WeakHandle<A>> {
		self.map.insert(key.into(), asset.downgrade())
	}

	pub fn get<K>(&self, key: &K) -> Option<AssetHandle<A>>
	where
		K: ?Sized + Hash + Eq,
		String: Borrow<K>,
	{
		self.map.get(key).and_then(WeakHandle::upgrade)
	}
}*/

#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct AssetStorage<A: Asset> {
	assets: HashMap<u32, A::Data>,
	handles: Vec<AssetHandle<A>>,
	highest_id: u32,
	names: HashMap<String, WeakHandle<A>>,
	unbuilt: Vec<(AssetHandle<A>, anyhow::Result<A::Intermediate>, String)>,
	unused_ids: VecDeque<u32>,
}

impl<A: Asset> AssetStorage<A> {
	#[inline]
	pub fn get(&self, handle: &AssetHandle<A>) -> Option<&A::Data> {
		self.assets.get(&handle.id())
	}

	#[inline]
	fn allocate_handle(&mut self) -> AssetHandle<A> {
		let id = self.unused_ids.pop_front().unwrap_or_else(|| {
			self.highest_id += 1;
			self.highest_id
		});

		AssetHandle {
			id: Arc::new(id),
			marker: PhantomData,
		}
	}

	#[inline]
	pub fn handle_for(&self, name: &str) -> Option<AssetHandle<A>> {
		self.names.get(name).and_then(WeakHandle::upgrade)
	}

	#[inline]
	pub fn insert(&mut self, data: A::Data) -> AssetHandle<A> {
		let handle = self.allocate_handle();
		self.assets.insert(handle.id(), data);
		self.handles.push(handle.clone());
		handle
	}

	pub fn load(&mut self, name: &str, source: &mut impl DataSource) -> AssetHandle<A> {
		self.handle_for(name).unwrap_or_else(|| {
			let handle = self.allocate_handle();
			self.names.insert(name.to_owned(), handle.downgrade());
			let intermediate = A::import(name, source);
			self.unbuilt
				.push((handle.clone(), intermediate, name.to_owned()));

			handle
		})
	}

	/*pub fn clear_unused(&mut self) {
		let assets = &mut self.assets;
		let unused_ids = &mut self.unused_ids;
		let old_len = self.handles.len();

		self.handles.retain(|handle| {
			if handle.is_unique() {
				assets.remove(&handle.id());
				unused_ids.push_back(handle.id());
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

	pub fn build_waiting<F: FnMut(A::Intermediate) -> anyhow::Result<A::Data>>(
		&mut self,
		mut build_func: F,
	) {
		for (handle, data, name) in self.unbuilt.drain(..) {
			let asset = match data.and_then(|d| build_func(d)) {
				Ok(asset) => {
					log::trace!("{} '{}' loaded", A::NAME, name);
					asset
				}
				Err(e) => {
					log::error!("{} '{}' could not be loaded: {}", A::NAME, name, e);
					continue;
				}
			};

			self.assets.insert(handle.id(), asset);
			self.handles.push(handle);
		}
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
