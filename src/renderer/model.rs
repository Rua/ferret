use crate::{
	assets::AssetHandle,
	renderer::mesh::Mesh,
	renderer::texture::Texture,
};

pub struct BSPModel {
	mesh: Mesh,
	faces: Vec<Face>,
}

impl BSPModel {
	pub fn new(mesh: Mesh, faces: Vec<Face>) -> BSPModel {
		BSPModel {
			mesh,
			faces,
		}
	}

	pub fn mesh(&self) -> &Mesh {
		&self.mesh
	}

	pub fn faces(&self) -> &Vec<Face> {
		&self.faces
	}
}

#[derive(Debug, Default, Clone)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 3],
	pub in_lightlevel: f32,
}
impl_vertex!(VertexData, in_position, in_texture_coord, in_lightlevel);

pub struct Face {
	pub first_vertex_index: usize,
	pub vertex_count: usize,
	pub texture: AssetHandle<Texture>,
}
