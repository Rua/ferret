use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::Angle,
		video::{DrawContext, DrawTarget, RenderContext},
	},
	doom::{
		camera::Camera,
		client::Client,
		components::Transform,
		draw::world::{world_frag, world_vert},
		map::MapDynamic,
		sprite::Sprite,
		ui::{UiAlignment, UiParams, UiTransform},
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
	buffer::{BufferUsage, CpuBufferPool},
	command_buffer::DynamicState,
	descriptor_set::FixedSizeDescriptorSetsPool,
	image::view::ImageViewAbstract,
	pipeline::{
		vertex::{BuffersDefinition, Vertex as VertexTrait, VertexMemberInfo, VertexMemberTy},
		viewport::Viewport,
		GraphicsPipeline, GraphicsPipelineAbstract,
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
) -> anyhow::Result<impl FnMut(&mut DrawContext, &World, &Resources)> {
	let (draw_target, render_context) = <(Read<DrawTarget>, Read<RenderContext>)>::fetch(resources);
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
			.build(device.clone())
			.context("Couldn't create sprite pipeline")?,
	) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

	let vertex_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer());
	let mut texture_set_pool =
		FixedSizeDescriptorSetsPool::new(pipeline.layout().descriptor_set_layouts()[1].clone());

	let mut queries = (
		<(Option<&Camera>, &Transform)>::query(),
		<&MapDynamic>::query(),
		<(Entity, &SpriteRender, &Transform)>::query(),
	);

	Ok(
		move |draw_context: &mut DrawContext, world: &World, resources: &Resources| {
			(|| -> anyhow::Result<()> {
				let (asset_storage, client, sampler, ui_params) = <(
					Read<AssetStorage>,
					Read<Client>,
					Read<Arc<Sampler>>,
					Read<UiParams>,
				)>::fetch(resources);
				let map_dynamic = queries.1.iter(world).next().unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();

				// Viewport
				let ui_transform = UiTransform {
					position: Vector2::new(0.0, 0.0),
					depth: 0.0,
					alignment: [UiAlignment::Near, UiAlignment::Near],
					size: Vector2::new(320.0, 168.0),
					stretch: [true, true],
				};
				let ratio = ui_params
					.framebuffer_dimensions()
					.component_div(&ui_params.dimensions());
				let position = ui_transform.position + ui_params.align(ui_transform.alignment);
				let size = ui_transform.size + ui_params.stretch(ui_transform.stretch);
				let dynamic_state = DynamicState {
					viewports: Some(vec![Viewport {
						origin: position.component_mul(&ratio).into(),
						dimensions: size.component_mul(&ratio).into(),
						depth_range: 0.0..1.0,
					}]),
					..DynamicState::none()
				};

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
						let ssect = map.find_subsector(Vector2::new(
							transform.position[0],
							transform.position[1],
						));
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
					let vertices = [0, 1, 2, 0, 2, 3].iter().map(|&i| vertices[i]);

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

					let vertex_buffer = vertex_buffer_pool
						.chunk(vertices)
						.context("Couldn't create instance buffer")?;

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
