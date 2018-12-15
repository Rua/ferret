use byteorder::{LE, ReadBytesExt};
use nalgebra::{Vector2, Vector3};
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::{ErrorKind, Read};
use std::str;

use crate::geometry::{BoundingBox2, BoundingBox3, Plane};
use crate::model::{BSPBranch, BSPLeaf, BSPModel, BSPNode, Face, Texture, VertexData};
use crate::doom::types::{DoomFlatLoader, DoomTextureLoader, palette};
use crate::doom::wad::WadLoader;

pub fn from_wad(name: &str, loader: &mut WadLoader) -> Result<BSPModel, Box<Error>> {
	let index = loader.index_for_name(name).unwrap();
	let _things = things::from_data(&mut loader.read_lump(index + things::OFFSET)?)?;
	let linedefs = linedefs::from_data(&mut loader.read_lump(index + linedefs::OFFSET)?)?;
	let sidedefs = sidedefs::from_data(&mut loader.read_lump(index + sidedefs::OFFSET)?)?;
	let vertexes = vertexes::from_data(&mut loader.read_lump(index + vertexes::OFFSET)?)?;
	let sectors = sectors::from_data(&mut loader.read_lump(index + sectors::OFFSET)?)?;
	
	let index = loader.index_for_name(&("GL_".to_owned() + name)).unwrap();
	let gl_vert = gl_vert::from_data(&mut loader.read_lump(index + gl_vert::OFFSET)?)?;
	let gl_segs = gl_segs::from_data(&mut loader.read_lump(index + gl_segs::OFFSET)?)?;
	let gl_ssect = gl_ssect::from_data(&mut loader.read_lump(index + gl_ssect::OFFSET)?)?;
	let gl_nodes = gl_nodes::from_data(&mut loader.read_lump(index + gl_nodes::OFFSET)?)?;
	
	// Process all subsectors, add geometry for each seg
	let mut texture_loader = DoomTextureLoader::new(loader)?;
	let mut flat_loader = DoomFlatLoader::new(loader)?;
	
	let mut vertices = Vec::new();
	let mut faces = Vec::new();
	let mut leaves = Vec::new();
	
	for ssect in gl_ssect {
		let mut leaf = BSPLeaf {
			first_face_index: faces.len(),
			face_count: 0,
			bounding_box: BoundingBox3::zero(),  // Temporary dummy value
		};
		
		let segs = &gl_segs[ssect.first_seg_index .. ssect.first_seg_index + ssect.seg_count];
		let mut sector = None;
		
		// Walls
		for seg in segs.iter() {
			if let Some(linedef_index) = seg.linedef_index {
				let linedef = &linedefs[linedef_index];
				
				if let Some(front_sidedef_index) = linedef.sidedef_indices[seg.side as usize] {
					let front_sidedef = &sidedefs[front_sidedef_index];
					
					// Assign sector
					if let Some(s) = sector {
						if s as *const _ != &sectors[front_sidedef.sector_index] as *const _ {
							return Err(Box::from("Not all the segs belong to the same sector!"));
						}
					} else {
						sector = Some(&sectors[front_sidedef.sector_index]);
					}
					
					let front_sector = sector.unwrap();
					
					// Add wall
					let start_vertex = if seg.start_vertex_index.1 {
						&gl_vert[seg.start_vertex_index.0]
					} else {
						&vertexes[seg.start_vertex_index.0]
					};
					
					let end_vertex = if seg.end_vertex_index.1 {
						&gl_vert[seg.end_vertex_index.0]
					} else {
						&vertexes[seg.end_vertex_index.0]
					};
					
					let diff = end_vertex - start_vertex;
					let width = nalgebra::norm(&diff);
					
					// Two-sided or one-sided sidedef?
					if let Some(back_sidedef_index) = linedef.sidedef_indices[!seg.side as usize] {
						let back_sidedef = &sidedefs[back_sidedef_index];
						let back_sector = &sectors[back_sidedef.sector_index];
						
						let top_span = (front_sector.ceiling_height.min(back_sector.ceiling_height), front_sector.ceiling_height);
						let bottom_span = (front_sector.floor_height, back_sector.floor_height.max(front_sector.floor_height));
						let middle_span = (front_sector.floor_height.max(back_sector.floor_height), front_sector.ceiling_height.min(back_sector.ceiling_height));
						
						let total_height = top_span.1 - bottom_span.0;
						let top_height = top_span.1 - top_span.0;
						let bottom_height = bottom_span.1 - bottom_span.0;
						let middle_height = middle_span.1 - middle_span.0;
						
						// Top section
						if let Some(texture_name) = &front_sidedef.top_texture_name {
							faces.push(Face {
								first_vertex_index: vertices.len(),
								vertex_count: 4,
								texture: texture_loader.load(texture_name, loader)?,
							});
							leaf.face_count += 1;
							
							vertices.push(VertexData {
								in_position: [start_vertex[0], start_vertex[1], top_span.0],
								in_tex_coord: [0.0, if linedef.flags & 8 != 0 { top_height } else { 0.0 }],
							});
							vertices.push(VertexData {
								in_position: [end_vertex[0], end_vertex[1], top_span.0],
								in_tex_coord: [width, if linedef.flags & 8 != 0 { top_height } else { 0.0 }],
							});
							vertices.push(VertexData {
								in_position: [end_vertex[0], end_vertex[1], top_span.1],
								in_tex_coord: [width, if linedef.flags & 8 != 0 { 0.0 } else { -top_height }],
							});
							vertices.push(VertexData {
								in_position: [start_vertex[0], start_vertex[1], top_span.1],
								in_tex_coord: [0.0, if linedef.flags & 8 != 0 { 0.0 } else { -top_height }],
							});
						}
						
						// Bottom section
						if let Some(texture_name) = &front_sidedef.bottom_texture_name {
							faces.push(Face {
								first_vertex_index: vertices.len(),
								vertex_count: 4,
								texture: texture_loader.load(texture_name, loader)?,
							});
							leaf.face_count += 1;
							
							vertices.push(VertexData {
								in_position: [start_vertex[0], start_vertex[1], bottom_span.0],
								in_tex_coord: [0.0, if linedef.flags & 16 != 0 { total_height } else { bottom_height }],
							});
							vertices.push(VertexData {
								in_position: [end_vertex[0], end_vertex[1], bottom_span.0],
								in_tex_coord: [width, if linedef.flags & 16 != 0 { total_height } else { bottom_height }],
							});
							vertices.push(VertexData {
								in_position: [end_vertex[0], end_vertex[1], bottom_span.1],
								in_tex_coord: [width, if linedef.flags & 16 != 0 { total_height - bottom_height } else { 0.0 }],
							});
							vertices.push(VertexData {
								in_position: [start_vertex[0], start_vertex[1], bottom_span.1],
								in_tex_coord: [0.0, if linedef.flags & 16 != 0 { total_height - bottom_height } else { 0.0 }],
							});
						}
						
						// Middle section
						if let Some(texture_name) = &front_sidedef.middle_texture_name {
							faces.push(Face {
								first_vertex_index: vertices.len(),
								vertex_count: 4,
								texture: texture_loader.load(texture_name, loader)?,
							});
							leaf.face_count += 1;
							
							vertices.push(VertexData {
								in_position: [start_vertex[0], start_vertex[1], middle_span.0],
								in_tex_coord: [0.0, if linedef.flags & 16 != 0 { 0.0 } else { middle_height }],
							});
							vertices.push(VertexData {
								in_position: [end_vertex[0], end_vertex[1], middle_span.0],
								in_tex_coord: [width, if linedef.flags & 16 != 0 { 0.0 } else { middle_height }],
							});
							vertices.push(VertexData {
								in_position: [end_vertex[0], end_vertex[1], middle_span.1],
								in_tex_coord: [width, if linedef.flags & 16 != 0 { -middle_height } else { 0.0 }],
							});
							vertices.push(VertexData {
								in_position: [start_vertex[0], start_vertex[1], middle_span.1],
								in_tex_coord: [0.0, if linedef.flags & 16 != 0 { -middle_height } else { 0.0 }],
							});
						}
					} else {
						if let Some(texture_name) = &front_sidedef.middle_texture_name {
							let total_height = front_sector.ceiling_height - front_sector.floor_height;
							
							faces.push(Face {
								first_vertex_index: vertices.len(),
								vertex_count: 4,
								texture: texture_loader.load(texture_name, loader)?,
							});
							leaf.face_count += 1;
							
							vertices.push(VertexData {
								in_position: [start_vertex[0], start_vertex[1], front_sector.floor_height],
								in_tex_coord: [0.0, if linedef.flags & 16 != 0 { 0.0 } else { total_height }],
							});
							vertices.push(VertexData {
								in_position: [end_vertex[0], end_vertex[1], front_sector.floor_height],
								in_tex_coord: [width, if linedef.flags & 16 != 0 { 0.0 } else { total_height }],
							});
							vertices.push(VertexData {
								in_position: [end_vertex[0], end_vertex[1], front_sector.ceiling_height],
								in_tex_coord: [width, if linedef.flags & 16 != 0 { -total_height } else { 0.0 }],
							});
							vertices.push(VertexData {
								in_position: [start_vertex[0], start_vertex[1], front_sector.ceiling_height],
								in_tex_coord: [0.0, if linedef.flags & 16 != 0 { -total_height } else { 0.0 }],
							});
						}
					}
				}
			}
		}
		
		// Floor
		faces.push(Face {
			first_vertex_index: vertices.len(),
			vertex_count: segs.len(),
			texture: flat_loader.load(&sector.unwrap().floor_flat_name, loader)?,
		});
		leaf.face_count += 1;
		
		for seg in segs.iter().rev() {
			let start_vertex = if seg.start_vertex_index.1 {
				gl_vert[seg.start_vertex_index.0]
			} else {
				vertexes[seg.start_vertex_index.0]
			};
			
			vertices.push(VertexData {
				in_position: [start_vertex[0], start_vertex[1], sector.unwrap().floor_height],
				in_tex_coord: [start_vertex[0], start_vertex[1]]
			});
		}
		
		// Ceiling
		faces.push(Face {
			first_vertex_index: vertices.len(),
			vertex_count: segs.len(),
			texture: flat_loader.load(&sector.unwrap().ceiling_flat_name, loader)?,
		});
		leaf.face_count += 1;
		
		for seg in segs.iter() {
			let start_vertex = if seg.start_vertex_index.1 {
				gl_vert[seg.start_vertex_index.0]
			} else {
				vertexes[seg.start_vertex_index.0]
			};
			
			vertices.push(VertexData {
				in_position: [start_vertex[0], start_vertex[1], sector.unwrap().ceiling_height],
				in_tex_coord: [start_vertex[0], start_vertex[1]]
			});
		}
		
		// Add the BSP leaf
		leaves.push(leaf);
	}
	
	// Process nodes
	let mut branches = Vec::new();
	
	for node in &gl_nodes {
		// Convert the Doom line definition into plane
		let normal = Vector2::new(
			node.partition_dir[1],  // 90 degree clockwise rotation
			-node.partition_dir[0],
		).normalize();
		
		branches.push(BSPBranch {
			plane: Plane {
				normal: Vector3::new(normal[0], normal[1], 0.0),
				distance: nalgebra::dot(&normal, &node.partition_point),
			},
			bounding_box: BoundingBox3::zero(),  // Temporary dummy value
			children: [node.right_child_index, node.left_child_index],
		});
	}
	
	// Set the bounding box values.
	// Doom stores bounding boxes in the parent node,
	// but BSPModel wants them in the current node.
	for node in &gl_nodes {
		match node.right_child_index {
			BSPNode::Leaf(index) => {
				leaves[index].bounding_box = BoundingBox3::from(&node.right_bbox);
			},
			BSPNode::Branch(index) => {
				branches[index].bounding_box = BoundingBox3::from(&node.right_bbox);
			},
		}
		
		match node.left_child_index {
			BSPNode::Leaf(index) => {
				leaves[index].bounding_box = BoundingBox3::from(&node.left_bbox);
			},
			BSPNode::Branch(index) => {
				branches[index].bounding_box = BoundingBox3::from(&node.left_bbox);
			},
		}
	}
	
	Ok(BSPModel::new(vertices, faces, leaves, branches))
}

