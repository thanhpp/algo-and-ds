mod bfs;
mod pre_order_dfs;

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

impl Solution {
    // 2ms, 2.8MB
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        Self::depth(root, 0)
    }

    fn depth(root: Option<Rc<RefCell<TreeNode>>>, curr: i32) -> i32 {
        if let Some(r) = root {
            let curr = curr + 1;
            let left = Self::depth(r.borrow().left.clone(), curr);
            let right = Self::depth(r.borrow().right.clone(), curr);
            if left > right {
                return left;
            }

            return right;
        }

        curr
    }
}

struct Solution {}
