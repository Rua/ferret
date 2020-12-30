//! Asset loading and storage.

use derivative::Derivative;
use downcast_rs::{impl_downcast, DowncastSync};
use fnv::FnvHashMap;
use relative_path::RelativePath;
use scoped_tls_hkt::scoped_thread_local;
use serde::{
	de::{Deserialize, Deserializer, Visitor},
	ser::{Serialize, Serializer},
};
use std::{
	any::{Any, TypeId},
	borrow::Cow,
	clone::Clone,
	marker::PhantomData,
	sync::{Arc, Weak},
};

/// A blanket trait for types that can be used as assets.
pub trait Asset: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Asset for T {}

/// A blanket trait for types that can be used as asset import data.
///
/// It implements [`DowncastSync`] so that it can be downcast to a known type.
/// Asset importer functions return a trait object of this type, which is then optionally provided
/// to the processing function in [`AssetStorage::process`].
pub trait ImportData: DowncastSync {}
impl_downcast!(sync ImportData);
impl<T: DowncastSync> ImportData for T {}

/// Types that can load raw asset data from a path.
pub trait DataSource: Send + Sync + 'static {
	/// Loads the asset at the given `path`, and returns the bytes loaded.
	fn load(&self, path: &RelativePath) -> anyhow::Result<Vec<u8>>;

	/// Returns whether an asset exists at the given `path`.
	fn exists(&self, path: &RelativePath) -> bool;

	/// Returns an iterator that yields all the available asset names.
	fn names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a>;
}

/// Loads assets and allows them to be retrieved.
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
	/// Constructs a new `AssetStorage`.
	/// The `importer` is a callback function that is called whenever a new asset needs to be
	/// loaded, and should return the imported asset data.
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

	/// Returns a reference to the source that the `AssetStorage` was constructed with.
	#[inline]
	pub fn source(&self) -> &dyn DataSource {
		&*self.source
	}

	/// Adds a storage for the given asset type.
	/// `needs_processing` indicates whether the asset is considered finished after loading,
	/// or needs further processing (for example, uploading to the GPU). If this is `true`,
	/// then the asset will not be available until it is processed with
	/// [`process`](AssetStorage::process).
	#[inline]
	pub fn add_storage<A: Asset>(&mut self, needs_processing: bool) {
		let mut storage = AssetStorageTyped::<A>::default();

		if needs_processing {
			storage.unprocessed = Some(Vec::new());
		}

		self.storages.insert(TypeId::of::<A>(), Box::new(storage));
	}

	/// Returns the asset associated with the given `handle`,
	/// or `None` if it does not exist in the storage.
	#[inline]
	pub fn get<A: Asset>(&self, handle: &AssetHandle<A>) -> Option<&A> {
		let storage = storage::<A>(&self.storages);
		storage.assets.get(&handle.id())
	}

	/// Returns an iterator that iterates over all assets of the given type.
	#[inline]
	pub fn iter<A: Asset>(&self) -> impl Iterator<Item = (&AssetHandle<A>, &A)> {
		let storage = storage::<A>(&self.storages);
		storage
			.handles
			.iter()
			.map(move |handle| (handle, storage.assets.get(&handle.id()).unwrap()))
	}

	/// Returns the handle associated with the given asset name,
	/// or `None` if it does not exist in the storage.
	#[inline]
	pub fn handle_for<A: Asset>(&self, name: &str) -> Option<AssetHandle<A>> {
		let storage = storage::<A>(&self.storages);
		storage.names.get(name).and_then(WeakHandle::upgrade)
	}

	/// Returns the name associated with the given handle,
	/// or `None` if it does not exist in the storage or has no name.
	#[inline]
	pub fn name_for<A: Asset>(&self, handle: &AssetHandle<A>) -> Option<&str> {
		let storage = storage::<A>(&self.storages);
		storage.names.iter().find_map(|(k, v)| {
			if v.upgrade().expect("name refers to deleted asset").id() == handle.id() {
				Some(k.as_str())
			} else {
				None
			}
		})
	}

	/// Inserts the given `asset` into the storage, returning a new handle for it.
	#[inline]
	pub fn insert<A: Asset>(&mut self, asset: A) -> AssetHandle<A> {
		let handle = self.handle_allocator.allocate();
		let storage = storage_mut::<A>(&mut self.storages);
		storage.assets.insert(handle.id(), asset);
		storage.handles.push(handle.clone());
		handle
	}

	/// Inserts the given `asset` into the storage, assigning it the given `name`, and
	/// returning a new handle for it.
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

	/// Loads the asset with the given `name`, returning a new handle for it.
	/// Panics if the asset could not be loaded.
	///
	/// If this asset type's storage was added with `needs_processing` set,
	/// then [`process`](AssetStorage::process) will need to be called before it is available,
	/// otherwise it will be available after `load` completes.
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
							log::trace!("Loaded \"{}\"", name);
							let asset = *data.downcast().ok().unwrap();
							storage.assets.insert(handle.id(), asset);
						}
						Err(e) => {
							panic!("\"{}\" could not be loaded: {}", name, e);
						}
					};
				}

				handle
			}
		}
	}

	/// Processes assets of the given type, using the provided processing function.
	/// This is used with asset types whose storage was added with `needs_processing` set,
	/// it has no effect for others.
	///
	/// The processing function should return the final form of the asset, which will be available
	/// once `process` completes. If it returns `Err`, `process` will panic.
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
					log::trace!("Loaded \"{}\"", name);
					asset
				}
				Err(e) => {
					panic!("\"{}\" could not be loaded: {}", name, e);
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

/// An opaque handle for an asset. The associated asset can be retrieved in the `AssetStorage`.
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

scoped_thread_local!(pub static mut SERDE_CONTEXT: AssetStorage);

impl<A: Asset> Serialize for AssetHandle<A> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		SERDE_CONTEXT.with(|asset_storage| {
			let name = asset_storage.name_for(self).expect("asset has no name");
			serializer.serialize_str(name)
		})
	}
}

impl<'de, A: Asset> Deserialize<'de> for AssetHandle<A> {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<AssetHandle<A>, D::Error> {
		SERDE_CONTEXT.with(|asset_storage| {
			let name = deserializer.deserialize_str(AssetVisitor)?;
			let handle = asset_storage.load(&name);
			Ok(handle)
		})
	}
}

struct AssetVisitor;

impl<'de> Visitor<'de> for AssetVisitor {
	type Value = Cow<'de, str>;

	fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter.write_str("a string")
	}

	fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
		Ok(Cow::Owned(v.to_owned()))
	}

	fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
		Ok(Cow::Borrowed(v))
	}

	fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
		Ok(Cow::Owned(v))
	}
}
