use crate::{
	assets::{AssetHandle, AssetStorage},
	doom::{
		client::Client,
		components::Transform,
		map::{Map, MapDynamic},
		render::normal_frag,
		sprite::{Sprite, SpriteImage},
	},
	geometry::Angle,
	renderer::{AsBytes, RenderContext},
};
use anyhow::Context;
use fnv::FnvHashMap;
use nalgebra::{Matrix4, Vector2, Vector3};
use specs::{Component, DenseVecStorage, Entities, Join, ReadExpect, ReadStorage, World};
use specs_derive::Component;
use std::{collections::hash_map::Entry, sync::Arc};
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool, ImmutableBuffer},
	command_buffer::{
		pool::standard::StandardCommandPoolBuilder, AutoCommandBufferBuilder, DynamicState,
	},
	descriptor::{
		descriptor_set::{DescriptorSet, FixedSizeDescriptorSetsPool},
		PipelineLayoutAbstract,
	},
	device::DeviceOwned,
	framebuffer::{RenderPassAbstract, Subpass},
	image::ImageViewAccess,
	impl_vertex,
	pipeline::{
		vertex::OneVertexOneInstanceDefinition, GraphicsPipeline, GraphicsPipelineAbstract,
	},
	sampler::Sampler,
};

pub struct SpriteRenderSystem {
	instance_buffer_pool: CpuBufferPool<InstanceData>,
	vertex_buffer: Arc<ImmutableBuffer<[u8]>>,
	pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	texture_set_pool: FixedSizeDescriptorSetsPool,
}

