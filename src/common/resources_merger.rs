use legion::storage::{
	Archetype, ArchetypeWriter, Component, ComponentStorage, ComponentTypeId, Components,
	EntityLayout,
};
use legion::{world::Merger, Resources};
use std::{collections::HashMap, ops::Range};

pub trait FromWithResources<FromT: Sized>
where
	Self: Sized,
{
	fn from_with_resources(src_component: &FromT, resources: &Resources) -> Self;
}

/// A Merger implementation that passes Resources to the closure
pub struct ResourcesMerger<'a, 'b> {
	pub handler_set: &'a ResourcesMergerHandlerSet,
	pub resources: &'b Resources,
}

impl<'a, 'b> ResourcesMerger<'a, 'b> {
	pub fn new(handler_set: &'a ResourcesMergerHandlerSet, resources: &'b Resources) -> Self {
		Self {
			handler_set,
			resources,
		}
	}
}

impl<'a, 'b> Merger for ResourcesMerger<'a, 'b> {
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
pub struct ResourcesMergerHandlerSet {
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

impl ResourcesMergerHandlerSet {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn register_clone<FromT>(&mut self)
	where
		FromT: Component + Clone,
	{
		self.register_closure::<FromT, FromT, _>(|_, src_component| {
			<FromT as Clone>::clone(src_component)
		})
	}

	pub fn register_from<FromT, IntoT>(&mut self)
	where
		FromT: Component + Clone,
		IntoT: Component + From<FromT>,
	{
		self.register_closure::<FromT, IntoT, _>(|_, src_component| {
			FromT::clone(src_component).into()
		})
	}

	pub fn register_from_with_resources<FromT, IntoT>(&mut self)
	where
		FromT: Component,
		IntoT: Component + FromWithResources<FromT>,
	{
		self.register_closure::<FromT, IntoT, _>(|resources, src_component| {
			IntoT::from_with_resources(src_component, resources)
		})
	}

	pub fn register_closure<FromT, IntoT, F>(&mut self, clone_fn: F)
	where
		FromT: Component,
		IntoT: Component,
		F: Fn(&Resources, &FromT) -> IntoT + 'static,
	{
		let merge_fn = move |resources: &Resources,
		                     src_entity_range: Range<usize>,
		                     src_arch: &Archetype,
		                     src_components: &Components,
		                     dst: &mut ArchetypeWriter| {
			let src = src_components.get_downcast::<FromT>().unwrap();
			let mut dst = dst.claim_components::<IntoT>();

			let src_slice =
				&src.get(src_arch.index()).unwrap().into_slice()[src_entity_range.clone()];
			dst.ensure_capacity(src_slice.len());

			for src_component in src_slice {
				let dst_component = clone_fn(resources, src_component);
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
