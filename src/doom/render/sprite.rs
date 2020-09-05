use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::Angle,
		video::{
			definition::NumberedInstanceBufferDefinition, DrawContext, DrawStep, RenderContext,
		},
	},
	doom::{
		client::Client, components::Transform, image::Image, map::MapDynamic,
		render::world::normal_frag, sprite::Sprite,
	},
};
use anyhow::Context;
use fnv::FnvHashMap;
use legion::prelude::{EntityStore, IntoQuery, Read, ResourceSet, Resources, World};
use nalgebra::{Matrix4, Vector2, Vector3};
use std::{collections::hash_map::Entry, sync::Arc};
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	descriptor::{descriptor_set::FixedSizeDescriptorSetsPool, PipelineLayoutAbstract},
	device::DeviceOwned,
	framebuffer::{RenderPassAbstract, Subpass},
	impl_vertex,
	pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
	sampler::Sampler,
};

pub struct DrawSprites {
	instance_buffer_pool: CpuBufferPool<InstanceData>,
	pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	texture_set_pool: FixedSizeDescriptorSetsPool,
	texture_uniform_pool: CpuBufferPool<ImageMatrix>,
}

impl DrawSprites {
	pub fn new(
		_render_context: &RenderContext,
		render_pass: &Arc<dyn RenderPassAbstract + Send + Sync>,
	) -> anyhow::Result<DrawSprites> {
		let device = render_pass.device();

		// Create pipeline
		let vert = sprite_vert::Shader::load(device.clone()).context("Couldn't load shader")?;
		let frag = normal_frag::Shader::load(device.clone()).context("Couldn't load shader")?;

		let pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0).context("Subpass index out of range")?,
				)
				.vertex_input(NumberedInstanceBufferDefinition::<InstanceData>::new(4))
				.vertex_shader(vert.main_entry_point(), ())
				.fragment_shader(frag.main_entry_point(), ())
				.triangle_fan()
				.primitive_restart(true)
				.viewports_dynamic_scissors_irrelevant(1)
				.depth_stencil_simple_depth()
				.build(device.clone())
				.context("Couldn't create pipeline")?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		Ok(DrawSprites {
			instance_buffer_pool: CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer()),
			texture_set_pool: FixedSizeDescriptorSetsPool::new(
				pipeline.descriptor_set_layout(1).unwrap().clone(),
			),
			texture_uniform_pool: CpuBufferPool::new(device.clone(), BufferUsage::uniform_buffer()),
			pipeline,
		})
	}
}

impl DrawStep for DrawSprites {
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

		let map_dynamic = <Read<MapDynamic>>::query().iter(world).next().unwrap();
		let map = asset_storage.get(&map_dynamic.map).unwrap();

		// Group draws into batches by texture
		let mut batches: FnvHashMap<&AssetHandle<Image>, Vec<InstanceData>> = FnvHashMap::default();

		for (entity, (sprite_render, transform)) in
			<(Read<SpriteRender>, Read<Transform>)>::query().iter_entities(world)
		{
			// Don't render the player's own sprite
			if let Some(view_entity) = client.entity {
				if entity == view_entity {
					continue;
				}
			}

			let sprite = asset_storage.get(&sprite_render.sprite).unwrap();
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
				let to_view_vec = camera_transform.position - transform.position;
				let to_view_angle =
					Angle::from_radians(f64::atan2(to_view_vec[1] as f64, to_view_vec[0] as f64));
				let delta = to_view_angle - transform.rotation[2]
					+ Angle::from_units(0.5 / frame.len() as f64);
				(delta.to_units_unsigned() * frame.len() as f64) as usize % frame.len()
			};

			let image_info = &frame[index];

			// Determine light level
			let light_level = if sprite_render.full_bright {
				1.0
			} else {
				let ssect =
					map.find_subsector(Vector2::new(transform.position[0], transform.position[1]));
				map_dynamic.sectors[ssect.sector_index].light_level
			};

			// Set up instance data
			let instance_data = InstanceData {
				in_transform: Matrix4::new_translation(&transform.position).into(),
				in_flip: image_info.flip,
				in_light_level: light_level,
			};

			// Add to batches
			match batches.entry(&image_info.handle) {
				Entry::Occupied(mut entry) => {
					entry.get_mut().push(instance_data);
				}
				Entry::Vacant(entry) => {
					entry.insert(vec![instance_data]);
				}
			}
		}

		// Draw the batches
		for (image_handle, instance_data) in batches {
			let image = asset_storage.get(image_handle).unwrap();
			let matrix = Matrix4::new_translation(&Vector3::new(
				-image.offset[0] as f32,
				-image.offset[1] as f32,
				0.0,
			)) * Matrix4::new_nonuniform_scaling(&Vector3::new(
				image.image.dimensions().width() as f32,
				image.image.dimensions().height() as f32,
				1.0,
			));

			draw_context.descriptor_sets.truncate(1);
			draw_context.descriptor_sets.push(Arc::new(
				self.texture_set_pool
					.next()
					.add_sampled_image(image.image.clone(), sampler.clone())?
					.add_buffer(self.texture_uniform_pool.next(ImageMatrix {
						image_matrix: matrix.into(),
					})?)?
					.build()?,
			));

			let instance_buffer = self.instance_buffer_pool.chunk(instance_data)?;

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

mod sprite_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/sprite.vert",
	}
}

use sprite_vert::ty::ImageMatrix;

#[derive(Clone, Debug, Default)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 2],
}
impl_vertex!(VertexData, in_position, in_texture_coord);

#[derive(Clone, Debug, Default)]
pub struct InstanceData {
	pub in_transform: [[f32; 4]; 4],
	pub in_flip: f32,
	pub in_light_level: f32,
}
impl_vertex!(InstanceData, in_transform, in_flip, in_light_level);

#[derive(Clone, Debug)]
pub struct SpriteRender {
	pub sprite: AssetHandle<Sprite>,
	pub frame: usize,
	pub full_bright: bool,
}
