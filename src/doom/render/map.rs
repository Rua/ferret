use crate::{
	assets::AssetStorage,
	doom::{
		map::{
			meshes::{SkyVertexData, VertexData},
			textures::{Flat, Wall},
			Map, MapDynamic,
		},
		render::normal_frag,
	},
	geometry::Angle,
	renderer::AsBytes,
};
use anyhow::{anyhow, Context};
use nalgebra::Vector3;
use specs::{Join, ReadExpect, ReadStorage, World};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	command_buffer::{
		pool::standard::StandardCommandPoolBuilder, AutoCommandBufferBuilder, DynamicState,
	},
	descriptor::{
		descriptor_set::{DescriptorSet, FixedSizeDescriptorSetsPool},
		PipelineLayoutAbstract,
	},
	device::DeviceOwned,
	framebuffer::{RenderPassAbstract, Subpass},
	pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
	sampler::Sampler,
};

mod normal_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/map_normal.vert",
	}
}

pub use normal_vert::ty::UniformBufferObject;

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

pub struct MapRenderSystem {
	index_buffer_pool: CpuBufferPool<u32>,
	normal_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	normal_texture_set_pool: FixedSizeDescriptorSetsPool,
	sky_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	sky_texture_set_pool: FixedSizeDescriptorSetsPool,
	sky_uniform_pool: CpuBufferPool<sky_frag::ty::FragParams>,
	vertex_buffer_pool: CpuBufferPool<u8>,
}

impl MapRenderSystem {
	pub fn new(
		render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	) -> anyhow::Result<MapRenderSystem> {
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

		Ok(MapRenderSystem {
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

	pub fn draw(
		&mut self,
		world: &World,
		mut command_buffer_builder: AutoCommandBufferBuilder<StandardCommandPoolBuilder>,
		dynamic_state: DynamicState,
		sampler: Arc<Sampler>,
		matrix_set: Arc<dyn DescriptorSet + Send + Sync>,
		rotation: Vector3<Angle>,
	) -> anyhow::Result<AutoCommandBufferBuilder> {
		let (flat_storage, map_storage, wall_storage, map_component) = world.system_data::<(
			ReadExpect<AssetStorage<Flat>>,
			ReadExpect<AssetStorage<Map>>,
			ReadExpect<AssetStorage<Wall>>,
			ReadStorage<MapDynamic>,
		)>();

		for map_dynamic in map_component.join() {
			let map = map_storage.get(&map_dynamic.map).unwrap();
			let (flat_meshes, sky_mesh, wall_meshes) =
				crate::doom::map::meshes::make_meshes(map, map_dynamic, world)
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
				let image = wall_storage.get(&handle).unwrap();

				let texture_set = Arc::new(
					self.normal_texture_set_pool
						.next()
						.add_sampled_image(image.clone(), sampler.clone())?
						.build()?,
				);

				command_buffer_builder = command_buffer_builder.draw_indexed(
					self.normal_pipeline.clone(),
					&dynamic_state,
					vec![Arc::new(vertex_buffer)],
					index_buffer,
					(matrix_set.clone(), texture_set.clone()),
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
				let image = flat_storage.get(handle).unwrap();

				let texture_set = Arc::new(
					self.normal_texture_set_pool
						.next()
						.add_sampled_image(image.clone(), sampler.clone())?
						.build()?,
				);

				command_buffer_builder = command_buffer_builder
					.draw_indexed(
						self.normal_pipeline.clone(),
						&dynamic_state,
						vec![Arc::new(vertex_buffer)],
						index_buffer,
						(matrix_set.clone(), texture_set.clone()),
						(),
					)
					.context("Draw error")?;
			}

			// Draw the sky
			let vertex_buffer = self
				.vertex_buffer_pool
				.chunk(sky_mesh.0.as_bytes().iter().copied())?;
			let index_buffer = self.index_buffer_pool.chunk(sky_mesh.1)?;
			let image = wall_storage.get(&map.sky).unwrap();
			let sky_buffer = self.sky_uniform_pool.next(sky_frag::ty::FragParams {
				screenSize: [800.0, 600.0],
				pitch: rotation[1].to_degrees() as f32,
				yaw: rotation[2].to_degrees() as f32,
			})?;

			let texture_params_set = Arc::new(
				self.sky_texture_set_pool
					.next()
					.add_sampled_image(image.clone(), sampler.clone())?
					.add_buffer(sky_buffer)?
					.build()?,
			);

			command_buffer_builder = command_buffer_builder
				.draw_indexed(
					self.sky_pipeline.clone(),
					&dynamic_state,
					vec![Arc::new(vertex_buffer)],
					index_buffer,
					(matrix_set.clone(), texture_params_set.clone()),
					(),
				)
				.context("Draw error")?;
		}

		Ok(command_buffer_builder)
	}
}
