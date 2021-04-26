use crate::{
	common::{
		assets::AssetStorage,
		geometry::{ortho_matrix, Interval, AABB3},
		video::{
			definition::NumberedInstanceBufferDefinition, DrawContext, DrawTarget, RenderContext,
		},
	},
	doom::ui::{FontSpacing, Hidden, UiHexFontText, UiImage, UiParams, UiText, UiTransform},
};
use anyhow::Context;
use legion::{
	component,
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, SystemBuilder,
};
use nalgebra::Vector3;
use std::{cmp::Ordering, sync::Arc};
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	command_buffer::DynamicState,
	descriptor::descriptor_set::FixedSizeDescriptorSetsPool,
	image::view::ImageViewAbstract,
	impl_vertex,
	pipeline::{viewport::Viewport, GraphicsPipeline, GraphicsPipelineAbstract},
	render_pass::Subpass,
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
		.read_resource::<UiParams>()
		.write_resource::<Option<DrawContext>>()
		.with_query(<(Entity, &UiTransform)>::query().filter(!component::<Hidden>()))
		.with_query(<(
			&UiTransform,
			Option<&UiImage>,
			Option<&UiText>,
			Option<&UiHexFontText>,
		)>::query())
		.build(move |_command_buffer, world, resources, queries| {
			(|| -> anyhow::Result<()> {
				let (asset_storage, sampler, ui_params, draw_context) = resources;
				let draw_context = draw_context.as_mut().unwrap();

				let dynamic_state = DynamicState {
					viewports: Some(vec![Viewport {
						origin: [0.0; 2],
						dimensions: ui_params.framebuffer_dimensions().into(),
						depth_range: 0.0..1.0,
					}]),
					..DynamicState::none()
				};

				let proj = ortho_matrix(AABB3::from_intervals(Vector3::new(
					Interval::new(0.0, ui_params.dimensions()[0]),
					Interval::new(0.0, ui_params.dimensions()[1]),
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
				let mut batches: Vec<(
					Arc<dyn ImageViewAbstract + Send + Sync>,
					Vec<InstanceData>,
				)> = Vec::new();

				for (ui_transform, ui_image, ui_text, ui_hexfont_text) in entities
					.into_iter()
					.filter_map(|(_, entity)| queries.1.get(world, entity).ok())
				{
					let position = ui_transform.position + ui_params.align(ui_transform.alignment);
					let size = ui_transform.size + ui_params.stretch(ui_transform.stretch);

					if let Some(ui_image) = ui_image {
						// Set up instance data
						let image = asset_storage.get(&ui_image.image).unwrap();

						let instance_data = InstanceData {
							in_position: (position - image.offset).into(),
							in_size: size.into(),
							in_texture_offset: [0.0; 2],
						};

						// Add to batches
						let image_view = &image.image_view;
						match batches.last_mut() {
							Some((i, id)) if i == image_view => id.push(instance_data),
							_ => batches.push((image_view.clone(), vec![instance_data])),
						}
					}

					if let Some(ui_text) = ui_text {
						let font = asset_storage.get(&ui_text.font).unwrap();
						let mut cursor_position = position;

						for ch in ui_text.text.chars() {
							if ch == ' ' {
								let width = match font.spacing {
									FontSpacing::FixedWidth { width } => width,
									FontSpacing::VariableWidth { space_width } => space_width,
								};
								cursor_position[0] += width;
							} else if let Some(image_handle) = font.characters.get(&ch) {
								let image = asset_storage.get(image_handle).unwrap();
								let instance_data = InstanceData {
									in_position: (cursor_position - image.offset).into(),
									in_size: image.size().into(),
									in_texture_offset: [0.0; 2],
								};

								let width = match font.spacing {
									FontSpacing::FixedWidth { width } => width,
									FontSpacing::VariableWidth { .. } => image.size()[0],
								};
								cursor_position[0] += width;

								// Add to batches
								let image_view = &image.image_view;
								match batches.last_mut() {
									Some((i, id)) if i == image_view => id.push(instance_data),
									_ => batches.push((image_view.clone(), vec![instance_data])),
								}
							}
						}
					}

					if let Some(ui_text) = ui_hexfont_text {
						let font = asset_storage.get(&ui_text.font).unwrap();
						let mut cursor_position = position;
						let start_of_line = cursor_position[0];

						for line in ui_text.lines.iter().map(|line| line.trim_end()) {
							for (ch_position, ch_size) in
								line.chars().filter_map(|ch| font.locations.get(&ch))
							{
								let instance_data = InstanceData {
									in_position: cursor_position.into(),
									in_size: ch_size.map(|x| x as f32).into(),
									in_texture_offset: ch_position.map(|x| x as f32).into(),
								};

								match batches.last_mut() {
									Some((i, id)) if i == &font.image_view => {
										id.push(instance_data)
									}
									_ => {
										batches.push((font.image_view.clone(), vec![instance_data]))
									}
								}

								cursor_position[0] += ch_size[0] as f32;
							}

							cursor_position[0] = start_of_line;
							cursor_position[1] += font.line_height as f32;

							if cursor_position[1] + font.line_height as f32 > ui_transform.size[1] {
								// No more room for another line
								break;
							}
						}
					}
				}

				// Draw the batches
				for (image_view, instance_data) in batches {
					//let image = asset_storage.get(&image_handle).unwrap();
					draw_context.descriptor_sets.truncate(1);
					draw_context.descriptor_sets.push(Arc::new(
						texture_set_pool
							.next()
							.add_sampled_image(image_view, sampler.clone())
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
							&dynamic_state,
							vec![Arc::new(instance_buffer)],
							draw_context.descriptor_sets.clone(),
							(),
							std::iter::empty(),
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
	pub in_texture_offset: [f32; 2],
}
impl_vertex!(InstanceData, in_position, in_size, in_texture_offset);
