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
pub fn flip_equiv(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    //sdfa
    // fn

    let string1 = parenthesize(root1);
    let string2 = parenthesize(root2);

    string1 == string2
}

pub fn parenthesize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut string = String::new();
    _parenthesize(root, &mut string);
    string
}

pub fn _parenthesize(root: Option<Rc<RefCell<TreeNode>>>, repr: &mut String) {
    if let Some(node) = root {
        let node = node.borrow();
        repr.push_str(&format!("({}", node.val));

        let mut vec = vec![];

        if let Some(l) = node.left.clone() {
            let mut left_repr = String::new();
            _parenthesize(Some(l), &mut left_repr);
            vec.push(left_repr);
        } else {
            repr.push_str("()");
        }

        if let Some(r) = node.right.clone() {
            let mut right_repr = String::new();
            _parenthesize(Some(r), &mut right_repr);
            vec.push(right_repr);
        } else {
            repr.push_str("()");
        }

        vec.sort();
        for v in vec {
            repr.push_str(&v);
        }

        repr.push(')');
    } else {
        repr.push_str("()");
    }
}
