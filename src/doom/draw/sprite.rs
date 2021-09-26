use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::Angle,
		video::{DrawTarget, RenderContext},
	},
	doom::{
		assets::sprite::Sprite,
		draw::world::{world_frag, world_vert},
		game::{camera::Camera, client::Client, map::MapDynamic, Transform},
	},
};
use anyhow::Context;
use fnv::FnvHashMap;
use legion::{systems::ResourceSet, Entity, IntoQuery, Read, Resources, World};
use memoffset::offset_of;
use nalgebra::{Matrix4, Vector2, Vector3};
use serde::{Deserialize, Serialize};
use std::{collections::hash_map::Entry, sync::Arc};
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool, TypedBufferAccess},
	command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
	descriptor_set::SingleLayoutDescSetPool,
	image::view::ImageViewAbstract,
	pipeline::{
		vertex::{BuffersDefinition, Vertex as VertexTrait, VertexMemberInfo, VertexMemberTy},
		GraphicsPipeline, PipelineBindPoint,
	},
	render_pass::Subpass,
	sampler::Sampler,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteRender {
	pub sprite: AssetHandle<Sprite>,
	pub frame: usize,
	pub full_bright: bool,
}

pub fn draw_sprites(
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
	let vert = world_vert::Shader::load(device.clone()).context("Couldn't load shader")?;
	let frag = world_frag::Shader::load(device.clone()).context("Couldn't load shader")?;

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
			.depth_stencil_simple_depth()
			.with_auto_layout(device.clone(), |set_descs| {
				set_descs[1].set_immutable_samplers(0, [sampler.clone()]);
			})
			.context("Couldn't create sprite pipeline")?,
	);

	let vertex_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer());
	let mut texture_set_pool =
		SingleLayoutDescSetPool::new(pipeline.layout().descriptor_set_layouts()[1].clone());

	let mut queries = (
		<(Option<&Camera>, &Transform)>::query(),
		<&MapDynamic>::query(),
		<(Entity, &SpriteRender, &Transform)>::query(),
	);

	Ok(
		move |command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
		      world: &World,
		      resources: &Resources|
		      -> anyhow::Result<()> {
			let (asset_storage, client) = <(Read<AssetStorage>, Read<Client>)>::fetch(resources);
			let map_dynamic = queries.1.iter(world).next().unwrap();
			let map = asset_storage.get(&map_dynamic.map).unwrap();

			// Camera
			let (camera, &(mut camera_transform)) =
				queries.0.get(world, client.entity.unwrap()).unwrap();
			let mut extra_light = 0.0;

			if let Some(camera) = camera {
				camera_transform.position += camera.base + camera.offset;
				extra_light = camera.extra_light;
			}

			// Billboard matrix
			let rot = camera_transform.rotation[2].to_radians() as f32;
			#[rustfmt::skip]
				let billboard = Matrix4::new(
					 rot.sin(),  0.0, rot.cos(), 0.0,
					-rot.cos(),  0.0, rot.sin(), 0.0,
					 0.0      , -1.0, 0.0      , 0.0,
					 0.0      ,  0.0, 0.0      , 1.0,
				);

			// Group draws into batches by texture
			let mut batches: FnvHashMap<Arc<dyn ImageViewAbstract + Send + Sync>, Vec<Vertex>> =
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

				// Get image
				let image_info = &frame[index];
				let image = asset_storage.get(&image_info.handle).unwrap();
				let image_view = &image.image_view;

				// Determine light level
				let light_level = if sprite_render.full_bright {
					1.0
				} else {
					let ssect = map
						.find_subsector(Vector2::new(transform.position[0], transform.position[1]));
					map_dynamic.sectors[ssect.sector_index].light_level
				};

				// Set up vertices
				let mut vertices = [
					Vertex {
						in_position: Vector3::new(0.0, 0.0, 0.0),
						in_texture_coord: Vector2::new(0.0, 0.0),
						in_light_level: light_level + extra_light,
					},
					Vertex {
						in_position: Vector3::new(0.0, 1.0, 0.0),
						in_texture_coord: Vector2::new(0.0, 1.0),
						in_light_level: light_level + extra_light,
					},
					Vertex {
						in_position: Vector3::new(1.0, 1.0, 0.0),
						in_texture_coord: Vector2::new(image_info.flip, 1.0),
						in_light_level: light_level + extra_light,
					},
					Vertex {
						in_position: Vector3::new(1.0, 0.0, 0.0),
						in_texture_coord: Vector2::new(image_info.flip, 0.0),
						in_light_level: light_level + extra_light,
					},
				];
				let transform =
					Matrix4::new_translation(&transform.position)
						* billboard * Matrix4::new_translation(&-image.offset.fixed_resize(0.0))
						* Matrix4::new_nonuniform_scaling(&image.size().fixed_resize(1.0));
				vertices.iter_mut().for_each(|v| {
					v.in_position =
						(transform * v.in_position.fixed_resize::<4, 1>(1.0)).fixed_resize(0.0)
				});
				let vertices = [0, 1, 2, 0, 2, 3].into_iter().map(|i| vertices[i]);

				// Add to batches
				match batches.entry(image_view.clone()) {
					Entry::Occupied(mut entry) => {
						entry.get_mut().extend(vertices);
					}
					Entry::Vacant(entry) => {
						entry.insert(vertices.collect());
					}
				}
			}

			// Draw the batches
			command_buffer.bind_pipeline_graphics(pipeline.clone());

			for (image_view, vertices) in batches {
				let descriptor_set = {
					let mut builder = texture_set_pool.next();
					builder
						.add_image(image_view)
						.context("Couldn't add image to descriptor set")?;
					builder.build().context("Couldn't create descriptor set")?
				};
				command_buffer.bind_descriptor_sets(
					PipelineBindPoint::Graphics,
					pipeline.layout().clone(),
					1,
					descriptor_set,
				);

				let vertex_buffer = vertex_buffer_pool
					.chunk(vertices)
					.context("Couldn't create instance buffer")?;
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
	pub in_position: Vector3<f32>,
	pub in_texture_coord: Vector2<f32>,
	pub in_light_level: f32,
}

unsafe impl VertexTrait for Vertex {
	#[inline(always)]
	fn member(name: &str) -> Option<VertexMemberInfo> {
		match name {
			"in_position" => Some(VertexMemberInfo {
				offset: offset_of!(Vertex, in_position),
				ty: VertexMemberTy::F32,
				array_size: 3,
			}),
			"in_texture_coord" => Some(VertexMemberInfo {
				offset: offset_of!(Vertex, in_texture_coord),
				ty: VertexMemberTy::F32,
				array_size: 2,
			}),
			"in_light_level" => Some(VertexMemberInfo {
				offset: offset_of!(Vertex, in_light_level),
				ty: VertexMemberTy::F32,
				array_size: 1,
			}),
			_ => None,
		}
	}
}
