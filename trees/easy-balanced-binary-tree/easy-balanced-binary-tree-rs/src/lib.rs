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

// 0ms, 2.9MB
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    dfs(root).1
}

pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    let n = match node {
        None => return (0, true),
        Some(n) => n,
    };

    let left = dfs(n.borrow().left.clone());
    let right = dfs(n.borrow().right.clone());

    // a tree is balanced if
    // - the left sub-tree & the right sub-tree are balanced
    // - the absolute subtraction of the left sub-tree height with the right sub-tree height is smaller than 1

    if !(left.1 && right.1) {
        return (0, false);
    }

    if (left.0 - right.0).abs() > 1 {
        return (0, false);
    }

    if left.0 > right.0 {
        return (left.0 + 1, true);
    }

    (right.0 + 1, true)
}
