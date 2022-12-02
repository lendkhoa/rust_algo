use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

/**
 * Defines a struct
 */
 pub struct Binary_Tree<T> {
	pub root: Option<NonNull<Node<T>>>
}

impl Binary_Tree {
	pub fn new() -> Self {
		Self {
			root: None,
		}
	}

	pub fn insert(&mut self, &mut Option<NonNull<Node<T>>> root, u32: data) -> Binary_Tree {
		let mut pNew = Box::new(Node::new(data));
		if self.root.is_none() {
			return pNew;
		}
	}

}
