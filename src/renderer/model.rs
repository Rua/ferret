use crate::{
	assets::AssetHandle,
	geometry::{BoundingBox3, Plane},
	renderer::mesh::Mesh,
	renderer::texture::Texture,
};

pub struct BSPModel {
	mesh: Mesh,
	faces: Vec<Face>,
	_leaves: Vec<BSPLeaf>,
	_branches: Vec<BSPBranch>,
}

impl BSPModel {
	pub fn new(
		mesh: Mesh,
		faces: Vec<Face>,
		leaves: Vec<BSPLeaf>,
		branches: Vec<BSPBranch>,
	) -> BSPModel {
		BSPModel {
			mesh,
			faces,
			_leaves: leaves,
			_branches: branches,
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
	pub in_lightmap_coord: [f32; 3],
}
impl_vertex!(VertexData, in_position, in_texture_coord, in_lightmap_coord);

pub struct Face {
	pub first_vertex_index: usize,
	pub vertex_count: usize,
	pub texture: AssetHandle<Texture>,
	pub lightmap: AssetHandle<Texture>,
}

#[derive(Debug, Clone)]
pub struct BSPBranch {
	pub plane: Plane,
	pub bounding_box: BoundingBox3,
	pub children: [BSPNode; 2],
}

#[derive(Debug, Clone)]
pub struct BSPLeaf {
	pub first_face_index: usize,
	pub face_count: usize,
	pub bounding_box: BoundingBox3,
}

#[derive(Debug, Copy, Clone)]
pub enum BSPNode {
	Leaf(usize),
	Branch(usize),
}
