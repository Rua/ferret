use crate::{
	common::{
		assets::AssetStorage,
		video::{AsBytes, DrawContext, DrawTarget, RenderContext},
	},
	doom::{
		camera::Camera,
		client::Client,
		components::Transform,
		draw::world::{world_frag, world_vert},
		map::{
			meshes::{SkyVertex, Vertex},
			MapDynamic,
		},
		ui::{UiAlignment, UiParams, UiTransform},
	},
};
use anyhow::{anyhow, Context};
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Read, Resources, SystemBuilder,
};
use nalgebra::{Matrix4, Vector2};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool, ImmutableBuffer},
	command_buffer::DynamicState,
	descriptor::{descriptor_set::FixedSizeDescriptorSetsPool, PipelineLayoutAbstract},
	impl_vertex,
	pipeline::{
		vertex::OneVertexOneInstanceDefinition, viewport::Viewport, GraphicsPipeline,
		GraphicsPipelineAbstract,
	},
	render_pass::Subpass,
	sampler::Sampler,
};

pub fn draw_map(resources: &mut Resources) -> anyhow::Result<impl Runnable> {
	let (draw_target, render_context) = <(Read<DrawTarget>, Read<RenderContext>)>::fetch(resources);
	let device = render_context.device();

	// Create pipeline for normal parts of the map
	let world_vert = world_vert::Shader::load(device.clone()).context("Couldn't load shader")?;
	let world_frag = world_frag::Shader::load(device.clone()).context("Couldn't load shader")?;

	let normal_pipeline = Arc::new(
		GraphicsPipeline::start()
			.render_pass(
				Subpass::from(draw_target.render_pass().clone(), 0)
					.ok_or(anyhow!("Subpass index out of range"))?,
			)
			.vertex_input(OneVertexOneInstanceDefinition::<Vertex, Instance>::new())
			.vertex_shader(world_vert.main_entry_point(), ())
			.fragment_shader(world_frag.main_entry_point(), ())
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
				Subpass::from(draw_target.render_pass().clone(), 0)
					.context("Subpass index out of range")?,
			)
			.vertex_input_single_buffer::<SkyVertex>()
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

	let index_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::index_buffer());
	let vertex_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer());
	let (instance_buffer, _future) = ImmutableBuffer::from_iter(
		std::array::IntoIter::new([Instance {
			in_transform: Matrix4::identity().into(),
		}]),
		BufferUsage::vertex_buffer(),
		render_context.queues().graphics.clone(),
	)
	.context("Couldn't create instance buffer")?;

	let mut normal_texture_set_pool =
		FixedSizeDescriptorSetsPool::new(normal_pipeline.descriptor_set_layout(1).unwrap().clone());

	let sky_uniform_pool = CpuBufferPool::new(device.clone(), BufferUsage::uniform_buffer());
	let mut sky_texture_set_pool =
		FixedSizeDescriptorSetsPool::new(sky_pipeline.descriptor_set_layout(1).unwrap().clone());

	Ok(SystemBuilder::new("draw_map")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.read_resource::<Arc<Sampler>>()
		.read_resource::<UiParams>()
		.write_resource::<Option<DrawContext>>()
		.with_query(<(Option<&Camera>, &Transform)>::query())
		.with_query(<&MapDynamic>::query())
		.build(move |_command_buffer, world, resources, queries| {
			(|| -> anyhow::Result<()> {
				let (asset_storage, client, sampler, ui_params, draw_context) = resources;
				let draw_context = draw_context.as_mut().unwrap();

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

				for map_dynamic in queries.1.iter(world) {
					let map = asset_storage.get(&map_dynamic.map).unwrap();
					let (flat_meshes, wall_meshes, sky_mesh) =
						crate::doom::map::meshes::make_meshes(
							map,
							map_dynamic,
							extra_light,
							asset_storage,
						)
						.context("Couldn't generate map mesh")?;

					// Draw the walls
					for (handle, mesh) in wall_meshes {
						let vertex_buffer = vertex_buffer_pool
							.chunk(mesh.0.as_bytes().iter().copied())
							.context("Couldn't create buffer")?;
						let index_buffer = index_buffer_pool
							.chunk(mesh.1)
							.context("Couldn't create buffer")?;

						// Redirect animation frames
						let handle = if let Some(anim_state) = map_dynamic.anim_states.get(&handle)
						{
							let anim = &map.anims[&handle];
							&anim.frames[anim_state.frame]
						} else {
							&handle
						};
						let image_view = &asset_storage.get(&handle).unwrap().image_view;

						draw_context.descriptor_sets.truncate(1);
						draw_context.descriptor_sets.push(Arc::new(
							normal_texture_set_pool
								.next()
								.add_sampled_image(image_view.clone(), sampler.clone())
								.context("Couldn't add image to descriptor set")?
								.build()
								.context("Couldn't create descriptor set")?,
						));

						draw_context
							.commands
							.draw_indexed(
								normal_pipeline.clone(),
								&dynamic_state,
								vec![Arc::new(vertex_buffer), instance_buffer.clone()],
								index_buffer,
								draw_context.descriptor_sets.clone(),
								(),
								std::iter::empty(),
							)
							.context("Couldn't issue draw to command buffer")?;
					}

					// Draw the flats
					for (handle, mesh) in flat_meshes {
						let vertex_buffer = vertex_buffer_pool
							.chunk(mesh.0.as_bytes().iter().copied())
							.context("Couldn't create buffer")?;
						let index_buffer = index_buffer_pool
							.chunk(mesh.1)
							.context("Couldn't create buffer")?;

						// Redirect animation frames
						let handle = if let Some(anim_state) = map_dynamic.anim_states.get(&handle)
						{
							let anim = &map.anims[&handle];
							&anim.frames[anim_state.frame]
						} else {
							&handle
						};
						let image = asset_storage.get(handle).unwrap();

						draw_context.descriptor_sets.truncate(1);
						draw_context.descriptor_sets.push(Arc::new(
							normal_texture_set_pool
								.next()
								.add_sampled_image(image.image_view.clone(), sampler.clone())
								.context("Couldn't add image to descriptor set")?
								.build()
								.context("Couldn't create descriptor set")?,
						));

						draw_context
							.commands
							.draw_indexed(
								normal_pipeline.clone(),
								&dynamic_state,
								vec![Arc::new(vertex_buffer), instance_buffer.clone()],
								index_buffer,
								draw_context.descriptor_sets.clone(),
								(),
								std::iter::empty(),
							)
							.context("Couldn't issue draw to command buffer")?;
					}

					// Draw the sky
					let vertex_buffer = vertex_buffer_pool
						.chunk(sky_mesh.0.as_bytes().iter().copied())
						.context("Couldn't create buffer")?;
					let index_buffer = index_buffer_pool
						.chunk(sky_mesh.1)
						.context("Couldn't create buffer")?;
					let image = asset_storage.get(&map.sky).unwrap();
					let sky_buffer = sky_uniform_pool
						.next(sky_frag::ty::FragParams {
							screenSize: [800.0, 600.0],
							pitch: camera_transform.rotation[1].to_degrees() as f32,
							yaw: camera_transform.rotation[2].to_degrees() as f32,
						})
						.context("Couldn't create buffer")?;

					draw_context.descriptor_sets.truncate(1);
					draw_context.descriptor_sets.push(Arc::new(
						sky_texture_set_pool
							.next()
							.add_sampled_image(image.image_view.clone(), sampler.clone())
							.context("Couldn't add image to descriptor set")?
							.add_buffer(sky_buffer)?
							.build()
							.context("Couldn't create descriptor set")?,
					));

					draw_context
						.commands
						.draw_indexed(
							sky_pipeline.clone(),
							&dynamic_state,
							vec![Arc::new(vertex_buffer)],
							index_buffer,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Instance {
	pub in_transform: [[f32; 4]; 4],
}
impl_vertex!(Instance, in_transform);

mod sky_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/sky.vert",
	}
}

mod sky_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/sky.frag",
	}
}
