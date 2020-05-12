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

	fn find_node(&self, index: usize, bbox: &AABB2) -> usize {
		if let Some((middle, child_nodes)) = self.nodes[index].children {
			let offset = bbox.offset(-middle);
			let crosses_split = [
				offset[0].min <= 0.0 && offset[0].max >= 0.0,
				offset[1].min <= 0.0 && offset[1].max >= 0.0,
			];

			if crosses_split[0] || crosses_split[1] {
				// Bounding box lies on the split
				index
			} else {
				// Bounding box lies in one of the quadrants
				let sides = [
					(offset[0].min > 0.0) as usize,
					(offset[1].min > 0.0) as usize,
				];
				self.find_node(child_nodes[sides[0]][sides[1]], bbox)
			}
		} else {
			index
		}
	}

	fn split_node(&mut self, index: usize) {
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
			let child = self.find_node(index, bbox);
			if child != index {
				// Entity goes in child node
				let data = self.nodes[index].entities.swap_remove(i);
				self.nodes[child].entities.push(data);
			} else {
				// Entity stays in current node
				i += 1;
			}
		}
	}

	pub fn insert(&mut self, entity: Entity, bbox: AABB2) {
		assert!(!bbox.is_empty());

		if !self.bboxes.contains_key(&entity) {
			const NODE_MAX: usize = 10;
			let index = self.find_node(0, &bbox);

			// Does the node need to be split?
			if self.nodes[index].entities.len() == NODE_MAX && self.nodes[index].children.is_none()
			{
				self.split_node(index);

				// Try again
				self.insert(entity, bbox);
			} else {
				self.nodes[index].entities.push(entity);
				self.bboxes.insert(entity, bbox);
			}
		}
	}

	pub fn remove(&mut self, entity: Entity) {
		if let Some(bbox) = self.bboxes.remove(&entity) {
			let index = self.find_node(0, &bbox);
			let pos = self.nodes[index]
				.entities
				.iter()
				.position(|x| *x == entity)
				.unwrap();
			self.nodes[index].entities.swap_remove(pos);
		}
	}

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
				self.traverse_nodes_r(child_nodes[0][0], bbox, func);
				self.traverse_nodes_r(child_nodes[0][1], bbox, func);
				self.traverse_nodes_r(child_nodes[1][0], bbox, func);
				self.traverse_nodes_r(child_nodes[1][1], bbox, func);
			} else if crosses_split[0] {
				// Bounding box crosses x-axis split only
				let side = (offset[1].min > 0.0) as usize;
				self.traverse_nodes_r(child_nodes[side][0], bbox, func);
				self.traverse_nodes_r(child_nodes[side][1], bbox, func);
			} else if crosses_split[1] {
				// Bounding box crosses y-axis split only
				let side = (offset[0].min > 0.0) as usize;
				self.traverse_nodes_r(child_nodes[0][side], bbox, func);
				self.traverse_nodes_r(child_nodes[1][side], bbox, func);
			} else {
				// Bounding box lies in one of the quadrants, recurse into that quadrant
				let sides = [
					(offset[0].min > 0.0) as usize,
					(offset[1].min > 0.0) as usize,
				];
				self.traverse_nodes_r(child_nodes[sides[0]][sides[1]], bbox, func);
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct QuadtreeNode {
	entities: Vec<Entity>,
	bbox: AABB2,
	children: Option<(Vector2<f32>, [[usize; 2]; 2])>,
}

impl QuadtreeNode {
	#[inline]
	fn new(bbox: AABB2) -> QuadtreeNode {
		QuadtreeNode {
			entities: Vec::new(),
			bbox,
			children: None,
		}
	}

	#[inline]
	fn clear(&mut self, bbox: AABB2) {
		self.entities.clear();
		self.bbox = bbox;
		self.children = None;
	}
}
