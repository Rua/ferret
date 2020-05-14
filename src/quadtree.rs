use crate::geometry::{Interval, AABB2};
use nalgebra::Vector2;
use specs::Entity;
use std::collections::HashMap;
use std::iter::IntoIterator;
use std::mem;

type Vector = Vector2<f32>;

#[derive(Clone, Debug)]
pub struct Quadtree {
	root: QuadtreeNode,
	bbox: AABB2,
	bboxes: HashMap<Entity, AABB2>,
}

#[derive(Clone, Debug)]
enum QuadtreeNode {
	Branch(Node),
	Leaf(Leaf),
}

#[derive(Clone, Debug)]
struct Node {
	middle: Vector,
	entities: Vec<Entity>,
	num_descendants: usize,
	children: Box<[QuadtreeNode; 4]>,
}

#[derive(Clone, Debug)]
struct Leaf {
	entities: Vec<Entity>,
}

impl Quadtree {
	// Minimum number of entities stored into an inner node;
	//  fewer entities are always represented as a lead
	const NODE_MIN: usize = 5;

	// Maximum number of entities represented as a leaf before splitting it into
	//  an inner node; may be exceeded if splitting wouldn't separated the
	//  entities (may happen if their AABBs overlap)
	const LEAF_MAX: usize = 10;

	#[inline]
	pub fn new(bbox: AABB2) -> Quadtree {
		assert!(!bbox.is_empty());
		Quadtree {
			root: QuadtreeNode::empty(),
			bbox,
			bboxes: HashMap::new(),
		}
	}

	#[inline]
	pub fn insert(&mut self, entity: Entity, bbox: &AABB2) {
		assert!(!bbox.is_empty());

		if !self.bboxes.contains_key(&entity) {
			self.bboxes.insert(entity, bbox.clone());
			// TODO: Find a better pattern than mem::replace / update / place back

			let root = mem::replace(&mut self.root, QuadtreeNode::empty());
			self.root = root.insert(&self.bbox, entity, &bbox, &self.bboxes);
		}
	}

	#[inline]
	pub fn remove(&mut self, entity: Entity) {
		if let Some(bbox) = self.bboxes.remove(&entity) {
			let root = mem::replace(&mut self.root, QuadtreeNode::empty());
			self.root = root.remove(entity, &bbox)
		}
	}

	#[inline]
	pub fn traverse_nodes<F: FnMut(&[Entity])>(&self, bbox: &AABB2, func: &mut F) {
		self.root.traverse(bbox, func);
	}
}

impl Leaf {
	#[inline]
	fn new() -> Leaf {
		Leaf {
			entities: Vec::new(),
		}
	}

	#[inline]
	fn from_entities(entities: Vec<Entity>) -> Leaf {
		Leaf { entities }
	}

	fn insert(
		mut self,
		self_bbox: &AABB2,
		entity: Entity,
		bbox: &AABB2,
		bboxes: &HashMap<Entity, AABB2>,
	) -> QuadtreeNode {
		if self.entities.len() < Quadtree::LEAF_MAX {
			self.entities.push(entity);
			return QuadtreeNode::Leaf(self);
		} else {
			let mut n = self.split(self_bbox, bboxes);
			n.insert(self_bbox, entity, bbox, bboxes);
			return QuadtreeNode::Branch(n);
		}
	}

	fn split(self, self_bbox: &AABB2, bboxes: &HashMap<Entity, AABB2>) -> Node {
		debug_assert!(self.entities.len() >= Quadtree::LEAF_MAX);

		// Create inner node
		let middle = self_bbox.middle();
		let mut n = Node::new(middle);

		for e in self.entities.into_iter() {
			n.insert(self_bbox, e, &bboxes[&e], &bboxes)
		}

		n
	}

	#[inline]
	fn remove(&mut self, entity: Entity) {
		let pos = self.entities.iter().position(|x| *x == entity).unwrap();
		self.entities.swap_remove(pos);
	}

	fn collapse_into(self, v: &mut Vec<Entity>) {
		v.extend(self.entities)
	}

	#[inline]
	fn traverse<F: FnMut(&[Entity])>(&self, f: &mut F) {
		if !self.entities.is_empty() {
			f(&self.entities);
		}
	}
}

impl Node {
	#[inline]
	fn new(middle: Vector) -> Node {
		Node {
			middle,
			entities: Vec::new(),
			num_descendants: 0,
			children: Box::new([
				QuadtreeNode::empty(),
				QuadtreeNode::empty(),
				QuadtreeNode::empty(),
				QuadtreeNode::empty(),
			]),
		}
	}

