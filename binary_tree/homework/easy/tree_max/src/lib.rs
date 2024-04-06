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
pub fn tree_max(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    _tree_max(root)
}

pub fn _tree_max(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let mut max = node.val;
        if let Some(l) = node.left.clone() {
            max = max.max(_tree_max(Some(l)))
        }

        if let Some(r) = node.right.clone() {
            max = max.max(_tree_max(Some(r)))
        }
        return max;
    }
    i32::MIN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let root = None;
        assert_eq!(tree_max(root), i32::MIN);
    }

    #[test]
    fn test_single_node_tree() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        assert_eq!(tree_max(root), 5);
    }

    #[test]
    fn test_left_subtree_only() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        })));
        assert_eq!(tree_max(root), 5);
    }

    #[test]
    fn test_right_subtree_only() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        })));
        assert_eq!(tree_max(root), 7);
    }

    #[test]
    fn test_both_subtrees() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        })));
        assert_eq!(tree_max(root), 7);
    }

    // Add more tests as needed to cover other scenarios
}
