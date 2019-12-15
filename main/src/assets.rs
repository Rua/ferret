use derivative::Derivative;
use std::{
	clone::Clone,
	collections::{HashMap, VecDeque},
	error::Error,
	marker::PhantomData,
	sync::Arc,
};
use specs::{System, Write};

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
	/*pub fn downgrade(&self) -> WeakHandle<A> {
		let id = Arc::downgrade(&self.id);

		WeakHandle {
			id,
			marker: PhantomData,
		}
	}*/

	fn id(&self) -> u32 {
		*self.id.as_ref()
	}

	fn is_unique(&self) -> bool {
        Arc::strong_count(&self.id) == 1
    }
}

/*#[derive(Derivative)]
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
}*/

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
pub struct AssetStorage<A> {
	assets: HashMap<u32, A>,
	handles: Vec<AssetHandle<A>>,
	highest_id: u32,
	unused_ids: VecDeque<u32>,
}

impl<A> AssetStorage<A> {
	pub fn get(&self, handle: &AssetHandle<A>) -> Option<&A> {
		self.assets.get(&handle.id())
	}

	pub fn insert(&mut self, asset: A) -> AssetHandle<A> {
		let id = self.unused_ids.pop_front().unwrap_or_else(|| {
			self.highest_id += 1;
			self.highest_id
		});

		self.assets.insert(id, asset);
		let handle = AssetHandle {
			id: Arc::new(id),
			marker: PhantomData,
		};

		self.handles.push(handle.clone());
		handle
	}

	pub fn maintain(&mut self) {
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
			trace!("Freed {} assets of type {}", count, std::any::type_name::<A>());
		}
	}
}

pub trait DataSource {
	fn load(&mut self, path: &str) -> Result<Vec<u8>, Box<dyn Error>>;
	fn names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a>;
}

pub trait AssetFormat {
	type Asset;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>>;
}

pub struct AssetMaintenanceSystem<A>(PhantomData<A>);

impl<A> AssetMaintenanceSystem<A> {
	pub fn new() -> AssetMaintenanceSystem<A> {
		AssetMaintenanceSystem(PhantomData)
	}
}

impl<'a, A:  Send + Sync + 'static> System<'a> for AssetMaintenanceSystem<A> {
	type SystemData = Write<'a, AssetStorage<A>>;

	fn run(&mut self, mut storage: Self::SystemData) {
		storage.maintain();
	}
}
