use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
	pub val: i32,
	pub left: Option<Rc<RefCell<TreeNode>>>,
	pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
	#[inline]
	pub fn new(val: i32) -> Self {
		TreeNode {
			val,
			left: None,
			right: None,
		}
	}
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
	let mut result = vec![];

	fn traversal(result: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
		if root == None {
			return;
		}
		let root = root.unwrap();

		traversal(result, root.borrow().left.clone());

		let value = root.borrow().val;
		result.push(value);

		traversal(result, root.borrow().right.clone());
	}
	traversal(&mut result, root);

	result
}