pub mod things {
	use super::*;
	pub const OFFSET: usize = 1;
	
	#[derive(Debug)]
	pub struct DoomMapThing {
		pub position: Vector2<f32>,
		pub angle: i16,
		pub type_id: u16,
		pub flags: u16,
	}
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<DoomMapThing>, io::Error> {
		let mut things = Vec::new();
		
		loop {
			let position_x = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break
					} else {
						return Err(err)
					}
				}
			} as f32;
			let position_y = data.read_i16::<LE>()? as f32;
			let angle = data.read_i16::<LE>()?;
			let type_id = data.read_u16::<LE>()?;
			let flags = data.read_u16::<LE>()?;
			
			things.push(DoomMapThing{
				position: Vector2::new(position_x, position_y),
				angle,
				type_id,
				flags,
			});
		}
		
		Ok(things)
	}
}

pub mod linedefs {
	use super::*;
	pub const OFFSET: usize = 2;
	
	#[derive(Debug)]
	pub struct DoomMapLinedef {
		pub start_vertex_index: usize,
		pub end_vertex_index: usize,
		pub flags: u16,
		pub special_type: u16,
		pub sector_tag: u16,
		pub sidedef_indices: [Option<usize>; 2],
	}
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<DoomMapLinedef>, io::Error> {
		let mut linedefs = Vec::new();
		
		loop {
			let start_vertex_index = match data.read_u16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break
					} else {
						return Err(err)
					}
				}
			} as usize;
			let end_vertex_index = data.read_u16::<LE>()? as usize;
			let flags = data.read_u16::<LE>()?;
			let special_type = data.read_u16::<LE>()?;
			let sector_tag = data.read_u16::<LE>()?;
			let right_sidedef_index = data.read_u16::<LE>()? as usize;
			let left_sidedef_index = data.read_u16::<LE>()? as usize;
			
			linedefs.push(DoomMapLinedef{
				start_vertex_index,
				end_vertex_index,
				flags,
				special_type,
				sector_tag,
				sidedef_indices: [
					if right_sidedef_index == 0xFFFF {
						None
					} else {
						Some(right_sidedef_index)
					},
					if left_sidedef_index == 0xFFFF {
						None
					} else {
						Some(left_sidedef_index)
					}
				],
			});
		}
		
		Ok(linedefs)
	}
}

