use std::{
    mem,
    ptr::{null_mut, NonNull},
};

// #[derive(Debug)]
pub struct Vecz {
    len: usize,
    ptr: NonNull<i32>,
    cap: usize,
}

impl Vecz {
    fn new() -> Self {
        assert!(mem::size_of::<usize>() != 0, "Whoops!");
        Vecz {
            len: 0,
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }
}

pub struct Fector {
    len: usize,
    cap: usize,
    ptr: *mut i32,
}

impl Fector {
    fn new() -> Self {
        Fector {
            len: 0,
            cap: 0,
            ptr: null_mut(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
