use crate::{
	common::{
		geometry::{perspective_matrix, Interval},
		video::{DrawContext, RenderContext},
	},
	doom::{camera::Camera, client::Client, components::Transform, ui::UiParams},
};
use anyhow::Context;
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Read, Resources, SystemBuilder,
};
use nalgebra::{Matrix4, Vector2, Vector3};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	descriptor_set::{
		layout::{DescriptorBufferDesc, DescriptorDesc, DescriptorDescTy, DescriptorSetLayout},
		FixedSizeDescriptorSetsPool,
	},
	pipeline::shader::ShaderStages,
};

pub fn draw_world(resources: &mut Resources) -> anyhow::Result<impl Runnable> {
	let render_context = <Read<RenderContext>>::fetch(resources);

	// Create descriptor sets pool for matrices
	let descriptors = [Some(DescriptorDesc {
		ty: DescriptorDescTy::Buffer(DescriptorBufferDesc {
			dynamic: Some(false),
			storage: false,
		}),
		array_count: 1,
		stages: ShaderStages {
			vertex: true,
			..ShaderStages::none()
		},
		readonly: true,
	})];

	let layout = Arc::new(
		DescriptorSetLayout::new(render_context.device().clone(), descriptors.iter().cloned())
			.context("Couldn't create descriptor set layout")?,
	);
	let mut matrix_set_pool = FixedSizeDescriptorSetsPool::new(layout);

	let matrix_uniform_pool = CpuBufferPool::new(
		render_context.device().clone(),
		BufferUsage::uniform_buffer(),
	);

	Ok(SystemBuilder::new("draw_world")
		.read_resource::<Client>()
		.read_resource::<UiParams>()
		.write_resource::<Option<DrawContext>>()
		.with_query(<(Option<&Camera>, &Transform)>::query())
		.build(move |_command_buffer, world, resources, query| {
			(|| -> anyhow::Result<()> {
				let (client, ui_params, draw_context) = resources;
				let draw_context = draw_context.as_mut().unwrap();

				// Projection matrix
				// Doom had non-square pixels, with a resolution of 320x200 (16:10) running on a 4:3
				// screen. This caused everything to be stretched vertically by some degree, and the game
				// art was made with that in mind.
				// The 1.2 factor here applies the same stretching as in the original.
				let proj = perspective_matrix(
					Vector2::new(1.0, 168.0 / 320.0).component_mul(&ui_params.factors()),
					Interval::new(4.0, 20000.0),
				);

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

				// Create matrix UBO
				draw_context.descriptor_sets.truncate(0);
				draw_context.descriptor_sets.push(Arc::new(
					matrix_set_pool
						.next()
						.add_buffer(
							matrix_uniform_pool
								.next(world_vert::ty::Matrices {
									proj: proj.into(),
									view: view.into(),
								})
								.context("Couldn't create buffer")?,
						)
						.context("Couldn't add buffer to descriptor set")?
						.build()
						.context("Couldn't create descriptor set")?,
				));

				Ok(())
			})()
			.unwrap_or_else(|e| panic!("{:?}", e));
		}))
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
