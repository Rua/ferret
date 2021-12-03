use crate::{
	common::{
		assets::AssetStorage,
		video::{AsBytes, DrawTarget, RenderContext},
	},
	doom::{
		assets::map::meshes::{make_meshes, SkyVertex, Vertex},
		draw::world::{world_frag, world_vert},
		game::{camera::Camera, client::Client, map::MapDynamic, Transform},
	},
};
use anyhow::{anyhow, Context};
use legion::{systems::ResourceSet, IntoQuery, Read, Resources, World};
use nalgebra::Matrix4;
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool, ImmutableBuffer, TypedBufferAccess},
	command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
	descriptor_set::SingleLayoutDescSetPool,
	impl_vertex,
	pipeline::{
		depth_stencil::DepthStencilState,
		input_assembly::{InputAssemblyState, PrimitiveTopology},
		rasterization::{CullMode, RasterizationState},
		vertex::BuffersDefinition,
		viewport::ViewportState,
		GraphicsPipeline, Pipeline, PipelineBindPoint,
	},
	render_pass::Subpass,
	sampler::Sampler,
};

pub fn draw_map(
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

	// Create pipeline for normal parts of the map
	let world_vert = world_vert::load(device.clone()).context("Couldn't load shader")?;
	let world_frag = world_frag::load(device.clone()).context("Couldn't load shader")?;

	let normal_pipeline = GraphicsPipeline::start()
		.render_pass(
			Subpass::from(draw_target.render_pass().clone(), 0)
				.ok_or(anyhow!("Subpass index out of range"))?,
		)
		.vertex_shader(
			world_vert
				.entry_point("main")
				.context("Couldn't find entry point \"main\"")?,
			(),
		)
		.fragment_shader(
			world_frag
				.entry_point("main")
				.context("Couldn't find entry point \"main\"")?,
			(),
		)
		.vertex_input(
			BuffersDefinition::new()
				.vertex::<Vertex>()
				.instance::<Instance>(),
		)
		.input_assembly_state(
			InputAssemblyState::new()
				.topology(PrimitiveTopology::TriangleFan)
				.primitive_restart_enable(),
		)
		.viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
		.rasterization_state(RasterizationState::new().cull_mode(CullMode::Back))
		.depth_stencil_state(DepthStencilState::simple_depth_test())
		.with_auto_layout(device.clone(), |set_descs| {
			set_descs[1].set_immutable_samplers(0, [sampler.clone()]);
		})
		.context("Couldn't create map pipeline")?;

	// Create pipeline for sky
	let sky_vert = sky_vert::load(device.clone())?;
	let sky_frag = sky_frag::load(device.clone())?;

	let sky_pipeline = GraphicsPipeline::start()
		.render_pass(
			Subpass::from(draw_target.render_pass().clone(), 0)
				.context("Subpass index out of range")?,
		)
		.vertex_shader(
			sky_vert
				.entry_point("main")
				.context("Couldn't find entry point \"main\"")?,
			(),
		)
		.fragment_shader(
			sky_frag
				.entry_point("main")
				.context("Couldn't find entry point \"main\"")?,
			(),
		)
		.vertex_input_single_buffer::<SkyVertex>()
		.input_assembly_state(
			InputAssemblyState::new()
				.topology(PrimitiveTopology::TriangleFan)
				.primitive_restart_enable(),
		)
		.viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
		.rasterization_state(RasterizationState::new().cull_mode(CullMode::Back))
		.depth_stencil_state(DepthStencilState::simple_depth_test())
		.with_auto_layout(device.clone(), |set_descs| {
			set_descs[1].set_immutable_samplers(0, [sampler.clone()]);
		})
		.context("Couldn't create sky pipeline")?;

	let index_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::index_buffer());
	let vertex_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer());
	let (instance_buffer, _future) = ImmutableBuffer::from_iter(
		[Instance {
			in_transform: Matrix4::identity().into(),
		}]
		.into_iter(),
		BufferUsage::vertex_buffer(),
		render_context.queues().graphics.clone(),
	)
	.context("Couldn't create instance buffer")?;
	let instance_count = instance_buffer.len() as u32;

	let mut normal_texture_set_pool =
		SingleLayoutDescSetPool::new(normal_pipeline.layout().descriptor_set_layouts()[1].clone());

	let sky_uniform_pool = CpuBufferPool::new(device.clone(), BufferUsage::uniform_buffer());
	let mut sky_texture_set_pool =
		SingleLayoutDescSetPool::new(sky_pipeline.layout().descriptor_set_layouts()[1].clone());

	let mut queries = (
		<(Option<&Camera>, &Transform)>::query(),
		<&MapDynamic>::query(),
	);

	Ok(
		move |command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
		      world: &World,
		      resources: &Resources|
		      -> anyhow::Result<()> {
			let (asset_storage, client) = <(Read<AssetStorage>, Read<Client>)>::fetch(resources);

			// Camera
			let (camera, &(mut camera_transform)) =
				queries.0.get(world, client.entity.unwrap()).unwrap();
			let mut extra_light = 0.0;

			if let Some(camera) = camera {
				camera_transform.position += camera.base + camera.offset;
				extra_light = camera.extra_light;
			}

			// Draw
			for map_dynamic in queries.1.iter(world) {
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let (flat_meshes, wall_meshes, sky_mesh) =
					make_meshes(map, map_dynamic, extra_light, &asset_storage)
						.context("Couldn't generate map mesh")?;

				command_buffer.bind_vertex_buffers(1, instance_buffer.clone());
				command_buffer.bind_pipeline_graphics(normal_pipeline.clone());

				// Draw the walls
				for (handle, mesh) in wall_meshes {
					// Redirect animation frames
					let handle = if let Some(anim_state) = map_dynamic.anim_states.get(&handle) {
						let anim = &map.anims[&handle];
						&anim.frames[anim_state.frame]
					} else {
						&handle
					};
					let image_view = &asset_storage.get(&handle).unwrap().image_view;

					// Draw
					let descriptor_set = {
						let mut builder = normal_texture_set_pool.next();
						builder
							.add_image(image_view.clone())
							.context("Couldn't add image to descriptor set")?;
						builder.build().context("Couldn't create descriptor set")?
					};
					command_buffer.bind_descriptor_sets(
						PipelineBindPoint::Graphics,
						normal_pipeline.layout().clone(),
						1,
						descriptor_set,
					);

					let vertex_buffer = vertex_buffer_pool
						.chunk(mesh.0.as_bytes().iter().copied())
						.context("Couldn't create buffer")?;
					command_buffer.bind_vertex_buffers(0, vertex_buffer);

					let index_buffer = index_buffer_pool
						.chunk(mesh.1)
						.context("Couldn't create buffer")?;
					let index_count = index_buffer.len() as u32;
					command_buffer.bind_index_buffer(index_buffer);

					command_buffer
						.draw_indexed(index_count, instance_count, 0, 0, 0)
						.context("Couldn't issue wall draw to command buffer")?;
				}

				// Draw the flats
				for (handle, mesh) in flat_meshes {
					// Redirect animation frames
					let handle = if let Some(anim_state) = map_dynamic.anim_states.get(&handle) {
						let anim = &map.anims[&handle];
						&anim.frames[anim_state.frame]
					} else {
						&handle
					};
					let image = asset_storage.get(handle).unwrap();

					let descriptor_set = {
						let mut builder = normal_texture_set_pool.next();
						builder
							.add_image(image.image_view.clone())
							.context("Couldn't add image to descriptor set")?;
						builder.build().context("Couldn't create descriptor set")?
					};
					command_buffer.bind_descriptor_sets(
						PipelineBindPoint::Graphics,
						normal_pipeline.layout().clone(),
						1,
						descriptor_set,
					);

					let vertex_buffer = vertex_buffer_pool
						.chunk(mesh.0.as_bytes().iter().copied())
						.context("Couldn't create buffer")?;
					command_buffer.bind_vertex_buffers(0, vertex_buffer);

					let index_buffer = index_buffer_pool
						.chunk(mesh.1)
						.context("Couldn't create buffer")?;
					let index_count = index_buffer.len() as u32;
					command_buffer.bind_index_buffer(index_buffer);

					command_buffer
						.draw_indexed(index_count, instance_count, 0, 0, 0)
						.context("Couldn't issue flat draw to command buffer")?;
				}

				// Draw the sky
				command_buffer.bind_pipeline_graphics(sky_pipeline.clone());

				let image = asset_storage.get(&map.sky).unwrap();
				let sky_buffer = sky_uniform_pool
					.next(sky_frag::ty::FragParams {
						screenSize: [800.0, 600.0],
						pitch: camera_transform.rotation[1].to_degrees() as f32,
						yaw: camera_transform.rotation[2].to_degrees() as f32,
					})
					.context("Couldn't create buffer")?;
				let descriptor_set = {
					let mut builder = sky_texture_set_pool.next();
					builder
						.add_image(image.image_view.clone())
						.context("Couldn't add image to descriptor set")?
						.add_buffer(sky_buffer)
						.context("Couldn't add buffer to descriptor set")?;
					builder.build().context("Couldn't create descriptor set")?
				};
				command_buffer.bind_descriptor_sets(
					PipelineBindPoint::Graphics,
					sky_pipeline.layout().clone(),
					1,
					descriptor_set,
				);

				let vertex_buffer = vertex_buffer_pool
					.chunk(sky_mesh.0.as_bytes().iter().copied())
					.context("Couldn't create buffer")?;
				command_buffer.bind_vertex_buffers(0, vertex_buffer);

				let index_buffer = index_buffer_pool
					.chunk(sky_mesh.1)
					.context("Couldn't create buffer")?;
				let index_count = index_buffer.len() as u32;
				command_buffer.bind_index_buffer(index_buffer);

				command_buffer
					.draw_indexed(index_count, 1, 0, 0, 0)
					.context("Couldn't issue sky draw to command buffer")?;
			}

			Ok(())
		},
	)
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
