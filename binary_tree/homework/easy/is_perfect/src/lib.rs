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
pub fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        return depth(node.left.clone()).max(depth(node.right.clone())) + 1;
    };
    0
}

fn is_perfect(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    _is_perfect(root.clone(), depth(root))
}

fn _is_perfect(root: Option<Rc<RefCell<TreeNode>>>, curr_height: i32) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        if node.left.is_none() && node.right.is_some()
            || node.left.is_some() && node.right.is_none()
        {
            return false;
        }
        if node.left.is_none() && node.right.is_none() {
            return curr_height == 0;
        }
        return _is_perfect(node.left.clone(), curr_height - 1)
            && _is_perfect(node.right.clone(), curr_height - 1);
    }
    true
}
