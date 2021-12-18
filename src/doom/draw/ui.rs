use crate::{
	common::{
		assets::AssetStorage,
		geometry::{ortho_matrix, Interval, AABB3},
		video::{DrawTarget, RenderContext},
	},
	doom::{
		assets::font::FontSpacing,
		draw::{world::draw_world, wsprite::draw_weapon_sprites},
		ui::{Hidden, UiGameView, UiHexFontText, UiImage, UiParams, UiText, UiTransform},
	},
};
use anyhow::Context;
use legion::{component, systems::ResourceSet, Entity, IntoQuery, Read, Resources, World};
use memoffset::offset_of;
use nalgebra::{Vector2, Vector3};
use std::{cmp::Ordering, sync::Arc};
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool, TypedBufferAccess},
	command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
	descriptor_set::{SingleLayoutDescSetPool, WriteDescriptorSet},
	image::view::ImageViewAbstract,
	impl_vertex,
	pipeline::{
		graphics::{
			input_assembly::{InputAssemblyState, PrimitiveTopology},
			vertex_input::{
				BuffersDefinition, Vertex as VertexTrait, VertexMemberInfo, VertexMemberTy,
			},
			viewport::{Viewport, ViewportState},
		},
		GraphicsPipeline, Pipeline, PipelineBindPoint,
	},
	render_pass::Subpass,
	sampler::Sampler,
};

