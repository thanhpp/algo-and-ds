use std::cell::RefCell;
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

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_diameter: i32 = 0;
    dfs(root, &mut max_diameter);

    max_diameter
}

// we define that the height of a None node = -1, and the height of a node without any children = 0.
// in case
// ~ a node that has only a left/right child: d = 0 + -1 + 2 = 1
// ~ a node that doesn't have any child: d = -1 + -1 + 2 = 0
// returning the height
pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
    let n = match node {
        None => return -1,
        Some(n) => n,
    };

    let left_height = dfs(n.borrow().left.clone(), max_diameter);
    let right_height = dfs(n.borrow().right.clone(), max_diameter);

    if left_height + right_height + 2 > *max_diameter {
        *max_diameter = left_height + right_height + 2
    }

    if left_height > right_height {
        return 1 + left_height;
    }

    1 + right_height
}
