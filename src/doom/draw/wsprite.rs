use crate::{
	common::{
		assets::AssetStorage,
		geometry::{ortho_matrix, Interval, AABB3},
		video::{DrawContext, DrawTarget, RenderContext},
	},
	doom::{
		client::Client,
		draw::{
			sprite::SpriteRender,
			ui::{ui_frag, ui_vert, Matrices, Vertex, VERTICES},
		},
		ui::{UiAlignment, UiParams},
	},
};
use anyhow::{bail, Context};
use arrayvec::ArrayVec;
use legion::{systems::ResourceSet, IntoQuery, Read, Resources, World};
use nalgebra::{Vector2, Vector3};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	command_buffer::DynamicState,
	descriptor_set::FixedSizeDescriptorSetsPool,
	image::view::ImageViewAbstract,
	pipeline::{
		vertex::BuffersDefinition, viewport::Viewport, GraphicsPipeline, GraphicsPipelineAbstract,
	},
	render_pass::Subpass,
	sampler::Sampler,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaponSpriteRender {
	pub position: Vector2<f32>,
	pub slots: [Option<SpriteRender>; 2],
}

pub fn draw_weapon_sprites(
	resources: &mut Resources,
) -> anyhow::Result<impl FnMut(&mut DrawContext, &World, &Resources)> {
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
			.vertex_input(BuffersDefinition::new().vertex::<Vertex>())
			.vertex_shader(vert.main_entry_point(), ())
			.fragment_shader(frag.main_entry_point(), ())
			.triangle_list()
			.viewports_dynamic_scissors_irrelevant(1)
			.build(device.clone())
			.context("Couldn't create weapon sprite pipeline")?,
	) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

	let layout = &pipeline.layout().descriptor_set_layouts()[0];
	let mut matrix_set_pool = FixedSizeDescriptorSetsPool::new(layout.clone());
	let vertex_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer());
	let matrix_uniform_pool = CpuBufferPool::new(
		render_context.device().clone(),
		BufferUsage::uniform_buffer(),
	);
	let mut texture_set_pool =
		FixedSizeDescriptorSetsPool::new(pipeline.layout().descriptor_set_layouts()[1].clone());

	let mut query = <&WeaponSpriteRender>::query();

	Ok(
		move |draw_context: &mut DrawContext, world: &World, resources: &Resources| {
			(|| -> anyhow::Result<()> {
				let (asset_storage, client, sampler, ui_params) = <(
					Read<AssetStorage>,
					Read<Client>,
					Read<Arc<Sampler>>,
					Read<UiParams>,
				)>::fetch(resources);

				let dynamic_state = DynamicState {
					viewports: Some(vec![Viewport {
						origin: [0.0; 2],
						dimensions: ui_params.framebuffer_dimensions().into(),
						depth_range: 0.0..1.0,
					}]),
					..DynamicState::none()
				};

				let proj = ortho_matrix(AABB3::from_intervals(Vector3::new(
					Interval::new(0.0, ui_params.framebuffer_dimensions()[0]),
					Interval::new(0.0, ui_params.framebuffer_dimensions()[1]),
					Interval::new(1000.0, 0.0),
				)));
				let framebuffer_ratio = ui_params
					.framebuffer_dimensions()
					.component_div(&ui_params.dimensions());

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

				let client_entity = match client.entity {
					Some(e) => e,
					None => return Ok(()),
				};

				let weapon_sprite_render = match query.get(world, client_entity) {
					Ok(x) => x,
					Err(_) => return Ok(()),
				};

				let mut batches: Vec<(Arc<dyn ImageViewAbstract + Send + Sync>, Vec<Vertex>)> =
					Vec::new();

				for sprite_render in weapon_sprite_render.slots.iter().flatten() {
					let sprite = asset_storage.get(&sprite_render.sprite).unwrap();
					let frame = &sprite.frames()[sprite_render.frame];

					// This frame has no images, nothing to draw
					if frame.is_empty() {
						continue;
					} else if frame.len() > 1 {
						bail!("Player sprite has rotation images");
					}

					let image_handle = &frame[0].handle;
					let image = asset_storage.get(image_handle).unwrap();
					let image_view = &image.image_view;
					let position = weapon_sprite_render.position
						+ ui_params.align([UiAlignment::Middle, UiAlignment::Far])
						- image.offset + Vector2::new(0.0, 16.0);
					// TODO use array::map when it's stable
					let vertices = VERTICES
						.iter()
						.map(|v| Vertex {
							in_position: (v.in_position.component_mul(&image.size()) + position)
								.component_mul(&framebuffer_ratio),
							in_texture_coord: v.in_texture_coord,
						})
						.collect::<ArrayVec<_, 4>>();
					let vertices = [0, 1, 2, 0, 2, 3].iter().map(|&i| vertices[i]);
					match batches.last_mut() {
						Some((i, id)) if i == image_view => id.extend(vertices),
						_ => batches.push((image_view.clone(), vertices.collect())),
					}
				}

				// Draw the batches
				for (image_view, vertices) in batches {
					draw_context.descriptor_sets.truncate(1);
					draw_context.descriptor_sets.push(Arc::new(
						texture_set_pool
							.next()
							.add_sampled_image(image_view, sampler.clone())
							.context("Couldn't add image to descriptor set")?
							.build()
							.context("Couldn't create descriptor set")?,
					));

					let vertex_buffer = vertex_buffer_pool.chunk(vertices)?;

					draw_context
						.commands
						.draw(
							pipeline.clone(),
							&dynamic_state,
							vec![Arc::new(vertex_buffer)],
							draw_context.descriptor_sets.clone(),
							(),
						)
						.context("Couldn't issue draw to command buffer")?;
				}

				Ok(())
			})()
			.unwrap_or_else(|e| panic!("{:?}", e));
		},
	)
}