pub fn draw_ui(
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
	let vert = ui_vert::load(device.clone()).context("Couldn't load shader")?;
	let frag = ui_frag::load(device.clone()).context("Couldn't load shader")?;

	let pipeline = GraphicsPipeline::start()
		.render_pass(
			Subpass::from(draw_target.render_pass().clone(), 0)
				.context("Subpass index out of range")?,
		)
		.vertex_shader(
			vert.entry_point("main")
				.context("Couldn't find entry point \"main\"")?,
			(),
		)
		.fragment_shader(
			frag.entry_point("main")
				.context("Couldn't find entry point \"main\"")?,
			(),
		)
		.vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
		.input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::TriangleList))
		.viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
		.with_auto_layout(device.clone(), |set_descs| {
			set_descs[1].set_immutable_samplers(0, [sampler.clone()]);
		})
		.context("Couldn't create UI pipeline")?;

	let layout = &pipeline.layout().descriptor_set_layouts()[0];
	let mut matrix_set_pool = SingleLayoutDescSetPool::new(layout.clone());
	let vertex_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer());
	let matrix_uniform_pool = CpuBufferPool::new(
		render_context.device().clone(),
		BufferUsage::uniform_buffer(),
	);
	let mut texture_set_pool =
		SingleLayoutDescSetPool::new(pipeline.layout().descriptor_set_layouts()[1].clone());

	let mut queries = (
		<(Entity, &UiTransform)>::query().filter(!component::<Hidden>()),
		<(
			&UiTransform,
			Option<&UiGameView>,
			Option<&UiImage>,
			Option<&UiText>,
			Option<&UiHexFontText>,
		)>::query(),
	);

	drop(draw_target);
	drop(render_context);
	drop(sampler);
	let mut draw_world = draw_world(resources)?;
	let mut draw_weapon_sprites = draw_weapon_sprites(resources)?;

	Ok(
		move |command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
		      world: &World,
		      resources: &Resources|
		      -> anyhow::Result<()> {
			let (asset_storage, ui_params) =
				<(Read<AssetStorage>, Read<UiParams>)>::fetch(resources);

			let viewport = Viewport {
				origin: [0.0; 2],
				dimensions: ui_params.framebuffer_dimensions().into(),
				depth_range: 0.0..1.0,
			};
			command_buffer.set_viewport(0, [viewport.clone()]);
			command_buffer.bind_pipeline_graphics(pipeline.clone());

			let proj = ortho_matrix(AABB3::from_intervals(Vector3::new(
				Interval::new(0.0, ui_params.framebuffer_dimensions()[0]),
				Interval::new(0.0, ui_params.framebuffer_dimensions()[1]),
				Interval::new(1000.0, 0.0),
			)));
			let framebuffer_ratio = ui_params
				.framebuffer_dimensions()
				.component_div(&ui_params.dimensions());

			// Create matrix uniform buffer
			let uniform_buffer = matrix_uniform_pool
				.next(Matrices { proj: proj.into() })
				.context("Couldn't create buffer")?;
			let descriptor_set = matrix_set_pool
				.next([WriteDescriptorSet::buffer(0, uniform_buffer)])
				.context("Couldn't create descriptor set")?;
			command_buffer.bind_descriptor_sets(
				PipelineBindPoint::Graphics,
				pipeline.layout().clone(),
				0,
				descriptor_set.clone(),
			);

			// Sort UiTransform entities by depth
			let mut entities: Vec<(f32, Entity)> = queries
				.0
				.iter(world)
				.map(|(&entity, ui_transform)| (ui_transform.depth, entity))
				.collect();
			entities.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

			// Group draws into batches by texture, preserving depth order
			let mut batches: Vec<(Arc<dyn ImageViewAbstract>, Vec<Vertex>)> = Vec::new();

			for (ui_transform, ui_game_view, ui_image, ui_text, ui_hexfont_text) in entities
				.into_iter()
				.filter_map(|(_, entity)| queries.1.get(world, entity).ok())
			{
				let position = ui_transform.position + ui_params.align(ui_transform.alignment);
				let size = ui_transform.size + ui_params.stretch(ui_transform.stretch);

				if let Some(UiGameView) = ui_game_view {
					command_buffer.set_viewport(
						0,
						[Viewport {
							origin: position.component_mul(&framebuffer_ratio).into(),
							dimensions: size.component_mul(&framebuffer_ratio).into(),
							depth_range: 0.0..1.0,
						}],
					);
					draw_world(command_buffer, world, resources)?;
					draw_weapon_sprites(command_buffer, world, resources)?;

					command_buffer.set_viewport(0, [viewport.clone()]);
					command_buffer.bind_pipeline_graphics(pipeline.clone());
					command_buffer.bind_descriptor_sets(
						PipelineBindPoint::Graphics,
						pipeline.layout().clone(),
						0,
						descriptor_set.clone(),
					);
				}

				if let Some(ui_image) = ui_image {
					let image = asset_storage.get(&ui_image.image).unwrap();
					let image_view = &image.image_view;
					let position = position - image.offset;
					let vertices = VERTICES.map(|v| Vertex {
						in_position: (v.in_position.component_mul(&size) + position)
							.component_mul(&framebuffer_ratio),
						in_texture_coord: v
							.in_texture_coord
							.component_mul(&size)
							.component_div(&image.size()),
					});
					let vertices = [0, 1, 2, 0, 2, 3].into_iter().map(|i| vertices[i]);
					match batches.last_mut() {
						Some((i, id)) if i == image_view => id.extend(vertices),
						_ => batches.push((image_view.clone(), vertices.collect())),
					}
				}

				if let Some(ui_text) = ui_text {
					let font = asset_storage.get(&ui_text.font).unwrap();
					let mut cursor_position = position;

					for ch in ui_text.text.chars() {
						if ch == ' ' {
							let width = match font.spacing {
								FontSpacing::FixedWidth { width } => width,
								FontSpacing::VariableWidth { space_width } => space_width,
							};
							cursor_position[0] += width;
						} else if let Some(image_handle) = font.characters.get(&ch) {
							let image = asset_storage.get(image_handle).unwrap();
							let image_view = &image.image_view;
							let position = cursor_position - image.offset;
							let vertices = VERTICES.map(|v| Vertex {
								in_position: (v.in_position.component_mul(&image.size())
									+ position)
									.component_mul(&framebuffer_ratio),
								in_texture_coord: v.in_texture_coord,
							});
							let vertices = [0, 1, 2, 0, 2, 3].into_iter().map(|i| vertices[i]);
							match batches.last_mut() {
								Some((i, id)) if i == image_view => id.extend(vertices),
								_ => batches.push((image_view.clone(), vertices.collect())),
							}

							// Move cursor
							let width = match font.spacing {
								FontSpacing::FixedWidth { width } => width,
								FontSpacing::VariableWidth { .. } => image.size()[0],
							};
							cursor_position[0] += width;
						}
					}
				}

				if let Some(ui_text) = ui_hexfont_text {
					let font = asset_storage.get(&ui_text.font).unwrap();
					let image_view = &font.image_view;
					let mut cursor_position = position;
					let start_of_line = cursor_position[0];

					for line in ui_text.lines.iter().map(|line| line.trim_end()) {
						for ch in line.chars().filter_map(|ch| font.chars.get(&ch)) {
							let vertices = ch.vertices.map(|v| Vertex {
								in_position: (v.in_position + cursor_position)
									.component_mul(&framebuffer_ratio),
								in_texture_coord: v.in_texture_coord,
							});
							let vertices = [0, 1, 2, 0, 2, 3].into_iter().map(|i| vertices[i]);
							match batches.last_mut() {
								Some((i, id)) if i == image_view => id.extend(vertices),
								_ => batches.push((image_view.clone(), vertices.collect())),
							}

							cursor_position[0] += ch.width;
						}

						// Move cursor
						cursor_position[0] = start_of_line;
						cursor_position[1] += font.line_height as f32;

						if cursor_position[1] + font.line_height as f32 > ui_transform.size[1] {
							// No more room for another line
							break;
						}
					}
				}
			}

			// Draw the batches
			for (image_view, vertices) in batches {
				let descriptor_set = texture_set_pool
					.next([WriteDescriptorSet::image_view(0, image_view)])
					.context("Couldn't create descriptor set")?;
				command_buffer.bind_descriptor_sets(
					PipelineBindPoint::Graphics,
					pipeline.layout().clone(),
					1,
					descriptor_set,
				);

				let vertex_buffer = vertex_buffer_pool
					.chunk(vertices)
					.context("Couldn't create buffer")?;
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

pub mod ui_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/ui.vert",
	}
}

pub use ui_vert::ty::Matrices;

pub mod ui_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/ui.frag",
	}
}