	#[inline]
	fn child(&self, bbox: &AABB2) -> Option<usize> {
		let offset = bbox.offset(-self.middle);
		let horizontal_cross = offset[0].min <= 0.0 && offset[0].max >= 0.0;
		let vertical_cross = offset[1].min <= 0.0 && offset[1].max >= 0.0;

		if horizontal_cross || vertical_cross {
			// Bounding box lies on the split
			None
		} else {
			// Bounding box lies in one of the quadrants
			let right = offset[0].min > 0.0;
			let up = offset[1].min > 0.0;
			Some(2 * (right as usize) + (up as usize))
		}
	}

	#[inline]
	fn child_box(&self, self_bbox: &AABB2, i: usize) -> AABB2 {
		debug_assert!(i < 4);
		let right = (i / 2) != 0;
		let up = (i % 2) != 0;

		AABB2::from_intervals(Vector2::new(
			if right {
				Interval::new(self.middle[0], self_bbox[0].max)
			} else {
				Interval::new(self_bbox[0].min, self.middle[0])
			},
			if up {
				Interval::new(self.middle[1], self_bbox[1].max)
			} else {
				Interval::new(self_bbox[1].min, self.middle[1])
			},
		))
	}

	fn insert(
		&mut self,
		self_bbox: &AABB2,
		entity: Entity,
		bbox: &AABB2,
		bboxes: &HashMap<Entity, AABB2>,
	) {
		self.num_descendants += 1;

		if let Some(i) = self.child(bbox) {
			let c_bbox = self.child_box(self_bbox, i);
			let child = mem::replace(&mut self.children[i], QuadtreeNode::empty());
			self.children[i] = child.insert(&c_bbox, entity, bbox, bboxes);
		} else {
			self.entities.push(entity);
		}
	}

	fn remove(mut self, entity: Entity, bbox: &AABB2) -> QuadtreeNode {
		self.num_descendants -= 1;

		if self.num_descendants < Quadtree::NODE_MIN {
			let mut leaf = self.collapse();
			leaf.remove(entity);
			QuadtreeNode::Leaf(leaf)
		} else {
			if let Some(i) = self.child(bbox) {
				let child = mem::replace(&mut self.children[i], QuadtreeNode::empty());
				self.children[i] = child.remove(entity, bbox)
			} else {
				let pos = self.entities.iter().position(|x| *x == entity).unwrap();
				self.entities.swap_remove(pos);
			}
			QuadtreeNode::Branch(self)
		}
	}

	fn collapse(mut self) -> Leaf {
		let mut v = mem::replace(&mut self.entities, Vec::new());
		self.collapse_into(&mut v);
		Leaf::from_entities(v)
	}

	fn collapse_into(self, v: &mut Vec<Entity>) {
		v.extend(self.entities);

		// TODO: Find a better way to express this.
		let [a, b, c, d] = *self.children;
		a.collapse_into(v);
		b.collapse_into(v);
		c.collapse_into(v);
		d.collapse_into(v);
	}

	fn traverse<F: FnMut(&[Entity])>(&self, bbox: &AABB2, f: &mut F) {
		if let Some(i) = self.child(&bbox) {
			self.children[i].traverse(&bbox, f);
		} else {
			if !self.entities.is_empty() {
				f(&self.entities);
			}

			for child in self.children.iter() {
				child.traverse(&bbox, f);
			}
		}
	}
}

impl QuadtreeNode {
	#[inline]
	fn insert(
		self,
		self_bbox: &AABB2,
		entity: Entity,
		bbox: &AABB2,
		bboxes: &HashMap<Entity, AABB2>,
	) -> QuadtreeNode {
		match self {
			QuadtreeNode::Leaf(l) => l.insert(self_bbox, entity, bbox, bboxes),
			QuadtreeNode::Branch(mut b) => {
				b.insert(self_bbox, entity, bbox, bboxes);
				QuadtreeNode::Branch(b)
			}
		}
	}

	#[inline]
	fn remove(self, entity: Entity, bbox: &AABB2) -> QuadtreeNode {
		match self {
			QuadtreeNode::Branch(b) => b.remove(entity, &bbox),
			QuadtreeNode::Leaf(mut l) => {
				l.remove(entity);
				QuadtreeNode::Leaf(l)
			}
		}
	}

	#[inline]
	fn empty() -> QuadtreeNode {
		QuadtreeNode::Leaf(Leaf::new())
	}

	#[inline]
	fn collapse_into(self, v: &mut Vec<Entity>) {
		match self {
			QuadtreeNode::Branch(b) => b.collapse_into(v),
			QuadtreeNode::Leaf(l) => l.collapse_into(v),
		}
	}

	#[inline]
	fn traverse<F: FnMut(&[Entity])>(&self, bbox: &AABB2, f: &mut F) {
		match self {
			QuadtreeNode::Leaf(l) => l.traverse(f),
			QuadtreeNode::Branch(b) => b.traverse(&bbox, f),
		}
	}
}
