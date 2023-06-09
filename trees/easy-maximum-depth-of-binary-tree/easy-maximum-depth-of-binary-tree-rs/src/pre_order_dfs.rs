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

#[allow(clippy::vec_init_then_push)]
// 0ms, 2.5MB
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut max_depth = 0;
    // (node, depth)
    let mut stack = Vec::<(Option<Rc<RefCell<TreeNode>>>, i32)>::new();
    stack.push((root, 1));

    while !stack.is_empty() {
        if let Some((Some(node), depth)) = stack.pop() {
            if depth > max_depth {
                max_depth = depth
            }

            if let Some(left) = node.borrow().left.clone() {
                stack.push((Some(left), depth + 1));
            }

            if let Some(right) = node.borrow().right.clone() {
                stack.push((Some(right), depth + 1));
            }
        }
    }

    max_depth
}
