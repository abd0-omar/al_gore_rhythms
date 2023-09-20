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

struct Solution {}

use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let mut mut_node = node.borrow_mut();
            let mut x = &mut *mut_node;
            swap(&mut x.left, &mut x.right);
            let _ = Solution::invert_tree(mut_node.left.clone());
            let _ = Solution::invert_tree(mut_node.right.clone());
        }
        root
    }
}

fn main() {
    println!("Hello, world!");
}