pub mod sidedefs {
	use super::*;
	pub const OFFSET: usize = 3;
	
	#[derive(Debug)]
	pub struct DoomMapSidedef {
		pub texture_offset: Vector2<i16>,
		pub top_texture_name: Option<String>,
		pub bottom_texture_name: Option<String>,
		pub middle_texture_name: Option<String>,
		pub sector_index: usize,
	}
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<DoomMapSidedef>, Box<Error>> {
		let mut sidedefs = Vec::new();
		
		loop {
			let texture_offset_x = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break
					} else {
						return Err(Box::from(err))
					}
				}
			};
			let texture_offset_y = data.read_i16::<LE>()?;
			let top_texture_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_right_matches('\0'))
			};
			let bottom_texture_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_right_matches('\0'))
			};
			let middle_texture_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_right_matches('\0'))
			};
			let sector_index = data.read_u16::<LE>()? as usize;
			
			sidedefs.push(DoomMapSidedef{
				texture_offset: Vector2::new(texture_offset_x, texture_offset_y),
				top_texture_name: if top_texture_name == "-" { None } else { Some(top_texture_name) },
				bottom_texture_name: if bottom_texture_name == "-" { None } else { Some(bottom_texture_name) },
				middle_texture_name: if middle_texture_name == "-" { None } else { Some(middle_texture_name) },
				sector_index,
			});
		}
		
		Ok(sidedefs)
	}
}

