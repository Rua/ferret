use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::{ortho_matrix, Interval, AABB3},
		video::{
			definition::NumberedInstanceBufferDefinition, DrawContext, DrawTarget, RenderContext,
		},
	},
	doom::{
		image::Image,
		ui::{UiAlignment, UiImage, UiTransform},
	},
};
use anyhow::Context;
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, SystemBuilder,
};
use nalgebra::{Vector2, Vector3};
use std::{cmp::Ordering, sync::Arc};
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	descriptor::descriptor_set::FixedSizeDescriptorSetsPool,
	framebuffer::{FramebufferAbstract, Subpass},
	impl_vertex,
	pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
	sampler::Sampler,
};

pub fn draw_ui(resources: &mut Resources) -> anyhow::Result<impl Runnable> {
	let (draw_target, render_context) = <(Read<DrawTarget>, Read<RenderContext>)>::fetch(resources);
	let device = render_context.device();

	// Create pipeline
	let vert = ui_vert::Shader::load(device.clone()).context("Couldn't load shader")?;
	let frag = ui_frag::Shader::load(device.clone()).context("Couldn't load shader")?;

	let pipeline = Arc::new(
		GraphicsPipeline::start()
			.render_pass(
				Subpass::from(draw_target.render_pass().clone(), 0)
					.context("Subpass index out of range")?,
			)
			.vertex_input(NumberedInstanceBufferDefinition::<InstanceData>::new(4))
			.vertex_shader(vert.main_entry_point(), ())
			.fragment_shader(frag.main_entry_point(), ())
			.triangle_fan()
			.viewports_dynamic_scissors_irrelevant(1)
			.build(device.clone())
			.context("Couldn't create pipeline")?,
	) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

	let layout = pipeline.descriptor_set_layout(0).unwrap();
	let mut matrix_set_pool = FixedSizeDescriptorSetsPool::new(layout.clone());
	let instance_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer());
	let matrix_uniform_pool = CpuBufferPool::new(
		render_context.device().clone(),
		BufferUsage::uniform_buffer(),
	);
	let mut texture_set_pool =
		FixedSizeDescriptorSetsPool::new(pipeline.descriptor_set_layout(1).unwrap().clone());

	Ok(SystemBuilder::new("draw_ui")
		.read_resource::<AssetStorage>()
		.read_resource::<Arc<Sampler>>()
		.write_resource::<Option<DrawContext>>()
		.with_query(<(Entity, &UiTransform)>::query())
		.with_query(<(&UiImage, &UiTransform)>::query())
		.build(move |_command_buffer, world, resources, queries| {
			(|| -> anyhow::Result<()> {
				let (asset_storage, sampler, draw_context) = resources;
				let draw_context = draw_context.as_mut().unwrap();

				let ui_params = UiParams::new(&draw_context.framebuffer);
				let viewport = &mut draw_context.dynamic_state.viewports.as_mut().unwrap()[0];
				viewport.origin = [0.0, 0.0];
				viewport.dimensions = ui_params.framebuffer_dimensions.into();

				let proj = ortho_matrix(AABB3::from_intervals(Vector3::new(
					Interval::new(0.0, ui_params.dimensions[0]),
					Interval::new(0.0, ui_params.dimensions[1]),
					Interval::new(1000.0, 0.0),
				)));

				// Create matrix UBO
				draw_context.descriptor_sets.truncate(0);
				draw_context.descriptor_sets.push(Arc::new(
					matrix_set_pool
						.next()
						.add_buffer(
							matrix_uniform_pool
								.next(Matrices { proj: proj.into() })
								.context("Couldn't create buffer")?,
						)
						.context("Couldn't add buffer to descriptor set")?
						.build()
						.context("Couldn't create descriptor set")?,
				));

				// Sort UiTransform entities by depth
				let mut entities: Vec<(f32, Entity)> = queries
					.0
					.iter(world)
					.map(|(&entity, ui_transform)| (ui_transform.depth, entity))
					.collect();
				entities.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

				// Group draws into batches by texture, preserving depth order
				let mut batches: Vec<(AssetHandle<Image>, Vec<InstanceData>)> = Vec::new();

				for (ui_image, ui_transform) in entities
					.into_iter()
					.filter_map(|(_, entity)| queries.1.get(world, entity).ok())
				{
					// Set up instance data
					let image = asset_storage.get(&ui_image.image).unwrap();
					let position = ui_transform.position + ui_params.align(ui_transform.alignment)
						- image.offset;
					let size = ui_transform.size + ui_params.stretch(ui_transform.stretch);

					let instance_data = InstanceData {
						in_position: position.into(),
						in_size: size.into(),
					};

					// Add to batches
					match batches.last_mut() {
						Some((i, id)) if *i == ui_image.image => id.push(instance_data),
						_ => batches.push((ui_image.image.clone(), vec![instance_data])),
					}
				}

				// Draw the batches
				for (image_handle, instance_data) in batches {
					let image = asset_storage.get(&image_handle).unwrap();
					draw_context.descriptor_sets.truncate(1);
					draw_context.descriptor_sets.push(Arc::new(
						texture_set_pool
							.next()
							.add_sampled_image(image.image.clone(), sampler.clone())
							.context("Couldn't add image to descriptor set")?
							.build()
							.context("Couldn't create descriptor set")?,
					));

					let instance_buffer = instance_buffer_pool
						.chunk(instance_data)
						.context("Couldn't create buffer")?;

					draw_context
						.commands
						.draw(
							pipeline.clone(),
							&draw_context.dynamic_state,
							vec![Arc::new(instance_buffer)],
							draw_context.descriptor_sets.clone(),
							(),
						)
						.context("Couldn't issue draw to command buffer")?;
				}

				Ok(())
			})()
			.unwrap_or_else(|e| panic!("{:?}", e));
		}))
}

