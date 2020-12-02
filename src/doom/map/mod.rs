pub mod load;
pub mod meshes;
pub mod textures;

use crate::{
	common::{
		assets::AssetHandle,
		geometry::{Angle, Interval, Line2, Line3, Plane2, Side, AABB2, AABB3},
		time::Timer,
	},
	doom::{
		image::Image,
		map::{load::LinedefFlags, textures::TextureType},
		physics::SolidBits,
		trace::CollisionPlane,
	},
};
use bitflags::bitflags;
use fnv::FnvHashMap;
use legion::Entity;
use nalgebra::{Vector2, Vector3};
use serde::Deserialize;
use std::{fmt::Debug, time::Duration};

#[derive(Debug)]
pub struct Map {
	pub anims: FnvHashMap<AssetHandle<Image>, Anim>,
	pub bbox: AABB2,
	pub linedefs: Vec<Linedef>,
	pub nodes: Vec<Node>,
	pub sectors: Vec<Sector>,
	pub subsectors: Vec<Subsector>,
	pub sky: AssetHandle<Image>,
	pub switches: FnvHashMap<AssetHandle<Image>, AssetHandle<Image>>,
}

#[derive(Clone, Debug)]
pub struct MapDynamic {
	pub anim_states: FnvHashMap<AssetHandle<Image>, AnimState>,
	pub map: AssetHandle<Map>,
	pub linedefs: Vec<LinedefDynamic>,
	pub sectors: Vec<SectorDynamic>,
}

#[derive(Clone, Debug)]
pub struct Anim {
	pub frames: Vec<AssetHandle<Image>>,
	pub frame_time: Duration,
}

#[derive(Clone, Copy, Debug)]
pub struct AnimState {
	pub frame: usize,
	pub timer: Timer,
}

pub struct Thing {
	pub position: Vector2<f32>,
	pub angle: Angle,
	pub r#type: u16,
	pub flags: ThingFlags,
}

bitflags! {
	#[derive(Deserialize)]
	pub struct ThingFlags: u16 {
		const EASY = 0b00000000_00000001;
		const NORMAL = 0b00000000_00000010;
		const HARD = 0b00000000_00000100;
		const DEAF = 0b00000000_00001000;
		const DMONLY = 0b00000000_00010000;
	}
}

#[derive(Clone, Debug)]
pub struct Linedef {
	pub line: Line2,
	pub normal: Vector2<f32>,
	pub collision_planes: Vec<CollisionPlane>,
	pub bbox: AABB2,
	pub flags: LinedefFlags,
	pub blocks_types: SolidBits,
	pub special_type: Option<u16>,
	pub sector_tag: u16,
	pub sidedefs: [Option<Sidedef>; 2],
}

#[derive(Clone, Debug)]
pub struct LinedefDynamic {
	pub entity: Entity,
	pub sidedefs: [Option<SidedefDynamic>; 2],
	pub texture_offset: Vector2<f32>,
}

#[derive(Clone, Debug)]
pub struct Sidedef {
	pub texture_offset: Vector2<f32>,
	pub textures: [TextureType; 3],
	pub sector_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SidedefSlot {
	Top = 0,
	Bottom = 1,
	Middle = 2,
}

#[derive(Clone, Debug)]
pub struct SidedefDynamic {
	pub textures: [TextureType; 3],
}

#[derive(Clone, Debug)]
pub struct LinedefRef {
	pub map_entity: Entity,
	pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Seg {
	pub line: Line2,
	pub normal: Vector2<f32>,
	pub linedef: Option<(usize, Side)>,
}

#[derive(Clone, Debug)]
pub struct Subsector {
	pub segs: Vec<Seg>,
	pub bbox: AABB2,
	pub collision_planes: Vec<CollisionPlane>,
	pub linedefs: Vec<usize>,
	pub sector_index: usize,
}

#[derive(Clone, Debug)]
pub struct Node {
	pub plane: Plane2,
	pub linedefs: Vec<usize>,
	pub child_bboxes: [AABB2; 2],
	pub child_indices: [NodeChild; 2],
}

#[derive(Copy, Clone, Debug)]
pub enum NodeChild {
	Subsector(usize),
	Node(usize),
}

#[derive(Clone, Debug)]
pub struct Sector {
	pub interval: Interval,
	pub textures: [TextureType; 2],
	pub light_level: f32,
	pub special_type: Option<u16>,
	pub sector_tag: u16,
	pub linedefs: Vec<usize>,
	pub subsectors: Vec<usize>,
	pub neighbours: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SectorSlot {
	Floor = 0,
	Ceiling = 1,
}

#[derive(Clone, Debug)]
pub struct SectorDynamic {
	pub entity: Entity,
	pub light_level: f32,
	pub interval: Interval,
}

#[derive(Clone, Copy, Debug)]
pub struct SectorRef {
	pub map_entity: Entity,
	pub index: usize,
}

impl Map {
	pub fn find_subsector(&self, point: Vector2<f32>) -> &Subsector {
		let mut child = NodeChild::Node(0);

		loop {
			child = match child {
				NodeChild::Subsector(index) => return &self.subsectors[index],
				NodeChild::Node(index) => {
					let node = &self.nodes[index];
					let dot = point.dot(&node.plane.normal) - node.plane.distance;
					node.child_indices[(dot <= 0.0) as usize]
				}
			};
		}
	}

