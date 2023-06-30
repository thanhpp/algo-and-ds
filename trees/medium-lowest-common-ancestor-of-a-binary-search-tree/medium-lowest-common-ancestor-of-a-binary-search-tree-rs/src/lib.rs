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

// 6ms, 3.2MB
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cur = root;
    // it is guaranted that p and q are not none
    let (p_val, q_val) = (p.unwrap().borrow().val, q.unwrap().borrow().val);

    loop {
        let c = cur.clone().unwrap();
        let c_val = c.borrow().val;

        // both larger -> on the right side
        if p_val > c_val && q_val > c_val {
            cur = c.borrow().right.clone();
            continue;
        };

        // both smaller -> on the right side
        if p_val < c_val && q_val < c_val {
            cur = c.borrow().left.clone();
            continue;
        };

        // equals 1 node or split point
        return cur;
    }
}
