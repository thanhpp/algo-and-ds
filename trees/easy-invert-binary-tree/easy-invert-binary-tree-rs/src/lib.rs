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

use std::cell::RefCell;
use std::rc::Rc;

// 1ms, 2.1MB
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // take a reference
    if let Some(ref n) = root {
        // clone children
        let left = n.borrow().left.clone();
        let right = n.borrow().right.clone();

        // swap between left and right
        let mut node = n.borrow_mut();
        node.left = invert_tree(right);
        node.right = invert_tree(left);
    }

    root
}
