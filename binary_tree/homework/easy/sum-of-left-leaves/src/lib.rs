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
pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // we could make a &mut i32 that will summ the left leaves with extra parameter to
    // the _sum_.. fn
    _sum_of_left_leaves(root, false)
}

pub fn _sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>, is_coming_from_left: bool) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let mut sum = 0;
        if node.left.is_none() && node.right.is_none() && is_coming_from_left {
            sum += node.val;
        }

        return _sum_of_left_leaves(node.left.clone(), true)
            + _sum_of_left_leaves(node.right.clone(), false)
            + sum;
    }
    0
}
