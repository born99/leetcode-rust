mod binary_tree;
use std::{cell::RefCell, rc::Rc};

use crate::binary_tree::inorder_traversal::{TreeNode, inorder_traversal};

fn main() {
    // Binary (?) 1: Inorder tranversal
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    println!("{:?}", inorder_traversal(root));
}
