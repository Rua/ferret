use crate::{
	common::{
		assets::AssetStorage,
		video::{AsBytes, DrawContext, DrawStep},
	},
	doom::{
		client::Client,
		components::Transform,
		map::{
			meshes::{SkyVertexData, VertexData},
			MapDynamic,
		},
		render::world::normal_frag,
	},
};
use anyhow::{anyhow, Context};
use legion::prelude::{EntityStore, IntoQuery, Read, ResourceSet, Resources, World};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	descriptor::{descriptor_set::FixedSizeDescriptorSetsPool, PipelineLayoutAbstract},
	device::DeviceOwned,
	framebuffer::{RenderPassAbstract, Subpass},
	pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
	sampler::Sampler,
};

pub struct DrawMap {
	index_buffer_pool: CpuBufferPool<u32>,
	normal_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	normal_texture_set_pool: FixedSizeDescriptorSetsPool,
	sky_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	sky_texture_set_pool: FixedSizeDescriptorSetsPool,
	sky_uniform_pool: CpuBufferPool<sky_frag::ty::FragParams>,
	vertex_buffer_pool: CpuBufferPool<u8>,
}

impl DrawMap {
	pub fn new(render_pass: &Arc<dyn RenderPassAbstract + Send + Sync>) -> anyhow::Result<DrawMap> {
		let device = render_pass.device();

		// Create pipeline for normal parts of the map
		let normal_vert =
			normal_vert::Shader::load(device.clone()).context("Couldn't load shader")?;
		let normal_frag =
			normal_frag::Shader::load(device.clone()).context("Couldn't load shader")?;

		let normal_pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0)
						.ok_or(anyhow!("Subpass index out of range"))?,
				)
				.vertex_input_single_buffer::<VertexData>()
				.vertex_shader(normal_vert.main_entry_point(), ())
				.fragment_shader(normal_frag.main_entry_point(), ())
				.triangle_fan()
				.primitive_restart(true)
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_back()
				.depth_stencil_simple_depth()
				.build(device.clone())
				.context("Couldn't create pipeline")?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		// Create pipeline for sky
		let sky_vert = sky_vert::Shader::load(device.clone())?;
		let sky_frag = sky_frag::Shader::load(device.clone())?;

		let sky_pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0).context("Subpass index out of range")?,
				)
				.vertex_input_single_buffer::<SkyVertexData>()
				.vertex_shader(sky_vert.main_entry_point(), ())
				.fragment_shader(sky_frag.main_entry_point(), ())
				.triangle_fan()
				.primitive_restart(true)
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_back()
				.depth_stencil_simple_depth()
				.build(device.clone())
				.context("Couldn't create pipeline")?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		Ok(DrawMap {
			index_buffer_pool: CpuBufferPool::new(device.clone(), BufferUsage::index_buffer()),
			vertex_buffer_pool: CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer()),

			normal_texture_set_pool: FixedSizeDescriptorSetsPool::new(
				normal_pipeline.descriptor_set_layout(1).unwrap().clone(),
			),
			normal_pipeline,

			sky_uniform_pool: CpuBufferPool::new(device.clone(), BufferUsage::uniform_buffer()),
			sky_texture_set_pool: FixedSizeDescriptorSetsPool::new(
				sky_pipeline.descriptor_set_layout(1).unwrap().clone(),
			),
			sky_pipeline,
		})
	}
}

