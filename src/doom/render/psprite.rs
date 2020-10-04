use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::{ortho_matrix, Interval, AABB3},
		video::{
			definition::NumberedInstanceBufferDefinition, DrawContext, DrawStep, RenderContext,
		},
	},
	doom::{
		client::Client,
		image::Image,
		render::{
			sprite::SpriteRender,
			ui::{ui_frag, ui_vert, InstanceData, Matrices, UiParams},
		},
		ui::UiTransform,
	},
};
use anyhow::{bail, Context};
use legion::{systems::ResourceSet, IntoQuery, Read, Resources, World};
use nalgebra::{Vector2, Vector3};
use std::{iter::once, sync::Arc};
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	descriptor::descriptor_set::FixedSizeDescriptorSetsPool,
	framebuffer::{RenderPassAbstract, Subpass},
	pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
	sampler::Sampler,
};

pub struct DrawPlayerSprites {
	instance_buffer_pool: CpuBufferPool<InstanceData>,
	matrix_uniform_pool: CpuBufferPool<Matrices>,
	matrix_set_pool: FixedSizeDescriptorSetsPool,
	pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	texture_set_pool: FixedSizeDescriptorSetsPool,
}

impl DrawPlayerSprites {
	pub fn new(
		render_context: &RenderContext,
		render_pass: &Arc<dyn RenderPassAbstract + Send + Sync>,
	) -> anyhow::Result<DrawPlayerSprites> {
		let device = render_pass.device();

		// Create pipeline
		let vert = ui_vert::Shader::load(device.clone()).context("Couldn't load shader")?;
		let frag = ui_frag::Shader::load(device.clone()).context("Couldn't load shader")?;

		let pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0).context("Subpass index out of range")?,
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
		let matrix_set_pool = FixedSizeDescriptorSetsPool::new(layout.clone());

		Ok(DrawPlayerSprites {
			instance_buffer_pool: CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer()),
			matrix_uniform_pool: CpuBufferPool::new(
				render_context.device().clone(),
				BufferUsage::uniform_buffer(),
			),
			matrix_set_pool,
			texture_set_pool: FixedSizeDescriptorSetsPool::new(
				pipeline.descriptor_set_layout(1).unwrap().clone(),
			),
			pipeline,
		})
	}
}

impl DrawStep for DrawPlayerSprites {
	fn draw(
		&mut self,
		draw_context: &mut DrawContext,
		world: &World,
		resources: &Resources,
	) -> anyhow::Result<()> {
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
			self.matrix_set_pool
				.next()
				.add_buffer(
					self.matrix_uniform_pool
						.next(Matrices { proj: proj.into() })?,
				)?
				.build()?,
		));

		let (asset_storage, client, sampler) =
			<(Read<AssetStorage>, Read<Client>, Read<Arc<Sampler>>)>::fetch(resources);

		let client_entity = match client.entity {
			Some(e) => e,
			None => return Ok(()),
		};

		let (player_sprite_render, ui_transform) =
			match <(&PlayerSpriteRender, &UiTransform)>::query().get(world, client_entity) {
				Ok(x) => x,
				Err(_) => return Ok(()),
			};

		let mut batches: Vec<(AssetHandle<Image>, InstanceData)> = Vec::new();

		for sprite_render in once(&player_sprite_render.weapon).chain(&player_sprite_render.flash) {
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
			let position = ui_transform.position + ui_params.align(ui_transform.alignment)
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
				self.texture_set_pool
					.next()
					.add_sampled_image(image.image.clone(), sampler.clone())?
					.build()?,
			));

			let instance_buffer = self.instance_buffer_pool.next(instance_data)?;

			draw_context
				.commands
				.draw(
					self.pipeline.clone(),
					&draw_context.dynamic_state,
					vec![Arc::new(instance_buffer)],
					draw_context.descriptor_sets.clone(),
					(),
				)
				.context("Draw error")?;
		}

		Ok(())
	}
}

#[derive(Clone, Debug)]
pub struct PlayerSpriteRender {
	pub weapon: SpriteRender,
	pub flash: Option<SpriteRender>,
}
