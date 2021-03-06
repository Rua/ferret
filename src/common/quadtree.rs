use crate::common::geometry::{Interval, Line2, AABB2};
use fnv::FnvHashMap;
use legion::Entity;
use nalgebra::Vector2;

#[derive(Clone, Debug)]
pub struct Quadtree {
	nodes: Vec<QuadtreeNode>,
	unused_nodes: Vec<usize>,
	bboxes: FnvHashMap<Entity, AABB2>,
}

impl Quadtree {
	const NODE_MIN: usize = 5;
	const NODE_MAX: usize = 10;

	#[inline]
	pub fn new(bbox: AABB2) -> Quadtree {
		assert!(!bbox.is_empty());
		Quadtree {
			nodes: vec![QuadtreeNode::new(bbox)],
			unused_nodes: Vec::new(),
			bboxes: FnvHashMap::default(),
		}
	}

	#[inline]
	fn allocate_node(&mut self, bbox: AABB2) -> usize {
		match self.unused_nodes.pop() {
			Some(index) => {
				self.nodes[index].clear(bbox);
				index
			}
			None => {
				self.nodes.push(QuadtreeNode::new(bbox));
				self.nodes.len() - 1
			}
		}
	}

	#[inline]
	fn child_index(&self, index: usize, bbox: &AABB2) -> Option<usize> {
		if let Some((middle, child_nodes)) = self.nodes[index].children {
			let offset = bbox.offset(-middle);
			let crosses_split = [
				offset[0].min <= 0.0 && offset[0].max >= 0.0,
				offset[1].min <= 0.0 && offset[1].max >= 0.0,
			];

			if crosses_split[0] || crosses_split[1] {
				// Bounding box lies on the split
				None
			} else {
				// Bounding box lies in one of the quadrants
				let sides = [
					(offset[0].min > 0.0) as usize,
					(offset[1].min > 0.0) as usize,
				];
				Some(child_nodes[sides[0]][sides[1]])
			}
		} else {
			None
		}
	}

	fn check_split(&mut self, index: usize) {
		if self.nodes[index].num_descendants <= Self::NODE_MAX {
			return;
		}

		if self.nodes[index].children.is_none() {
			// Create child nodes
			let middle = self.nodes[index].bbox.middle();
			let child_nodes = [
				[
					self.allocate_node(AABB2::from_minmax(self.nodes[index].bbox.min(), middle)),
					self.allocate_node(AABB2::from_intervals(Vector2::new(
						Interval::new(self.nodes[index].bbox[0].min, middle[0]),
						Interval::new(middle[1], self.nodes[index].bbox[1].max),
					))),
				],
				[
					self.allocate_node(AABB2::from_intervals(Vector2::new(
						Interval::new(middle[0], self.nodes[index].bbox[0].max),
						Interval::new(self.nodes[index].bbox[1].min, middle[1]),
					))),
					self.allocate_node(AABB2::from_minmax(middle, self.nodes[index].bbox.max())),
				],
			];
			self.nodes[index].children = Some((middle, child_nodes));

			// Re-insert the entities
			let mut i = 0;
			while i != self.nodes[index].entities.len() {
				let bbox = &self.bboxes[&self.nodes[index].entities[i]];
				if let Some(child) = self.child_index(index, bbox) {
					// Entity goes in child node
					let data = self.nodes[index].entities.swap_remove(i);
					self.nodes[child].entities.push(data);
					self.nodes[child].num_descendants += 1;
				} else {
					// Entity stays in current node
					i += 1;
				}
			}

			// Check if any of the children need splitting
			for &(x, y) in &[(0, 0), (0, 1), (1, 0), (1, 1)] {
				self.check_split(child_nodes[x][y]);
			}
		}
	}

	fn collapse(&mut self, index: usize) {
		if let Some((_, child_nodes)) = self.nodes[index].children {
			let mut entities = std::mem::replace(&mut self.nodes[index].entities, Vec::new());
			for &(x, y) in &[(0, 0), (0, 1), (1, 0), (1, 1)] {
				// Move the entities to the parent
				self.collapse_r(&mut entities, child_nodes[x][y]);
			}

			self.nodes[index].children = None;
			self.nodes[index].entities = entities;
		}
	}

