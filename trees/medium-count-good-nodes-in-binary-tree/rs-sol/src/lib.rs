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

// 15ms, 6.8MB
pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut result = 0;
    dfs(root, std::i32::MIN, &mut result);

    result
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut max: i32, result: &mut i32) {
    let n = match node {
        Some(n) => n,
        _ => return,
    };

    let n = n.borrow();
    if n.val >= max {
        *result += 1;
        max = n.val;
    }
    dfs(n.left.clone(), max, result);
    dfs(n.right.clone(), max, result);
}
