mod sol1;

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

// 11ms, 2.2MB
pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let (r, s) = match (root, sub_root) {
        (None, None) => return true,
        (Some(r), Some(s)) => (r, s),
        _ => return false,
    };

    if is_same_tree(Some(r.clone()), Some(s.clone())) {
        return true;
    }

    let (r_left, r_right) = (r.borrow().left.clone(), r.borrow().right.clone());

    is_subtree(r_left, Some(s.clone())) || is_subtree(r_right, Some(s))
}

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let (p_in, q_in) = match (p, q) {
        (None, None) => return true,
        (Some(pp), Some(qq)) => (pp, qq),
        _ => return false,
    };

    if p_in.borrow().val != q_in.borrow().val {
        return false;
    }

    let (p_left, p_right) = (p_in.borrow().left.clone(), p_in.borrow().right.clone());
    let (q_left, q_right) = (q_in.borrow().left.clone(), q_in.borrow().right.clone());

    is_same_tree(p_left, q_left) && is_same_tree(p_right, q_right)
}
