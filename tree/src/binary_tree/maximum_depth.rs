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


pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut result = 0;

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, count: i32, result: &mut i32) { 
        if root == None {
            return ;
        }
        let root = root.unwrap();

        let count = count + 1;
        if count > *result {
            *result = count;
        }

        traverse(root.borrow().left.clone(), count,  result);
        traverse(root.borrow().right.clone(), count, result);
    }

    traverse(root, 0, &mut result);

    result
}