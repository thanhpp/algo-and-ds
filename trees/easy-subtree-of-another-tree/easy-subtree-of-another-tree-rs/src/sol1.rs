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

type NodeRef = Rc<RefCell<TreeNode>>;

// 8ms, 2.2MB
pub fn is_subtree(root: Option<NodeRef>, sub_root: Option<NodeRef>) -> bool {
    is_subtree_recursive(&root, &sub_root)
}

pub fn is_subtree_recursive(root: &Option<NodeRef>, sub_root: &Option<NodeRef>) -> bool {
    match (root, sub_root) {
        (None, None) => true,
        (Some(r), Some(s)) => {
            if is_subtree_recursive(root, sub_root) {
                return true;
            }

            let r = r.borrow();

            is_subtree_recursive(&r.left, sub_root) || is_subtree_recursive(&r.right, sub_root)
        }
        _ => false,
    }
}

pub fn is_same_tree(p: &Option<NodeRef>, q: &Option<NodeRef>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let (p, q) = (p.borrow(), q.borrow());

            if p.val != q.val {
                return false;
            }

            is_same_tree(&p.left, &q.left) && is_same_tree(&p.right, &q.right)
        }
        _ => false,
    }
}
