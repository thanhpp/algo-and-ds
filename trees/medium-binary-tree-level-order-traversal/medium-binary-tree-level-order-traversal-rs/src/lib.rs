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

// 1ms, 2.3MB
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let cur = root;
    let mut queue = std::collections::VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
    let mut result = Vec::<Vec<i32>>::new();

    queue.push_back(cur);

    while !queue.is_empty() {
        let l = queue.len();
        let mut level = Vec::<i32>::new();

        for _ in 0..l {
            let n = queue.pop_front();

            let n = match n {
                Some(Some(val)) => val,
                _ => continue,
            };

            level.push(n.borrow().val);

            if n.borrow().left.is_some() {
                queue.push_back(n.borrow().left.clone())
            }

            if n.borrow().right.is_some() {
                queue.push_back(n.borrow().right.clone())
            }
        }

        if !level.is_empty() {
            result.push(level)
        }
    }

    result
}