impl DrawStep for DrawMap {
	fn draw(
		&mut self,
		draw_context: &mut DrawContext,
		world: &World,
		resources: &Resources,
	) -> anyhow::Result<()> {
		let (asset_storage, client, sampler) =
			<(Read<AssetStorage>, Read<Client>, Read<Arc<Sampler>>)>::fetch(resources);
		let camera_entity = client.entity.unwrap();
		let camera_transform = world.get_component::<Transform>(camera_entity).unwrap();

		for map_dynamic in <Read<MapDynamic>>::query().iter(world) {
			let map = asset_storage.get(&map_dynamic.map).unwrap();
			let (flat_meshes, sky_mesh, wall_meshes) =
				crate::doom::map::meshes::make_meshes(map, map_dynamic.as_ref(), resources)
					.context("Couldn't generate map mesh")?;

			// Draw the walls
			for (handle, mesh) in wall_meshes {
				let vertex_buffer = self
					.vertex_buffer_pool
					.chunk(mesh.0.as_bytes().iter().copied())?;
				let index_buffer = self.index_buffer_pool.chunk(mesh.1)?;

				// Redirect animation frames
				let handle = if let Some(anim_state) = map_dynamic.anim_states_wall.get(&handle) {
					let anim = &map.anims_wall[&handle];
					&anim.frames[anim_state.frame]
				} else {
					&handle
				};
				let image = asset_storage.get(&handle).unwrap();

				draw_context.descriptor_sets.truncate(1);
				draw_context.descriptor_sets.push(Arc::new(
					self.normal_texture_set_pool
						.next()
						.add_sampled_image(image.clone(), sampler.clone())?
						.build()?,
				));

				draw_context.commands.draw_indexed(
					self.normal_pipeline.clone(),
					&draw_context.dynamic_state,
					vec![Arc::new(vertex_buffer)],
					index_buffer,
					draw_context.descriptor_sets.clone(),
					(),
				)?;
			}

			// Draw the flats
			for (handle, mesh) in flat_meshes {
				let vertex_buffer = self
					.vertex_buffer_pool
					.chunk(mesh.0.as_bytes().iter().copied())?;
				let index_buffer = self.index_buffer_pool.chunk(mesh.1)?;

				// Redirect animation frames
				let handle = if let Some(anim_state) = map_dynamic.anim_states_flat.get(&handle) {
					let anim = &map.anims_flat[&handle];
					&anim.frames[anim_state.frame]
				} else {
					&handle
				};
				let image = asset_storage.get(handle).unwrap();

				draw_context.descriptor_sets.truncate(1);
				draw_context.descriptor_sets.push(Arc::new(
					self.normal_texture_set_pool
						.next()
						.add_sampled_image(image.clone(), sampler.clone())?
						.build()?,
				));

				draw_context
					.commands
					.draw_indexed(
						self.normal_pipeline.clone(),
						&draw_context.dynamic_state,
						vec![Arc::new(vertex_buffer)],
						index_buffer,
						draw_context.descriptor_sets.clone(),
						(),
					)
					.context("Draw error")?;
			}

			// Draw the sky
			let vertex_buffer = self
				.vertex_buffer_pool
				.chunk(sky_mesh.0.as_bytes().iter().copied())?;
			let index_buffer = self.index_buffer_pool.chunk(sky_mesh.1)?;
			let image = asset_storage.get(&map.sky).unwrap();
			let sky_buffer = self.sky_uniform_pool.next(sky_frag::ty::FragParams {
				screenSize: [800.0, 600.0],
				pitch: camera_transform.rotation[1].to_degrees() as f32,
				yaw: camera_transform.rotation[2].to_degrees() as f32,
			})?;

			draw_context.descriptor_sets.truncate(1);
			draw_context.descriptor_sets.push(Arc::new(
				self.sky_texture_set_pool
					.next()
					.add_sampled_image(image.clone(), sampler.clone())?
					.add_buffer(sky_buffer)?
					.build()?,
			));

			draw_context
				.commands
				.draw_indexed(
					self.sky_pipeline.clone(),
					&draw_context.dynamic_state,
					vec![Arc::new(vertex_buffer)],
					index_buffer,
					draw_context.descriptor_sets.clone(),
					(),
				)
				.context("Draw error")?;
		}

		Ok(())
	}
}

mod normal_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/map_normal.vert",
	}
}

pub use normal_vert::ty::Matrices;

mod sky_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/map_sky.vert",
	}
}

mod sky_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/sky.frag",
	}
}
