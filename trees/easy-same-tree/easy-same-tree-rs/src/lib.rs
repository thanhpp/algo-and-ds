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

// 1ms, 2MB
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    }
    if !(p.is_some() && q.is_some()) {
        return false;
    }

    let (p_val, p_left, p_right) = match p {
        None => (-1, None, None),
        Some(p) => (
            p.borrow().val,
            p.borrow().left.clone(),
            p.borrow().right.clone(),
        ),
    };
    let (q_val, q_left, q_right) = match q {
        None => (-1, None, None),
        Some(q) => (
            q.borrow().val,
            q.borrow().left.clone(),
            q.borrow().right.clone(),
        ),
    };

    if p_val != q_val {
        return false;
    }

    is_same_tree(p_left, q_left) && is_same_tree(p_right, q_right)
}
