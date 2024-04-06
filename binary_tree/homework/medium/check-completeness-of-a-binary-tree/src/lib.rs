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
pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut q = std::collections::VecDeque::new();
    if let Some(node) = root {
        q.push_back(node);
    }

    let mut done = false;
    while !q.is_empty() {
        for _ in 0..q.len() {
            let curr_node = q.pop_front().unwrap();

            if let Some(l) = curr_node.borrow_mut().left.take() {
                if done {
                    return false;
                }
                q.push_back(l);
            } else {
                // wohaosho
                done = true;
            };

            if let Some(r) = curr_node.borrow_mut().right.take() {
                if done {
                    return false;
                }
                q.push_back(r);
            } else {
                // woah
                done = true;
            };
        }
    }
    true
}
