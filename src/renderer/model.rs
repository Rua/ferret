use crate::{
	assets::AssetHandle,
	renderer::mesh::Mesh,
	renderer::texture::Texture,
};

pub struct BSPModel {
	meshes: Vec<(AssetHandle<Texture>, Mesh)>,
}

impl BSPModel {
	pub fn new(meshes: Vec<(AssetHandle<Texture>, Mesh)>) -> BSPModel {
		BSPModel {
			meshes,
		}
	}

	pub fn meshes(&self) -> &Vec<(AssetHandle<Texture>, Mesh)> {
		&self.meshes
	}
}

#[derive(Debug, Default, Clone)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 3],
	pub in_lightlevel: f32,
}
impl_vertex!(VertexData, in_position, in_texture_coord, in_lightlevel);