impl SpriteRenderSystem {
	pub fn new(
		render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
		render_context: &RenderContext,
	) -> anyhow::Result<SpriteRenderSystem> {
		let device = render_pass.device();

		// Create pipeline
		let vert = sprite_vert::Shader::load(device.clone()).context("Couldn't load shader")?;
		let frag = normal_frag::Shader::load(device.clone()).context("Couldn't load shader")?;

		let pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0).context("Subpass index out of range")?,
				)
				.vertex_input(OneVertexOneInstanceDefinition::<VertexData, InstanceData>::new())
				.vertex_shader(vert.main_entry_point(), ())
				.fragment_shader(frag.main_entry_point(), ())
				.triangle_fan()
				.primitive_restart(true)
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_disabled()
				.depth_stencil_simple_depth()
				.build(device.clone())
				.context("Couldn't create pipeline")?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		// Create mesh
		let (vertex_buffer, _future) = ImmutableBuffer::from_iter(
			vec![
				VertexData {
					in_position: [0.0, -1.0, 0.0],
					in_texture_coord: [1.0, 0.0],
				},
				VertexData {
					in_position: [0.0, 0.0, 0.0],
					in_texture_coord: [0.0, 0.0],
				},
				VertexData {
					in_position: [0.0, 0.0, -1.0],
					in_texture_coord: [0.0, 1.0],
				},
				VertexData {
					in_position: [0.0, -1.0, -1.0],
					in_texture_coord: [1.0, 1.0],
				},
			]
			.as_bytes()
			.iter()
			.copied(),
			BufferUsage::vertex_buffer(),
			render_context.queues().graphics.clone(),
		)?;

		Ok(SpriteRenderSystem {
			vertex_buffer,

			instance_buffer_pool: CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer()),
			texture_set_pool: FixedSizeDescriptorSetsPool::new(
				pipeline.descriptor_set_layout(1).unwrap().clone(),
			),
			pipeline,
		})
	}

	pub fn draw(
		&mut self,
		world: &World,
		mut command_buffer_builder: AutoCommandBufferBuilder<StandardCommandPoolBuilder>,
		dynamic_state: DynamicState,
		sampler: Arc<Sampler>,
		matrix_set: Arc<dyn DescriptorSet + Send + Sync>,
		yaw: Angle,
		view_pos: Vector3<f32>,
	) -> anyhow::Result<AutoCommandBufferBuilder> {
		let (
			entities,
			client,
			map_storage,
			sprite_storage,
			sprite_image_storage,
			map_dynamic_component,
			sprite_component,
			transform_component,
		) = world.system_data::<(
			Entities,
			ReadExpect<Client>,
			ReadExpect<AssetStorage<Map>>,
			ReadExpect<AssetStorage<Sprite>>,
			ReadExpect<AssetStorage<SpriteImage>>,
			ReadStorage<MapDynamic>,
			ReadStorage<SpriteRender>,
			ReadStorage<Transform>,
		)>();

		let map_dynamic = map_dynamic_component.join().next().unwrap();
		let map = map_storage.get(&map_dynamic.map).unwrap();

		// Group draws into batches by texture
		let mut batches: FnvHashMap<Arc<dyn ImageViewAccess + Send + Sync>, Vec<InstanceData>> =
			FnvHashMap::default();

		for (entity, sprite_render, transform) in
			(&entities, &sprite_component, &transform_component).join()
		{
			// Don't render the player's own sprite
			if let Some(view_entity) = client.entity {
				if entity == view_entity {
					continue;
				}
			}

			let sprite = sprite_storage.get(&sprite_render.sprite).unwrap();
			let frame = &sprite.frames()[sprite_render.frame];

			// This frame has no images, nothing to render
			if frame.is_empty() {
				continue;
			}

			// Figure out which rotation image to use
			// Treat non-rotating frames specially for efficiency
			let index = if frame.len() == 1 {
				0
			} else {
				let to_view_vec = view_pos - transform.position;
				let to_view_angle =
					Angle::from_radians(f64::atan2(to_view_vec[1] as f64, to_view_vec[0] as f64));
				let delta = to_view_angle - transform.rotation[2]
					+ Angle::from_units(0.5 / frame.len() as f64);
				(delta.to_units_unsigned() * frame.len() as f64) as usize % frame.len()
			};

			let image_info = &frame[index];
			let sprite_image = sprite_image_storage.get(&image_info.handle).unwrap();

			// Determine light level
			let light_level = if sprite_render.full_bright {
				1.0
			} else {
				let ssect =
					map.find_subsector(Vector2::new(transform.position[0], transform.position[1]));
				map_dynamic.sectors[ssect.sector_index].light_level
			};

			// Set up instance data
			let instance_matrix = Matrix4::new_translation(&transform.position)
				* Matrix4::new_rotation(Vector3::new(0.0, 0.0, yaw.to_radians() as f32))
				* sprite_image.matrix;
			let instance_data = InstanceData {
				in_flip: image_info.flip,
				in_light_level: light_level,
				in_matrix: instance_matrix.into(),
			};

			// Add to batches
			match batches.entry(
				sprite_image_storage
					.get(&image_info.handle)
					.unwrap()
					.image
					.clone(),
			) {
				Entry::Occupied(mut entry) => {
					entry.get_mut().push(instance_data);
				}
				Entry::Vacant(entry) => {
					entry.insert(vec![instance_data]);
				}
			}
		}

		// Draw the batches
		for (texture, instance_data) in batches {
			let texture_set = Arc::new(
				self.texture_set_pool
					.next()
					.add_sampled_image(texture, sampler.clone())?
					.build()?,
			);

			let instance_buffer = self.instance_buffer_pool.chunk(instance_data)?;

			command_buffer_builder = command_buffer_builder
				.draw(
					self.pipeline.clone(),
					&dynamic_state,
					vec![self.vertex_buffer.clone(), Arc::new(instance_buffer)],
					(matrix_set.clone(), texture_set.clone()),
					(),
				)
				.context("Draw error")?;
		}

		Ok(command_buffer_builder)
	}
}

mod sprite_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/sprite.vert",
	}
}

#[derive(Clone, Debug, Default)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 2],
}
impl_vertex!(VertexData, in_position, in_texture_coord);

#[derive(Clone, Debug, Default)]
pub struct InstanceData {
	pub in_flip: f32,
	pub in_light_level: f32,
	pub in_matrix: [[f32; 4]; 4],
}
impl_vertex!(InstanceData, in_flip, in_light_level, in_matrix);

#[derive(Clone, Component, Debug)]
pub struct SpriteRender {
	pub sprite: AssetHandle<Sprite>,
	pub frame: usize,
	pub full_bright: bool,
}
