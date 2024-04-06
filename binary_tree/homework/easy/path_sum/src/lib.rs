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
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    _path_sum(root, target_sum)
}

pub fn _path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        let curr_val = node.val;

        if node.left.is_none() && node.right.is_none() && target_sum - curr_val == 0 {
            return true;
        }

        let left = _path_sum(
            node.left.clone(),
            target_sum - curr_val, /*  - left_val */
        );
        let right = _path_sum(
            node.right.clone(),
            target_sum - curr_val, /* - right_val */
        );

        return left || right;
    };

    false
}
