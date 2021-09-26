use crate::{
	common::{
		geometry::{perspective_matrix, Interval},
		video::RenderContext,
	},
	doom::{
		draw::{map::draw_map, sprite::draw_sprites, NON_SQUARE_CORRECTION},
		game::{camera::Camera, client::Client, Transform},
	},
};
use anyhow::Context;
use legion::{systems::ResourceSet, IntoQuery, Read, Resources, World};
use nalgebra::{Matrix4, Vector2, Vector3};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
	descriptor_set::{
		layout::{DescriptorDesc, DescriptorDescTy, DescriptorSetLayout},
		SingleLayoutDescSetPool,
	},
	pipeline::{layout::PipelineLayout, shader::ShaderStages, PipelineBindPoint},
};

pub fn draw_world(
	resources: &mut Resources,
) -> anyhow::Result<
	impl FnMut(
		&mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
		&World,
		&Resources,
	) -> anyhow::Result<()>,
> {
	let render_context = <Read<RenderContext>>::fetch(resources);

	// Create descriptor sets pool for matrices
	let descriptors = [Some(DescriptorDesc {
		ty: DescriptorDescTy::UniformBuffer,
		descriptor_count: 1,
		stages: ShaderStages {
			vertex: true,
			..ShaderStages::none()
		},
		mutable: false,
		variable_count: false,
	})];

	let descriptor_set_layout = Arc::new(
		DescriptorSetLayout::new(
			render_context.device().clone(),
			std::array::IntoIter::new(descriptors),
		)
		.context("Couldn't create descriptor set layout")?,
	);
	let mut matrix_set_pool = SingleLayoutDescSetPool::new(descriptor_set_layout.clone());
	let pipeline_layout = Arc::new(
		PipelineLayout::new(render_context.device().clone(), [descriptor_set_layout], [])
			.context("Couldn't create pipeline layout")?,
	);

	let matrix_uniform_pool = CpuBufferPool::new(
		render_context.device().clone(),
		BufferUsage::uniform_buffer(),
	);

	let mut query = <(Option<&Camera>, &Transform)>::query();

	drop(render_context);
	let mut draw_map = draw_map(resources)?;
	let mut draw_sprites = draw_sprites(resources)?;

	Ok(
		move |command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
		      world: &World,
		      resources: &Resources|
		      -> anyhow::Result<()> {
			let client = <Read<Client>>::fetch(resources);
			let viewport = command_buffer.inner().current_viewport(0).unwrap();

			// Projection matrix
			const MIN_ASPECT_RATIO: f32 = 168.0 / 320.0 * NON_SQUARE_CORRECTION;
			let aspect_ratio = viewport.dimensions[1] / viewport.dimensions[0];

			let fov = if aspect_ratio < MIN_ASPECT_RATIO {
				Vector2::new(1.0 / aspect_ratio, 1.0) * MIN_ASPECT_RATIO
			} else {
				Vector2::new(1.0, aspect_ratio)
			}
			.component_div(&Vector2::new(1.0, NON_SQUARE_CORRECTION));

			let proj = perspective_matrix(fov, Interval::new(4.0, 20000.0));

			// View matrix
			let (camera, &(mut camera_transform)) =
				query.get(world, client.entity.unwrap()).unwrap();

			if let Some(camera) = camera {
				camera_transform.position += camera.base + camera.offset;
			}

			let view = Matrix4::new_rotation(Vector3::new(
				-camera_transform.rotation[0].to_radians() as f32,
				0.0,
				0.0,
			)) * Matrix4::new_rotation(Vector3::new(
				0.0,
				-camera_transform.rotation[1].to_radians() as f32,
				0.0,
			)) * Matrix4::new_rotation(Vector3::new(
				0.0,
				0.0,
				-camera_transform.rotation[2].to_radians() as f32,
			)) * Matrix4::new_translation(&-camera_transform.position);

			// Create matrix uniform buffer
			let uniform_buffer = Arc::new(
				matrix_uniform_pool
					.next(world_vert::ty::Matrices {
						proj: proj.into(),
						view: view.into(),
					})
					.context("Couldn't create buffer")?,
			);
			let descriptor_set = {
				let mut builder = matrix_set_pool.next();
				builder
					.add_buffer(uniform_buffer)
					.context("Couldn't add buffer to descriptor set")?;
				builder.build().context("Couldn't create descriptor set")?
			};
			command_buffer.bind_descriptor_sets(
				PipelineBindPoint::Graphics,
				pipeline_layout.clone(),
				0,
				descriptor_set,
			);

			draw_map(command_buffer, world, resources)?;
			draw_sprites(command_buffer, world, resources)?;

			Ok(())
		},
	)
}

pub mod world_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/world.vert",
	}
}

pub mod world_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/world.frag",
	}
}
