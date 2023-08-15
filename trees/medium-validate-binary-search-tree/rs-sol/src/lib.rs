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

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let r = match root {
        Some(r) => r,
        None => return true,
    };

    let r = r.borrow();
    let val = r.val as i64;

    dfs(r.left.clone(), val, std::i64::MIN) && dfs(r.right.clone(), std::i64::MAX, val)
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max: i64, min: i64) -> bool {
    let n = match node {
        Some(n) => n,
        _ => return true,
    };

    let n = n.borrow();
    let val = n.val as i64;

    if val >= max || val <= min {
        return false;
    }

    dfs(n.left.clone(), val, min) && dfs(n.right.clone(), max, val)
}