	fn collapse_r(&mut self, dest: &mut Vec<Entity>, index: usize) {
		let QuadtreeNode {
			entities,
			num_descendants: _,
			bbox: _,
			children,
		} = std::mem::replace(&mut self.nodes[index], QuadtreeNode::empty());
		self.unused_nodes.push(index);

		dest.extend(entities);

		if let Some((_, child_nodes)) = children {
			for &(x, y) in &[(0, 0), (0, 1), (1, 0), (1, 1)] {
				// Move the entities to the parent
				self.collapse_r(dest, child_nodes[x][y]);
			}
		}
	}

	#[inline]
	pub fn insert(&mut self, entity: Entity, bbox: &AABB2) {
		assert!(!bbox.is_empty());

		if !self.bboxes.contains_key(&entity) {
			self.bboxes.insert(entity, bbox.clone());
			self.insert_r(0, entity, &bbox);
		}
	}

	fn insert_r(&mut self, index: usize, entity: Entity, bbox: &AABB2) {
		self.nodes[index].num_descendants += 1;

		if let Some(child) = self.child_index(index, bbox) {
			self.insert_r(child, entity, bbox);
		} else {
			self.nodes[index].entities.push(entity);
		}

		self.check_split(index);
	}

	#[inline]
	pub fn remove(&mut self, entity: Entity) {
		if let Some(bbox) = self.bboxes.remove(&entity) {
			self.remove_r(0, entity, &bbox);
		}
	}

	fn remove_r(&mut self, index: usize, entity: Entity, bbox: &AABB2) {
		self.nodes[index].num_descendants -= 1;

		if self.nodes[index].num_descendants < Self::NODE_MIN {
			self.collapse(index);
		}

		if let Some(child) = self.child_index(index, bbox) {
			self.remove_r(child, entity, bbox);
		} else {
			let pos = self.nodes[index]
				.entities
				.iter()
				.position(|x| *x == entity)
				.unwrap();
			self.nodes[index].entities.swap_remove(pos);
		}
	}

	#[inline]
	pub fn traverse_nodes<F>(&self, bbox: AABB2, move_step: Line2, func: &mut F)
	where
		F: FnMut(&[Entity]) -> Vector2<f32>,
	{
		self.traverse_nodes_r(0, &bbox, move_step, func);
	}

	fn traverse_nodes_r<F>(
		&self,
		index: usize,
		bbox: &AABB2,
		mut move_step: Line2,
		func: &mut F,
	) -> Vector2<f32>
	where
		F: FnMut(&[Entity]) -> Vector2<f32>,
	{
		if !self.nodes[index].entities.is_empty() {
			move_step.dir = func(&self.nodes[index].entities);
		}

		if let Some((middle, child_nodes)) = self.nodes[index].children {
			let start_interval = bbox.offset(move_step.point);

			// Start with the side that the start point is on
			let point_side = middle.zip_map(&move_step.point, |middle, point| point >= middle);
			let sides = [
				Vector2::new(point_side[0], point_side[1]),
				Vector2::new(point_side[0], !point_side[1]),
				Vector2::new(!point_side[0], point_side[1]),
				Vector2::new(!point_side[0], !point_side[1]),
			];

			for side in sides.iter() {
				let test = start_interval
					.extend(move_step.dir)
					.direction_from(middle)
					.zip_map(&side, |direction, side| match side {
						true => direction >= 0.0,
						false => direction <= 0.0,
					});

				if test[0] && test[1] {
					move_step.dir = self.traverse_nodes_r(
						child_nodes[side[0] as usize][side[1] as usize],
						bbox,
						move_step,
						func,
					);
				}
			}
		}

		move_step.dir
	}
}

#[derive(Clone, Debug)]
struct QuadtreeNode {
	entities: Vec<Entity>,
	num_descendants: usize,
	bbox: AABB2,
	children: Option<(Vector2<f32>, [[usize; 2]; 2])>,
}

impl QuadtreeNode {
	#[inline]
	fn new(bbox: AABB2) -> QuadtreeNode {
		QuadtreeNode {
			entities: Vec::new(),
			num_descendants: 0,
			bbox,
			children: None,
		}
	}

	#[inline]
	fn empty() -> QuadtreeNode {
		QuadtreeNode::new(AABB2::empty())
	}

	#[inline]
	fn clear(&mut self, bbox: AABB2) {
		self.entities.clear();
		self.num_descendants = 0;
		self.bbox = bbox;
		self.children = None;
	}
}
