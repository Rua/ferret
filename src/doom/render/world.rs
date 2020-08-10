use crate::{
	doom::{camera::Camera, client::Client, components::Transform, render::map::Matrices},
	geometry::{perspective_matrix, Interval},
	renderer::{DrawContext, DrawStep, RenderContext},
};
use anyhow::Context;
use legion::prelude::{EntityStore, Read, ResourceSet, Resources, World};
use nalgebra::{Matrix4, Vector3};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	descriptor::{
		descriptor::{DescriptorBufferDesc, DescriptorDesc, DescriptorDescTy, ShaderStages},
		descriptor_set::{FixedSizeDescriptorSetsPool, UnsafeDescriptorSetLayout},
	},
};

pub struct DrawWorld {
	matrix_uniform_pool: CpuBufferPool<Matrices>,
	matrix_set_pool: FixedSizeDescriptorSetsPool,
}

impl DrawWorld {
	pub fn new(render_context: &RenderContext) -> anyhow::Result<DrawWorld> {
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
			UnsafeDescriptorSetLayout::new(
				render_context.device().clone(),
				descriptors.iter().cloned(),
			)
			.context("Couldn't create descriptor set layout")?,
		);
		let matrix_set_pool = FixedSizeDescriptorSetsPool::new(layout);

		Ok(DrawWorld {
			matrix_uniform_pool: CpuBufferPool::new(
				render_context.device().clone(),
				BufferUsage::uniform_buffer(),
			),
			matrix_set_pool,
		})
	}
}

impl DrawStep for DrawWorld {
	fn draw(
		&mut self,
		draw_context: &mut DrawContext,
		world: &World,
		resources: &Resources,
	) -> anyhow::Result<()> {
		let framebuffer_dimensions = [
			draw_context.framebuffer.width() as f32,
			draw_context.framebuffer.height() as f32,
		];
		let ratio = (framebuffer_dimensions[0] / framebuffer_dimensions[1]) / (4.0 / 3.0);
		let viewport = &mut draw_context.dynamic_state.viewports.as_mut().unwrap()[0];
		viewport.origin = [0.0, 0.0];
		viewport.dimensions = [
			framebuffer_dimensions[0],
			ratio.min(1.0) * (1.0 - 32.0 / 200.0) * framebuffer_dimensions[1],
		];

		// Projection matrix
		// Doom had non-square pixels, with a resolution of 320x200 (16:10) running on a 4:3
		// screen. This caused everything to be stretched vertically by some degree, and the game
		// art was made with that in mind.
		// The 1.2 factor here applies the same stretching as in the original.
		let aspect_ratio = (viewport.dimensions[0] / viewport.dimensions[1]) * 1.2;
		let proj = perspective_matrix(90.0, aspect_ratio, Interval::new(1.0, 20000.0));

		// View matrix
		let client = <Read<Client>>::fetch(resources);
		let camera_entity = client.entity.unwrap();

		let Transform {
			mut position,
			rotation,
		} = *world.get_component::<Transform>(camera_entity).unwrap();

		if let Some(camera) = world.get_component::<Camera>(camera_entity) {
			position += camera.base + camera.offset;
		}

		let view = Matrix4::new_rotation(Vector3::new(-rotation[0].to_radians() as f32, 0.0, 0.0))
			* Matrix4::new_rotation(Vector3::new(0.0, -rotation[1].to_radians() as f32, 0.0))
			* Matrix4::new_rotation(Vector3::new(0.0, 0.0, -rotation[2].to_radians() as f32))
			* Matrix4::new_translation(&-position);

		// Billboard matrix
		let billboard =
			Matrix4::new_rotation(Vector3::new(0.0, 0.0, rotation[2].to_radians() as f32));

		// Create matrix UBO
		draw_context.descriptor_sets.truncate(0);
		draw_context.descriptor_sets.push(Arc::new(
			self.matrix_set_pool
				.next()
				.add_buffer(self.matrix_uniform_pool.next(Matrices {
					proj: proj.into(),
					view: view.into(),
					billboard: billboard.into(),
				})?)?
				.build()?,
		));

		Ok(())
	}
}

pub mod normal_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/normal.frag",
	}
}
