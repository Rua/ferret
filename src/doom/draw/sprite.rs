use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::Angle,
		video::{
			definition::NumberedInstanceBufferDefinition, DrawContext, DrawTarget, RenderContext,
		},
	},
	doom::{
		client::Client, components::Transform, draw::world::normal_frag, image::Image,
		map::MapDynamic, sprite::Sprite,
	},
};
use anyhow::Context;
use fnv::FnvHashMap;
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, SystemBuilder,
};
use nalgebra::{Matrix4, Vector2};
use std::{collections::hash_map::Entry, sync::Arc};
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	descriptor::{descriptor_set::FixedSizeDescriptorSetsPool, PipelineLayoutAbstract},
	framebuffer::Subpass,
	impl_vertex,
	pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
	sampler::Sampler,
};

#[derive(Clone, Debug)]
pub struct SpriteRender {
	pub sprite: AssetHandle<Sprite>,
	pub frame: usize,
	pub full_bright: bool,
}

pub fn draw_sprites(resources: &mut Resources) -> anyhow::Result<impl Runnable> {
	let (draw_target, render_context) = <(Read<DrawTarget>, Read<RenderContext>)>::fetch(resources);
	let device = render_context.device();

	// Create pipeline
	let vert = sprite_vert::Shader::load(device.clone()).context("Couldn't load shader")?;
	let frag = normal_frag::Shader::load(device.clone()).context("Couldn't load shader")?;

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
			.primitive_restart(true)
			.viewports_dynamic_scissors_irrelevant(1)
			.depth_stencil_simple_depth()
			.build(device.clone())
			.context("Couldn't create pipeline")?,
	) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

	let instance_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer());
	let mut texture_set_pool =
		FixedSizeDescriptorSetsPool::new(pipeline.descriptor_set_layout(1).unwrap().clone());
	let texture_uniform_pool = CpuBufferPool::new(device.clone(), BufferUsage::uniform_buffer());

	Ok(SystemBuilder::new("draw_sprites")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.read_resource::<Arc<Sampler>>()
		.write_resource::<Option<DrawContext>>()
		.with_query(<&Transform>::query())
		.with_query(<&MapDynamic>::query())
		.with_query(<(Entity, &SpriteRender, &Transform)>::query())
		.build(move |_command_buffer, world, resources, queries| {
			(|| -> anyhow::Result<()> {
				let (asset_storage, client, sampler, draw_context) = resources;
				let draw_context = draw_context.as_mut().unwrap();

				let camera_transform = queries.0.get(world, client.entity.unwrap()).unwrap();
				let map_dynamic = queries.1.iter(world).next().unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();

				// Group draws into batches by texture
				let mut batches: FnvHashMap<&AssetHandle<Image>, Vec<InstanceData>> =
					FnvHashMap::default();

				for (&entity, sprite_render, transform) in queries.2.iter(world) {
					// Don't draw the player's own sprite
					if let Some(view_entity) = client.entity {
						if entity == view_entity {
							continue;
						}
					}

					let sprite = asset_storage.get(&sprite_render.sprite).unwrap();
					let frame = &sprite.frames()[sprite_render.frame];

					// This frame has no images, nothing to draw
					if frame.is_empty() {
						continue;
					}

					// Figure out which rotation image to use
					// Treat non-rotating frames specially for efficiency
					let index = if frame.len() == 1 {
						0
					} else {
						let to_view_vec = camera_transform.position - transform.position;
						let to_view_angle = Angle::from_radians(f64::atan2(
							to_view_vec[1] as f64,
							to_view_vec[0] as f64,
						));
						let delta = to_view_angle - transform.rotation[2]
							+ Angle::from_units(0.5 / frame.len() as f64);
						(delta.to_units_unsigned() * frame.len() as f64) as usize % frame.len()
					};

					let image_info = &frame[index];

					// Determine light level
					let light_level = if sprite_render.full_bright {
						1.0
					} else {
						let ssect = map.find_subsector(Vector2::new(
							transform.position[0],
							transform.position[1],
						));
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
					let matrix = Matrix4::new_translation(&-image.offset.fixed_resize(0.0))
						* Matrix4::new_nonuniform_scaling(&image.size().fixed_resize(1.0));

					draw_context.descriptor_sets.truncate(1);
					draw_context.descriptor_sets.push(Arc::new(
						texture_set_pool
							.next()
							.add_sampled_image(image.image.clone(), sampler.clone())
							.context("Couldn't add image to descriptor set")?
							.add_buffer(
								texture_uniform_pool
									.next(ImageMatrix {
										image_matrix: matrix.into(),
									})
									.context("Couldn't create buffer")?,
							)
							.context("Couldn't add buffer to descriptor set")?
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