#[derive(Clone, Debug, Default)]
pub struct InstanceData {
	pub in_position: [f32; 2],
	pub in_size: [f32; 2],
	pub in_texture_position: [f32; 2],
	pub in_texture_size: [f32; 2],
}
impl_vertex!(
	InstanceData,
	in_position,
	in_size,
	in_texture_position,
	in_texture_size
);

#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
	pub in_position: Vector2<f32>,
	pub in_texture_coord: Vector2<f32>,
}

unsafe impl VertexTrait for Vertex {
	#[inline(always)]
	fn member(name: &str) -> Option<VertexMemberInfo> {
		match name {
			"in_position" => Some(VertexMemberInfo {
				offset: offset_of!(Vertex, in_position),
				ty: VertexMemberTy::F32,
				array_size: 2,
			}),
			"in_texture_coord" => Some(VertexMemberInfo {
				offset: offset_of!(Vertex, in_texture_coord),
				ty: VertexMemberTy::F32,
				array_size: 2,
			}),
			_ => None,
		}
	}
}

pub static VERTICES: [Vertex; 4] = [
	Vertex {
		in_position: Vector2::new(0.0, 0.0),
		in_texture_coord: Vector2::new(0.0, 0.0),
	},
	Vertex {
		in_position: Vector2::new(0.0, 1.0),
		in_texture_coord: Vector2::new(0.0, 1.0),
	},
	Vertex {
		in_position: Vector2::new(1.0, 1.0),
		in_texture_coord: Vector2::new(1.0, 1.0),
	},
	Vertex {
		in_position: Vector2::new(1.0, 0.0),
		in_texture_coord: Vector2::new(1.0, 0.0),
	},
];
