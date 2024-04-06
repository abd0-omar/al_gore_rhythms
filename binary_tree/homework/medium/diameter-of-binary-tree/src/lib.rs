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
// there is two solutions
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    _diameter_of_binary_tree(root)
}

// first solution
pub fn another_sol_diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    _tree_dia(root).diameter
}

#[derive(Debug)]
pub struct Diameter {
    max_depth: i32,
    diameter: i32,
}

pub fn _tree_dia(root: Option<Rc<RefCell<TreeNode>>>) -> Diameter {
    let mut max_diameter = Diameter {
        max_depth: 0,
        diameter: 0,
    };
    if let Some(node) = root {
        let node = node.borrow();

        let mut left_max_depth = Diameter {
            max_depth: 0,
            diameter: 0,
        };

        if let Some(l) = node.left.clone() {
            left_max_depth = _tree_dia(Some(l));
        }

        let mut right_max_depth = Diameter {
            max_depth: 0,
            diameter: 0,
        };

        if let Some(r) = node.right.clone() {
            right_max_depth = _tree_dia(Some(r));
        }

        max_diameter.max_depth =
            std::cmp::max(left_max_depth.max_depth, right_max_depth.max_depth) + 1;

        max_diameter.diameter = std::cmp::max(
            left_max_depth.max_depth + right_max_depth.max_depth,
            std::cmp::max(left_max_depth.diameter, right_max_depth.diameter),
        );

        println!("max_diameter={:?}", max_diameter);
    }
    max_diameter
}

// second solution
pub fn _diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        // let curr_depth = depth(Some(node.clone()));
        let node = node.borrow();
        let left_subtree_dia = _diameter_of_binary_tree(node.left.clone());
        let right_subtree_dia = _diameter_of_binary_tree(node.right.clone());
        let left_depth = depth(node.left.clone());
        let right_depth = depth(node.right.clone());
        let curr_dia = left_depth + right_depth;
        return curr_dia.max(left_subtree_dia.max(right_subtree_dia));
    }
    0
}

pub fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left = depth(node.left.clone());
        let right = depth(node.right.clone());
        return left.max(right) + 1;
    }
    0
}
