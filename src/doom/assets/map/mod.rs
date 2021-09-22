pub mod load;
pub mod meshes;
pub mod textures;

use crate::{
	common::{
		assets::AssetHandle,
		geometry::{Angle, Interval, Line2, Line3, Plane2, Side, AABB2},
	},
	doom::{
		assets::{
			image::Image,
			map::{load::LinedefFlags, textures::TextureType},
		},
		game::{map::MapDynamic, physics::SolidBits, trace::CollisionPlane},
	},
};
use bitflags::bitflags;
use fnv::FnvHashMap;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, time::Duration};

#[derive(Debug)]
pub struct Map {
	pub name: String,
	pub anims: FnvHashMap<AssetHandle<Image>, Anim>,
	pub bbox: AABB2,
	pub sky: AssetHandle<Image>,
	pub switches: FnvHashMap<AssetHandle<Image>, AssetHandle<Image>>,
	pub exit: Option<String>,
	pub secret_exit: Option<String>,

	pub linedefs: Vec<Linedef>,
	pub nodes: Vec<Node>,
	pub sectors: Vec<Sector>,
	pub subsectors: Vec<Subsector>,
}

#[derive(Clone, Debug)]
pub struct Anim {
	pub frames: Vec<AssetHandle<Image>>,
	pub frame_time: Duration,
}

#[derive(Clone, Copy, Debug)]
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
	pub special_type: u16,
	pub sector_tag: u16,
	pub sidedefs: [Option<Sidedef>; 2],
}

#[derive(Clone, Debug)]
pub struct Sidedef {
	pub texture_offset: Vector2<f32>,
	pub textures: [TextureType; 3],
	pub sector_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SidedefSlot {
	Top = 0,
	Bottom = 1,
	Middle = 2,
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
	pub special_type: u16,
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

	pub fn traverse_nodes<F>(&self, bbox: AABB2, move_step: Line2, mut func: F)
	where
		F: FnMut(NodeChild) -> Vector2<f32>,
	{
		self.traverse_nodes_r(NodeChild::Node(0), &bbox, move_step, &mut func);
	}

	pub fn traverse_nodes_r<F>(
		&self,
		node: NodeChild,
		bbox: &AABB2,
		mut move_step: Line2,
		func: &mut F,
	) -> Vector2<f32>
	where
		F: FnMut(NodeChild) -> Vector2<f32>,
	{
		move_step.dir = func(node);

		if let NodeChild::Node(index) = node {
			let node = &self.nodes[index];

			// Calculate the bounding box's min and max distances from the plane
			let start_interval = if bbox.is_point() {
				Interval::from_point(bbox.min().dot(&node.plane.normal))
			} else {
				std::array::IntoIter::new([
					Vector2::new(bbox[0].min, bbox[1].min),
					Vector2::new(bbox[0].min, bbox[1].max),
					Vector2::new(bbox[0].max, bbox[1].min),
					Vector2::new(bbox[0].max, bbox[1].max),
				])
				.fold(Interval::empty(), |i, p| {
					i.add_point(p.dot(&node.plane.normal))
				})
			}
			.offset(move_step.point.dot(&node.plane.normal));

			// Start with the side that the start point is on
			let point_side = move_step.point.dot(&node.plane.normal) < 0.0;
			let sides = [point_side, !point_side];

			for &side in sides.iter() {
				let direction = start_interval
					.extend(move_step.dir.dot(&node.plane.normal))
					.direction_from(node.plane.distance);

				let test = match side {
					false => direction >= 0.0,
					true => direction <= 0.0,
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

	pub fn visible_interval(
		&self,
		map_dynamic: &MapDynamic,
		move_step: Line3,
		height: f32,
	) -> Interval {
		let move_step2 = Line2::from(move_step);
		let mut ret = Interval::new(move_step.dir[2], move_step.dir[2] + height);

		self.traverse_nodes(
			AABB2::from_point(Vector2::zeros()),
			move_step2,
			|node: NodeChild| -> Vector2<f32> {
				let linedefs = match node {
					NodeChild::Subsector(index) => &self.subsectors[index].linedefs,
					NodeChild::Node(index) => &self.nodes[index].linedefs,
				};

				for linedef_index in linedefs.iter().copied() {
					let linedef = &self.linedefs[linedef_index];

					if !move_step2.intersects(&linedef.line) {
						continue;
					}

					if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
						let front_interval =
							&map_dynamic.sectors[front_sidedef.sector_index].interval;
						let back_interval =
							&map_dynamic.sectors[back_sidedef.sector_index].interval;
						let intersection = front_interval
							.intersection(*back_interval)
							.offset(-move_step.point[2]);

						if intersection.is_empty() {
							// Walls fully block sight
							ret = Interval::empty();
							return Vector2::zeros();
						}

						let fraction = {
							let denom = move_step2.dir.dot(&linedef.normal);

							if denom == 0.0 {
								0.0
							} else {
								(linedef.line.point - move_step2.point).dot(&linedef.normal) / denom
							}
						};

						assert!(fraction >= 0.0 && fraction <= 1.0);

						// Scale by fraction because nearer objects take up more vertical space
						ret = ret.intersection(Interval::new(
							intersection.min / fraction,
							intersection.max / fraction,
						));

						if ret.is_empty() {
							// Gap does not overlap
							return Vector2::zeros();
						}
					} else if let [Some(_), None] = &linedef.sidedefs {
						// Can't see through a onesided linedef
						ret = Interval::empty();
						return Vector2::zeros();
					}
				}

				move_step2.dir
			},
		);

		ret
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