pub mod vertexes {
	use super::*;
	pub const OFFSET: usize = 4;
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<Vector2<f32>>, io::Error> {
		let mut vertexes = Vec::new();
		
		loop {
			let x = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break
					} else {
						return Err(err)
					}
				}
			} as f32;
			let y = data.read_i16::<LE>()? as f32;
			
			vertexes.push(Vector2::new(x, y));
		}
		
		Ok(vertexes)
	}
}

pub mod sectors {
	use super::*;
	pub const OFFSET: usize = 8;
	
	#[derive(Debug)]
	pub struct DoomMapSector {
		pub floor_height: f32,
		pub ceiling_height: f32,
		pub floor_flat_name: String,
		pub ceiling_flat_name: String,
		pub light_level: u16,
		pub special_type: u16,
		pub sector_tag: u16,
	}
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<DoomMapSector>, Box<Error>> {
		let mut sectors = Vec::new();
		
		loop {
			let floor_height = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break
					} else {
						return Err(Box::from(err))
					}
				}
			} as f32;
			let ceiling_height = data.read_i16::<LE>()? as f32;
			let floor_flat_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_right_matches('\0'))
			};
			let ceiling_flat_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_right_matches('\0'))
			};
			let light_level = data.read_u16::<LE>()?;
			let special_type = data.read_u16::<LE>()?;
			let sector_tag = data.read_u16::<LE>()?;
			
			sectors.push(DoomMapSector{
				floor_height,
				ceiling_height,
				floor_flat_name,
				ceiling_flat_name,
				light_level,
				special_type,
				sector_tag,
			});
		}
		
		Ok(sectors)
	}
}

