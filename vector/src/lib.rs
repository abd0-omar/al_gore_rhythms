use std::alloc;
use std::ptr::NonNull;

struct Vecz {
    len: usize,
    cap: usize,
    ptr: NonNull<i32>,
}

impl Vecz {
    fn new() -> Self {
        Vecz {
            len: 0,
            cap: 0,
            ptr: NonNull::dangling(),
        }
    }

    fn get_len(&self) -> usize {
        self.len
    }

    fn capac(&self) -> usize {
        self.cap
    }

    fn push(&mut self, item: i32) {
        assert_ne!(std::mem::size_of::<i32>(), 0, "No zero sized types");
        // same as
        // if std::mem::size_of::<i32>() == 0 {
        //     panic!("No zero sized types");
        // }
        if self.cap == 0 {
            // let layout = alloc::Layout::array(4).expect("could not allocatez");
            // let layout = Layout::array::<i32>(4).expect("could not allocatez");
            let layout = alloc::Layout::array::<i32>(2).expect("could not allocate"); //it was 4
            let ptr = unsafe { alloc::alloc(layout) } as *mut i32; //u8 might change it later
            let ptr = NonNull::new(ptr).expect("could not allocate memory");
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.cap = 2; //it was 4, no cap
            self.len = 1;
        } else if self.len < self.cap {
            unsafe { self.ptr.as_ptr().add(self.len).write(item) } //change ptr to a raw ptr and make it point
                                                                   //at the place of uninitiazlized mem and it is zero based
            self.len += 1;
        } else {
            assert!(self.len == self.cap, "how did that happen?!");
            let new_cap = self.cap * 2;
            // let layout = unsafe { alloc::Layout::array::<i32>(self.cap * 2).unwrap() };
            let size = std::mem::size_of::<i32>() * new_cap;
            let align = std::mem::align_of::<i32>();
            let layout = alloc::Layout::from_size_align(size, align)
                .expect("could not lay it out aka allocate");
            let ptr = unsafe { alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_cap) };
            let ptr = NonNull::new(ptr as *mut i32).expect("could not reallocate");
            unsafe { ptr.as_ptr().add(self.len).write(item) };
            self.ptr = ptr;
            self.len += 1;
            self.cap = new_cap;
        }
        // unsafe {
        //     let ptr = alloc(layout);
        // }
        // alloc::alloc(layout);
        // unsafe {
        //     let layout = Layout::new::<u16>();
        //     let ptr = alloc(layout);
        //     if ptr.is_null() {
        //         handle_alloc_error(layout);
        //     }
        //
        //     *(ptr as *mut u16) = 42;
        //     assert_eq!(*(ptr as *mut u16), 42);
        //
        //     dealloc(ptr, layout);
        // }
    }
}

#[test]
fn name() {
    // use super *;
    unimplemented!();
}
