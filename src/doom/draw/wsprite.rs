use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::{ortho_matrix, Interval, AABB3},
		video::{
			definition::NumberedInstanceBufferDefinition, DrawContext, DrawTarget, RenderContext,
		},
	},
	doom::{
		client::Client,
		draw::{
			sprite::SpriteRender,
			ui::{ui_frag, ui_vert, InstanceData, Matrices},
		},
		image::Image,
		ui::{UiAlignment, UiParams},
	},
};
use anyhow::{bail, Context};
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Read, Resources, SystemBuilder,
};
use nalgebra::{Vector2, Vector3};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	command_buffer::DynamicState,
	descriptor::descriptor_set::FixedSizeDescriptorSetsPool,
	pipeline::{viewport::Viewport, GraphicsPipeline, GraphicsPipelineAbstract},
	render_pass::Subpass,
	sampler::Sampler,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaponSpriteRender {
	pub position: Vector2<f32>,
	pub slots: [Option<SpriteRender>; 2],
}

pub fn draw_weapon_sprites(resources: &mut Resources) -> anyhow::Result<impl Runnable> {
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

	Ok(SystemBuilder::new("draw_weapon_sprites")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.read_resource::<Arc<Sampler>>()
		.read_resource::<UiParams>()
		.write_resource::<Option<DrawContext>>()
		.with_query(<&WeaponSpriteRender>::query())
		.build(move |_command_buffer, world, resources, query| {
			(|| -> anyhow::Result<()> {
				let (asset_storage, client, sampler, ui_params, draw_context) = resources;
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

				let client_entity = match client.entity {
					Some(e) => e,
					None => return Ok(()),
				};

				let weapon_sprite_render = match query.get(world, client_entity) {
					Ok(x) => x,
					Err(_) => return Ok(()),
				};

				let mut batches: Vec<(AssetHandle<Image>, InstanceData)> = Vec::new();

				for sprite_render in weapon_sprite_render.slots.iter().flatten() {
					// Set up instance data
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
					let position = weapon_sprite_render.position
						+ ui_params.align([UiAlignment::Middle, UiAlignment::Far])
						- image.offset + Vector2::new(0.0, 16.0);

					let instance_data = InstanceData {
						in_position: position.into(),
						in_size: image.size().into(),
					};

					// Add to batches
					batches.push((image_handle.clone(), instance_data));
				}

				// Draw the batches
				for (image_handle, instance_data) in batches {
					let image = asset_storage.get(&image_handle).unwrap();
					draw_context.descriptor_sets.truncate(1);
					draw_context.descriptor_sets.push(Arc::new(
						texture_set_pool
							.next()
							.add_sampled_image(image.image_view.clone(), sampler.clone())
							.context("Couldn't add image to descriptor set")?
							.build()
							.context("Couldn't create descriptor set")?,
					));

					let instance_buffer = instance_buffer_pool.next(instance_data)?;

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
