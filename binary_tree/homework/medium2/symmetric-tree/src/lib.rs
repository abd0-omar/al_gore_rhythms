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
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        let left = _is_symmetric(node.left.clone(), true);
        let right = _is_symmetric(node.right.clone(), false);
        println!("{}, {}", left, right);
        left == right
    } else {
        false
    }
}

pub fn _is_symmetric(root: Option<Rc<RefCell<TreeNode>>>, normal: bool) -> String {
    if let Some(node) = root {
        let mut parenthesized = String::new();
        parenthesized.push_str(&format!("({}", node.borrow().val.to_string()));

        if normal {
            if let Some(l) = node.borrow_mut().left.take() {
                // parenthesized.push_str(&format!("({}", l.borrow().val));
                parenthesized.push_str(&_is_symmetric(Some(l), normal));
            } else {
                parenthesized.push_str("()");
            }

            if let Some(r) = node.borrow_mut().right.take() {
                // parenthesized.push_str(&format!("({}", r.borrow().val));
                parenthesized.push_str(&_is_symmetric(Some(r), normal));
            } else {
                parenthesized.push_str("()");
            }
        } else {
            if let Some(r) = node.borrow_mut().right.take() {
                // parenthesized.push_str(&format!("({}", r.borrow().val));
                parenthesized.push_str(&_is_symmetric(Some(r), normal));
            } else {
                parenthesized.push_str("()");
            }
            if let Some(l) = node.borrow_mut().left.take() {
                // parenthesized.push_str(&format!("({}", l.borrow().val));
                parenthesized.push_str(&_is_symmetric(Some(l), normal));
            } else {
                parenthesized.push_str("()");
            }
        }
        // could've also just reversed the string..I think
        parenthesized.push(')');
        return parenthesized;
    }
    String::new()
}
