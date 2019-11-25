use crate::{
	assets::AssetHandle,
	renderer::mesh::Mesh,
	renderer::texture::Texture,
};

pub struct BSPModel {
	mesh: Mesh,
	faces: Vec<Face>,
	lightmap: AssetHandle<Texture>,
}

impl BSPModel {
	pub fn new(mesh: Mesh, faces: Vec<Face>, lightmap: AssetHandle<Texture>) -> BSPModel {
		BSPModel {
			mesh,
			faces,
			lightmap,
		}
	}

	pub fn mesh(&self) -> &Mesh {
		&self.mesh
	}

	pub fn faces(&self) -> &Vec<Face> {
		&self.faces
	}

	pub fn lightmap(&self) -> AssetHandle<Texture> {
		self.lightmap.clone()
	}
}

#[derive(Debug, Default, Clone)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 3],
	pub in_lightmap_coord: [f32; 3],
}
impl_vertex!(VertexData, in_position, in_texture_coord, in_lightmap_coord);

pub struct Face {
	pub first_vertex_index: usize,
	pub vertex_count: usize,
	pub texture: AssetHandle<Texture>,
}
