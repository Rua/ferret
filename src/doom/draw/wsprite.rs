use crate::{
	common::{
		assets::AssetStorage,
		geometry::{ortho_matrix, Interval, AABB3},
		video::{DrawTarget, RenderContext},
	},
	doom::{
		draw::{
			sprite::SpriteRender,
			ui::{ui_frag, ui_vert, Matrices, Vertex, VERTICES},
		},
		game::client::Client,
		ui::{UiAlignment, UiParams},
	},
};
use anyhow::{bail, Context};
use legion::{systems::ResourceSet, IntoQuery, Read, Resources, World};
use nalgebra::{Vector2, Vector3};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool, TypedBufferAccess},
	command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
	descriptor_set::{SingleLayoutDescSetPool, WriteDescriptorSet},
	image::view::ImageViewAbstract,
	pipeline::{
		graphics::{
			input_assembly::{InputAssemblyState, PrimitiveTopology},
			vertex_input::BuffersDefinition,
			viewport::ViewportState,
		},
		GraphicsPipeline, Pipeline, PipelineBindPoint,
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
) -> anyhow::Result<
	impl FnMut(
		&mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
		&World,
		&Resources,
	) -> anyhow::Result<()>,
> {
	let (draw_target, render_context, sampler) =
		<(Read<DrawTarget>, Read<RenderContext>, Read<Arc<Sampler>>)>::fetch(resources);
	let device = render_context.device();

	// Create pipeline
	let vert = ui_vert::load(device.clone()).context("Couldn't load shader")?;
	let frag = ui_frag::load(device.clone()).context("Couldn't load shader")?;

	let pipeline = GraphicsPipeline::start()
		.render_pass(
			Subpass::from(draw_target.render_pass().clone(), 0)
				.context("Subpass index out of range")?,
		)
		.vertex_shader(
			vert.entry_point("main")
				.context("Couldn't find entry point \"main\"")?,
			(),
		)
		.fragment_shader(
			frag.entry_point("main")
				.context("Couldn't find entry point \"main\"")?,
			(),
		)
		.vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
		.input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::TriangleList))
		.viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
		.with_auto_layout(device.clone(), |set_descs| {
			set_descs[1].set_immutable_samplers(0, [sampler.clone()]);
		})
		.context("Couldn't create weapon sprite pipeline")?;

	let layout = &pipeline.layout().descriptor_set_layouts()[0];
	let mut matrix_set_pool = SingleLayoutDescSetPool::new(layout.clone());
	let vertex_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer());
	let matrix_uniform_pool = CpuBufferPool::new(
		render_context.device().clone(),
		BufferUsage::uniform_buffer(),
	);
	let mut texture_set_pool =
		SingleLayoutDescSetPool::new(pipeline.layout().descriptor_set_layouts()[1].clone());

	let mut query = <&WeaponSpriteRender>::query();

	Ok(
		move |command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
		      world: &World,
		      resources: &Resources|
		      -> anyhow::Result<()> {
			let (asset_storage, client, ui_params) =
				<(Read<AssetStorage>, Read<Client>, Read<UiParams>)>::fetch(resources);

			command_buffer.bind_pipeline_graphics(pipeline.clone());
			let viewport = command_buffer.state().viewport(0).unwrap();

			// TODO make this work with nonstandard viewport sizes
			let proj = ortho_matrix(AABB3::from_intervals(Vector3::new(
				Interval::new(0.0, viewport.dimensions[0]),
				Interval::new(0.0, viewport.dimensions[1]),
				Interval::new(1000.0, 0.0),
			)));
			let framebuffer_ratio = ui_params
				.framebuffer_dimensions()
				.component_div(&ui_params.dimensions());

			// Create matrix uniform buffer
			let uniform_buffer = matrix_uniform_pool
				.next(Matrices { proj: proj.into() })
				.context("Couldn't create buffer")?;
			let descriptor_set = matrix_set_pool
				.next([WriteDescriptorSet::buffer(0, uniform_buffer)])
				.context("Couldn't create descriptor set")?;
			command_buffer.bind_descriptor_sets(
				PipelineBindPoint::Graphics,
				pipeline.layout().clone(),
				0,
				descriptor_set,
			);

			let client_entity = match client.entity {
				Some(e) => e,
				None => return Ok(()),
			};

			let weapon_sprite_render = match query.get(world, client_entity) {
				Ok(x) => x,
				Err(_) => return Ok(()),
			};

			let mut batches: Vec<(Arc<dyn ImageViewAbstract>, Vec<Vertex>)> = Vec::new();

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
				let vertices = VERTICES.map(|v| Vertex {
					in_position: (v.in_position.component_mul(&image.size()) + position)
						.component_mul(&framebuffer_ratio),
					in_texture_coord: v.in_texture_coord,
				});
				let vertices = [0, 1, 2, 0, 2, 3].into_iter().map(|i| vertices[i]);
				match batches.last_mut() {
					Some((i, id)) if i == image_view => id.extend(vertices),
					_ => batches.push((image_view.clone(), vertices.collect())),
				}
			}

			// Draw the batches
			for (image_view, vertices) in batches {
				let descriptor_set = texture_set_pool
					.next([WriteDescriptorSet::image_view(0, image_view)])
					.context("Couldn't create descriptor set")?;
				command_buffer.bind_descriptor_sets(
					PipelineBindPoint::Graphics,
					pipeline.layout().clone(),
					1,
					descriptor_set,
				);

				let vertex_buffer = vertex_buffer_pool.chunk(vertices)?;
				let vertex_count = vertex_buffer.len() as u32;
				command_buffer.bind_vertex_buffers(0, vertex_buffer);

				command_buffer
					.draw(vertex_count, 1, 0, 0)
					.context("Couldn't issue draw to command buffer")?;
			}

			Ok(())
		},
	)
}