pub mod ui_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/ui.vert",
	}
}

pub use ui_vert::ty::Matrices;

pub mod ui_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/ui.frag",
	}
}

#[derive(Clone, Debug, Default)]
pub struct InstanceData {
	pub in_position: [f32; 2],
	pub in_size: [f32; 2],
}
impl_vertex!(InstanceData, in_position, in_size);

#[derive(Clone, Copy, Debug)]
pub struct UiParams {
	pub dimensions: Vector2<f32>,
	pub framebuffer_dimensions: Vector2<f32>,
	pub alignment_offsets: [Vector2<f32>; 3],
	pub stretch_offsets: [Vector2<f32>; 2],
}

impl UiParams {
	pub fn new<T: FramebufferAbstract + Send + Sync>(framebuffer: &T) -> UiParams {
		let framebuffer_dimensions =
			Vector2::new(framebuffer.width() as f32, framebuffer.height() as f32);
		let ratio = (framebuffer_dimensions[0] / framebuffer_dimensions[1]) / (4.0 / 3.0);

		// If the current aspect ratio is wider than 4:3, stretch horizontally.
		// If narrower, stretch vertically.
		let base_dimensions = Vector2::new(320.0, 200.0);
		let dimensions = if ratio >= 1.0 {
			Vector2::new(base_dimensions[0] * ratio, base_dimensions[1])
		} else {
			Vector2::new(base_dimensions[0], base_dimensions[1] / ratio)
		};
		let alignment_offsets = [
			Vector2::zeros(),
			(dimensions - base_dimensions) * 0.5,
			dimensions - base_dimensions,
		];
		let stretch_offsets = [Vector2::zeros(), dimensions - base_dimensions];

		UiParams {
			dimensions,
			framebuffer_dimensions,
			alignment_offsets,
			stretch_offsets,
		}
	}

	pub fn align(&self, alignment: [UiAlignment; 2]) -> Vector2<f32> {
		Vector2::new(
			self.alignment_offsets[alignment[0] as usize][0],
			self.alignment_offsets[alignment[1] as usize][1],
		)
	}

	pub fn stretch(&self, stretch: [bool; 2]) -> Vector2<f32> {
		Vector2::new(
			self.stretch_offsets[stretch[0] as usize][0],
			self.stretch_offsets[stretch[1] as usize][1],
		)
	}
}
