use legion::storage::{
	Archetype, ArchetypeWriter, Component, ComponentStorage, ComponentTypeId, Components,
	EntityLayout,
};
use legion::{world::Merger, Resources};
use std::{collections::HashMap, ops::Range};

pub trait SpawnFrom<FromT: Sized>
where
	Self: Sized,
{
	fn spawn(
		component: &FromT,
		accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self;
}

/// A Merger implementation that passes Resources and ComponentAccessor to the closure
pub struct SpawnMerger<'a, 'b> {
	pub handler_set: &'a SpawnMergerHandlerSet,
	pub resources: &'b Resources,
}

impl<'a, 'b> SpawnMerger<'a, 'b> {
	pub fn new(handler_set: &'a SpawnMergerHandlerSet, resources: &'b Resources) -> Self {
		Self {
			handler_set,
			resources,
		}
	}
}

impl<'a, 'b> Merger for SpawnMerger<'a, 'b> {
	fn convert_layout(&mut self, source_layout: EntityLayout) -> EntityLayout {
		let mut dest_layout = EntityLayout::default();
		for component_type in source_layout.component_types() {
			let (_, register_fn) = &self.handler_set.handlers[&component_type];
			register_fn(&mut dest_layout);
		}

		dest_layout
	}

	fn merge_archetype(
		&mut self,
		src_entity_range: Range<usize>,
		src_arch: &Archetype,
		src_components: &Components,
		dst: &mut ArchetypeWriter,
	) {
		for src_type in src_arch.layout().component_types() {
			let (merge_fn, _) = &self.handler_set.handlers[&src_type];
			merge_fn(
				self.resources,
				src_entity_range.clone(),
				src_arch,
				src_components,
				dst,
			);
		}
	}
}

#[derive(Default)]
pub struct SpawnMergerHandlerSet {
	handlers: HashMap<
		ComponentTypeId,
		(
			Box<
				dyn Fn(
					&Resources,           // resources
					Range<usize>,         // src_entity_range
					&Archetype,           // src_arch
					&Components,          // src_components
					&mut ArchetypeWriter, // dst
				),
			>,
			Box<dyn Fn(&mut EntityLayout)>,
		),
	>,
}

impl SpawnMergerHandlerSet {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn register_clone<FromT>(&mut self)
	where
		FromT: Component + Clone,
	{
		self.register_closure::<FromT, FromT, _>(|component, _, _| {
			<FromT as Clone>::clone(component)
		})
	}

	pub fn register_from<FromT, IntoT>(&mut self)
	where
		FromT: Component + Clone,
		IntoT: Component + From<FromT>,
	{
		self.register_closure::<FromT, IntoT, _>(|component, _, _| FromT::clone(component).into())
	}

	pub fn register_spawn<FromT, IntoT>(&mut self)
	where
		FromT: Component,
		IntoT: Component + SpawnFrom<FromT>,
	{
		self.register_closure::<FromT, IntoT, _>(|component, accessor, resources| {
			IntoT::spawn(component, accessor, resources)
		})
	}

	pub fn register_closure<FromT, IntoT, F>(&mut self, clone_fn: F)
	where
		FromT: Component,
		IntoT: Component,
		F: Fn(&FromT, ComponentAccessor, &Resources) -> IntoT + 'static,
	{
		let merge_fn = move |resources: &Resources,
		                     src_entity_range: Range<usize>,
		                     src_arch: &Archetype,
		                     src_components: &Components,
		                     dst: &mut ArchetypeWriter| {
			let src = src_components.get_downcast::<FromT>().unwrap();
			let src_slice = &src.get(src_arch.index()).unwrap().into_slice();

			let mut dst = dst.claim_components::<IntoT>();
			dst.ensure_capacity(src_entity_range.len());

			for i in src_entity_range {
				let component = &src_slice[i];
				let accessor = ComponentAccessor {
					archetype: src_arch,
					components: src_components,
					index: i,
				};
				let dst_component = clone_fn(component, accessor, resources);

				unsafe {
					dst.extend_memcopy(&dst_component as *const IntoT, 1);
					std::mem::forget(dst_component);
				}
			}
		};
		let register_fn =
			|entity_layout: &mut EntityLayout| entity_layout.register_component::<IntoT>();

		self.handlers.insert(
			ComponentTypeId::of::<FromT>(),
			(Box::new(merge_fn), Box::new(register_fn)),
		);
	}
}

/// Gives SpawnFrom access to the components of the current source entity
pub struct ComponentAccessor<'a> {
	archetype: &'a Archetype,
	components: &'a Components,
	index: usize,
}

impl<'a> ComponentAccessor<'a> {
	pub fn get<T: Component>(&self) -> Option<&T> {
		self.components
			.get_downcast::<T>()
			.and_then(|storage| storage.get(self.archetype.index()))
			.and_then(|slice| slice.into_slice().get(self.index))
	}
}
