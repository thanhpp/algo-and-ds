use std::cell::RefCell;
use std::collections::VecDeque;
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

// 2ms, 2.5MB
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut level: i32 = 0;
    let mut node_queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

    node_queue.push_back(root);

    while !node_queue.is_empty() {
        let length = node_queue.len();

        for _ in 0..length {
            if let Some(Some(curr)) = node_queue.pop_front() {
                if let Some(left) = curr.borrow().left.clone() {
                    node_queue.push_back(Some(left));
                };
                if let Some(right) = curr.borrow().right.clone() {
                    node_queue.push_back(Some(right));
                }
            }
        }

        level += 1;
    }

    level
}
