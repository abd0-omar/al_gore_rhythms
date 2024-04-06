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
use std::collections::VecDeque;
use std::rc::Rc;
pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    let mut q = VecDeque::new();
    if let Some(node) = root {
        q.push_back((node, None));
    }
    while !q.is_empty() {
        let mut x_found = None;
        let mut y_found = None;
        for _ in 0..q.len() {
            let (node, parent) = q.pop_front().unwrap();
            let val = node.borrow().val;
            if val == x {
                x_found = parent;
            }
            if val == y {
                y_found = parent;
            }
            if let Some(n) = node.borrow_mut().left.take() {
                q.push_back((n, Some(val)));
            }
            if let Some(n) = node.borrow_mut().right.take() {
                q.push_back((n, Some(val)));
            };
        }
        if let (Some(x_parent), Some(y_parent)) = (x_found, y_found) {
            return x_parent != y_parent;
        }
    }
    false
}
