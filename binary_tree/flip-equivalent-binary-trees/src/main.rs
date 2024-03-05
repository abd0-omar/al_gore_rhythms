fn main() {
    println!("Hello, world!");
}

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

    let string1 = parenthesize(root1, "".to_string());
    let string2 = parenthesize(root2, "".to_string());

    string1 == string2
}

pub fn parenthesize(root: Option<Rc<RefCell<TreeNode>>>, repr: String) -> String {
    if let Some(node) = root {
        let node = node.borrow();
        let mut repr = format!("({}", node.val);
        // repr.push_str(&node.val.to_string());
        let mut vec = vec![];
        if let Some(l) = node.left.clone() {
            // vec.push(parenthesize(Some(l)));
            vec.push(parenthesize(Some(l), repr.clone()));
        } else {
            repr.push_str("()");
        }
        if let Some(r) = node.right.clone() {
            vec.push(parenthesize(Some(r), repr.clone()));
        } else {
            repr.push_str("()");
        }
        vec.sort();
        println!("vec={:#?}", vec);
        for v in vec {
            repr.push_str(&v);
        }
        repr.push(')');

        repr
    } else {
        "()".to_string()
    }
}

pub fn parenthesizez(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut string = String::new();
    _parenthesize(root, &mut string);
    string
}

pub fn _parenthesize(root: Option<Rc<RefCell<TreeNode>>>, repr: &mut String) {
    if let Some(node) = root {
        let node = node.borrow();
        repr.push_str(&format!("({}", node.val));

        if let Some(l) = node.left.clone() {
            _parenthesize(Some(l), repr);
        } else {
            repr.push_str("()");
        }

        if let Some(r) = node.right.clone() {
            _parenthesize(Some(r), repr);
        } else {
            repr.push_str("()");
        }

        repr.push(')');
    } else {
        repr.push_str("()");
    }
}