pub mod gl_vert {
	use super::*;
	pub const OFFSET: usize = 1;
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<Vector2<f32>>, Box<Error>> {
		let mut gl_vert = Vec::new();
		
		let mut signature = [0u8; 4];
		data.read_exact(&mut signature)?;
		
		if &signature != b"gNd2" {
			return Err(Box::from("No gNd2 signature found"))
		}
		
		loop {
			let x = match data.read_i32::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break
					} else {
						return Err(Box::from(err))
					}
				}
			} as f32;
			let y = data.read_i32::<LE>()? as f32;
			
			gl_vert.push(Vector2::new(x / 65536.0, y / 65536.0));
		}
		
		Ok(gl_vert)
	}
}

pub mod gl_segs {
	use super::*;
	pub const OFFSET: usize = 2;
	
	#[derive(Debug)]
	pub struct DoomMapGLSegs {
		pub start_vertex_index: (usize, bool),
		pub end_vertex_index: (usize, bool),
		pub linedef_index: Option<usize>,
		pub side: bool,
		pub partner_seg_index: Option<usize>,
	}
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<DoomMapGLSegs>, io::Error> {
		let mut gl_segs = Vec::new();
		
		loop {
			let start_vertex_index = match data.read_u16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break
					} else {
						return Err(err)
					}
				}
			} as usize;
			let end_vertex_index = data.read_u16::<LE>()? as usize;
			let linedef_index = data.read_u16::<LE>()? as usize;
			let side = data.read_u16::<LE>()? != 0;
			let partner_seg_index = data.read_u16::<LE>()? as usize;
			
			gl_segs.push(DoomMapGLSegs {
				start_vertex_index: {
					if (start_vertex_index & 0x8000) != 0 {
						(start_vertex_index & 0x7FFF, true)
					} else {
						(start_vertex_index, false)
					}
				},
				end_vertex_index: {
					if (end_vertex_index & 0x8000) != 0 {
						(end_vertex_index & 0x7FFF, true)
					} else {
						(end_vertex_index, false)
					}
				},
				linedef_index: {
					if linedef_index == 0xFFFF {
						None
					} else {
						Some(linedef_index)
					}
				},
				side,
				partner_seg_index: {
					if partner_seg_index == 0xFFFF {
						None
					} else {
						Some(partner_seg_index)
					}
				},
			});
		}
		
		Ok(gl_segs)
	}
}

