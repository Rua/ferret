use crate::geometry::{Interval, AABB2};
use nalgebra::Vector2;
use specs::Entity;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Quadtree {
	nodes: Vec<QuadtreeNode>,
	unused_nodes: Vec<usize>,
	bboxes: HashMap<Entity, AABB2>,
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
			bboxes: HashMap::new(),
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

	fn check_collapse(&mut self, index: usize) {
		if self.nodes[index].num_descendants >= Self::NODE_MIN {
			return;
		}

		if let Some((_, child_nodes)) = self.nodes[index].children {
			for &(x, y) in &[(0, 0), (0, 1), (1, 0), (1, 1)] {
				// Collapse the child nodes first, if needed
				self.check_collapse(child_nodes[x][y]);

				// Move the entities to the parent
				while let Some(entity) = self.nodes[child_nodes[x][y]].entities.pop() {
					self.nodes[index].entities.push(entity);
				}

				// Put the child node back in the pool
				self.unused_nodes.push(child_nodes[x][y]);
			}

			self.nodes[index].children = None;
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

		self.check_collapse(index);
	}

	#[inline]
	pub fn traverse_nodes<F: FnMut(&[Entity])>(&self, bbox: &AABB2, func: &mut F) {
		self.traverse_nodes_r(0, bbox, func);
	}

	fn traverse_nodes_r<F: FnMut(&[Entity])>(&self, index: usize, bbox: &AABB2, func: &mut F) {
		if !self.nodes[index].entities.is_empty() {
			func(&self.nodes[index].entities);
		}

		if let Some((middle, child_nodes)) = self.nodes[index].children {
			let offset = bbox.offset(-middle);
			let crosses_split = [
				offset[0].min <= 0.0 && offset[0].max >= 0.0,
				offset[1].min <= 0.0 && offset[1].max >= 0.0,
			];

			if crosses_split[0] && crosses_split[1] {
				// Bounding box crosses both splits, recurse into all four children
				for &(x, y) in &[(0, 0), (0, 1), (1, 0), (1, 1)] {
					self.traverse_nodes_r(child_nodes[x][y], bbox, func);
				}
			} else if crosses_split[0] {
				// Bounding box crosses x-axis split only
				let y = (offset[1].min > 0.0) as usize;
				for x in 0..2 {
					self.traverse_nodes_r(child_nodes[x][y], bbox, func);
				}
			} else if crosses_split[1] {
				// Bounding box crosses y-axis split only
				let x = (offset[0].min > 0.0) as usize;
				for y in 0..2 {
					self.traverse_nodes_r(child_nodes[x][y], bbox, func);
				}
			} else {
				// Bounding box lies in one of the quadrants, recurse into that quadrant
				let [x, y] = [
					(offset[0].min > 0.0) as usize,
					(offset[1].min > 0.0) as usize,
				];
				self.traverse_nodes_r(child_nodes[x][y], bbox, func);
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct QuadtreeNode {
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
	fn clear(&mut self, bbox: AABB2) {
		self.entities.clear();
		self.num_descendants = 0;
		self.bbox = bbox;
		self.children = None;
	}
}
