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
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_recursion(&preorder, &inorder)
}

fn build_tree_recursion(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }

    // the first value of the preorder traversal is always the root
    let (root_val, preorder) = match preorder.split_first() {
        Some((&first, preorder)) => (first, preorder),
        None => return None,
    };
    let mut node = TreeNode {
        val: root_val,
        left: None,
        right: None,
    };

    // the left side of the root_val in the inorder traversal contains nodes of the left subtree of the root
    let inorder_split = inorder.split(|&v| v == root_val).collect::<Vec<_>>();
    let inorder_left_subtree = inorder_split[0];
    let inorder_right_subtree = inorder_split[1];

    let (preorder_left_subtree, preorder_right_subtree) =
        preorder.split_at(inorder_left_subtree.len());

    node.left = build_tree_recursion(preorder_left_subtree, inorder_left_subtree);
    node.right = build_tree_recursion(preorder_right_subtree, inorder_right_subtree);

    Some(Rc::new(RefCell::new(node)))
}