pub mod gl_ssect {
	use super::*;
	pub const OFFSET: usize = 3;
	
	#[derive(Debug)]
	pub struct DoomMapGLSSect {
		pub seg_count: usize,
		pub first_seg_index: usize,
	}
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<DoomMapGLSSect>, io::Error> {
		let mut gl_ssect = Vec::new();
		
		loop {
			let seg_count = match data.read_u16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break
					} else {
						return Err(err)
					}
				}
			} as usize;
			let first_seg_index = data.read_u16::<LE>()? as usize;
			
			gl_ssect.push(DoomMapGLSSect {
				seg_count,
				first_seg_index,
			});
		}
		
		Ok(gl_ssect)
	}
}

pub mod gl_nodes {
	use super::*;
	pub const OFFSET: usize = 4;
	
	#[derive(Debug)]
	pub struct DoomMapGLNodes {
		pub partition_point: Vector2<f32>,
		pub partition_dir: Vector2<f32>,
		pub right_bbox: BoundingBox2,
		pub left_bbox: BoundingBox2,
		pub right_child_index: BSPNode,
		pub left_child_index: BSPNode,
	}
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<DoomMapGLNodes>, io::Error> {
		let mut gl_nodes = Vec::new();
		
		loop {
			let partition_point_x = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break
					} else {
						return Err(err)
					}
				}
			} as f32;
			let partition_point_y = data.read_i16::<LE>()? as f32;
			let partition_dir_x = data.read_i16::<LE>()? as f32;
			let partition_dir_y = data.read_i16::<LE>()? as f32;
			let right_bbox_top = data.read_i16::<LE>()? as f32;
			let right_bbox_bottom = data.read_i16::<LE>()? as f32;
			let right_bbox_left = data.read_i16::<LE>()? as f32;
			let right_bbox_right = data.read_i16::<LE>()? as f32;
			let left_bbox_top = data.read_i16::<LE>()? as f32;
			let left_bbox_bottom = data.read_i16::<LE>()? as f32;
			let left_bbox_left = data.read_i16::<LE>()? as f32;
			let left_bbox_right = data.read_i16::<LE>()? as f32;
			let right_child_index = data.read_u16::<LE>()? as usize;
			let left_child_index = data.read_u16::<LE>()? as usize;
			
			gl_nodes.push(DoomMapGLNodes {
				partition_point: Vector2::new(partition_point_x, partition_point_y),
				partition_dir: Vector2::new(partition_dir_x, partition_dir_y),
				right_bbox: BoundingBox2::from_extents(
					right_bbox_top,
					right_bbox_bottom,
					right_bbox_left,
					right_bbox_right,
				),
				left_bbox: BoundingBox2::from_extents(
					left_bbox_top,
					left_bbox_bottom,
					left_bbox_left,
					left_bbox_right,
				),
				right_child_index: {
					if (right_child_index & 0x8000) != 0 {
						BSPNode::Leaf(right_child_index & 0x7FFF)
					} else {
						BSPNode::Branch(right_child_index)
					}
				},
				left_child_index: {
					if (left_child_index & 0x8000) != 0 {
						BSPNode::Leaf(left_child_index & 0x7FFF)
					} else {
						BSPNode::Branch(left_child_index)
					}
				},
			});
		}
		
		Ok(gl_nodes)
	}
}
