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

// 3ms, 3,06MB
// Time O(n): visit all nodes
// Space O(n): stack can contains every node
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut idx = 0;
    let mut stack = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
    let mut curr = root;

    while curr.is_some() || !stack.is_empty() {
        // go to the left most node (the smallest node on the BST)
        while curr.is_some() {
            stack.push(curr.clone());
            let c = match curr {
                Some(c) => c,
                _ => break,
            };
            let c = c.borrow();
            curr = c.left.clone();
        }

        // pop the left most node
        let n = match stack.pop() {
            Some(Some(n)) => n,
            _ => continue,
        };
        let n = n.borrow();
        idx += 1;
        if idx == k {
            return n.val;
        }

        // start to visit the right side of the left most node
        curr = n.right.clone()
    }

    -1
}
