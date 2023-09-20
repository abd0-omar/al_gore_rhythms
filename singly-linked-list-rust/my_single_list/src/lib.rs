use std::ptr::null_mut;

struct Node {
    data: i32,
    next: *mut Node,
}

impl Node {
    pub fn new(&self, data: i32) -> Self {
        Node {
            data,
            next: null_mut(),
        }
    }
}

struct SLL {
    Head: *mut Node,
}

impl SLL {
    pub fn insert_end(&self, data: i32) {
        // if self::SLL::Head()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
