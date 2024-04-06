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
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut q = std::collections::VecDeque::new();
    let mut right_traverse = false;
    if let Some(node) = root {
        q.push_back(node);
    }
    let mut rezult = Vec::new();

    while !q.is_empty() {
        let size = q.len();
        let mut curr_lvl_nodes = Vec::with_capacity(size);
        for _ in 0..size {
            if right_traverse {
                let curr_node = q.pop_front().expect("can't be None inside !is_empty loop");
                curr_lvl_nodes.push(curr_node.borrow().val);
                if let Some(r) = curr_node.borrow_mut().right.take() {
                    q.push_back(r);
                }
                if let Some(l) = curr_node.borrow_mut().left.take() {
                    q.push_back(l);
                };
            } else {
                let curr_node = q.pop_back().expect("can't be None inside !is_empty loop");
                curr_lvl_nodes.push(curr_node.borrow().val);
                if let Some(l) = curr_node.borrow_mut().left.take() {
                    q.push_front(l);
                }
                if let Some(r) = curr_node.borrow_mut().right.take() {
                    q.push_front(r);
                };
            }
        }
        right_traverse = !right_traverse;
        rezult.push(curr_lvl_nodes);
    }
    rezult
}