	pub fn traverse_nodes<F>(&self, bbox: &AABB3, move_step: &Line3, mut func: F)
	where
		F: FnMut(NodeChild) -> Vector3<f32>,
	{
		self.traverse_nodes_r(
			NodeChild::Node(0),
			&bbox.into(),
			move_step.into(),
			&mut func,
		);
	}

	pub fn traverse_nodes_r<F>(
		&self,
		node: NodeChild,
		bbox: &AABB2,
		mut move_step: Line2,
		func: &mut F,
	) -> Vector2<f32>
	where
		F: FnMut(NodeChild) -> Vector3<f32>,
	{
		move_step.dir = func(node).fixed_resize(0.0);

		if let NodeChild::Node(index) = node {
			let node = &self.nodes[index];

			// Calculate the bounding box's min and max distances from the plane
			let start_interval = if bbox.is_point() {
				Interval::from_point(bbox.min().dot(&node.plane.normal))
			} else {
				let points = [
					Vector2::new(bbox[0].min, bbox[1].min),
					Vector2::new(bbox[0].min, bbox[1].max),
					Vector2::new(bbox[0].max, bbox[1].min),
					Vector2::new(bbox[0].max, bbox[1].max),
				];
				points.iter().copied().fold(Interval::empty(), |i, p| {
					i.add_point(p.dot(&node.plane.normal))
				})
			}
			.offset(move_step.point.dot(&node.plane.normal));

			// Start with the side that the start point is on
			let sides = if move_step.point.dot(&node.plane.normal) >= 0.0 {
				[Side::Right, Side::Left]
			} else {
				[Side::Left, Side::Right]
			};

			for &side in sides.iter() {
				let move_interval = start_interval.extend(move_step.dir.dot(&node.plane.normal));
				let direction = move_interval.direction_from(node.plane.distance);

				let test = match side {
					Side::Right => direction >= 0.0,
					Side::Left => direction <= 0.0,
				};

				if test {
					move_step.dir = self.traverse_nodes_r(
						node.child_indices[side as usize],
						bbox,
						move_step,
						func,
					);
				}
			}
		}

		move_step.dir
	}

	pub fn lowest_neighbour_floor(&self, map_dynamic: &MapDynamic, sector_index: usize) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.min)
			.min_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(self.sectors[sector_index].interval.min)
	}

	pub fn lowest_neighbour_floor_above(
		&self,
		map_dynamic: &MapDynamic,
		sector_index: usize,
		height: f32,
	) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.min)
			.filter(|h| *h > height)
			.min_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(self.sectors[sector_index].interval.min)
	}

	pub fn highest_neighbour_floor(&self, map_dynamic: &MapDynamic, sector_index: usize) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.min)
			.max_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(-500.0)
	}

	pub fn lowest_neighbour_ceiling(&self, map_dynamic: &MapDynamic, sector_index: usize) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.max)
			.min_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(32768.0)
	}

	/*pub fn highest_neighbour_ceiling(&self, map_dynamic: &MapDynamic, sector_index: usize) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.max)
			.max_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(0.0)
	}*/
}
