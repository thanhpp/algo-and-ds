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

// 1ms, 2.8MB
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut result = Vec::<i32>::new();
    let mut queue = std::collections::VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let l = queue.len();

        // right most node of a level
        let n = match queue.pop_back() {
            Some(Some(n)) => n,
            _ => continue,
        };
        let n = n.borrow();
        result.push(n.val);
        println!("node {}", n.val);

        // handle remain nodes of the current level
        for _ in 0..(l - 1) {
            let n = match queue.pop_front() {
                Some(Some(n)) => n,
                _ => continue,
            };
            let n = n.borrow();
            println!("node {}", n.val);
            if n.left.is_some() {
                queue.push_back(n.left.clone());
            }
            if n.right.is_some() {
                queue.push_back(n.right.clone());
            }
        }

        // next level
        if n.left.is_some() {
            queue.push_back(n.left.clone());
        }
        if n.right.is_none() {
            queue.push_back(n.right.clone());
        }
    }

    result
}
