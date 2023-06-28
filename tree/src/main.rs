mod binary_tree;
use std::{cell::RefCell, rc::Rc};

use crate::binary_tree::{inorder_traversal::{inorder_traversal, TreeNode}, maximum_depth::{max_depth, self}};

fn main() {
	// Binary tree 1: 104 - Inorder tranversal
	let root = Some(Rc::new(RefCell::new(TreeNode {
		val: 1,
		left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
		right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
	})));
	println!("{:?}", inorder_traversal(root));

    // Binary tree 2: 104-- Max height
    let root = Some(Rc::new(RefCell::new(maximum_depth::TreeNode {
		val: 1,
		left: Some(Rc::new(RefCell::new(maximum_depth::TreeNode::new(2)))),
		right: Some(Rc::new(RefCell::new(maximum_depth::TreeNode::new(3)))),
	})));

    println!("{}", max_depth(root));

}
