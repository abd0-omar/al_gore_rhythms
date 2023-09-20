//788
use std::fmt;
use std::ptr;
use std::vec::Vec;

struct Node {
    data: i32,
    next: *mut Node,
}

impl Node {
    fn new(data: i32) -> Self {
        Node {
            data,
            next: ptr::null_mut(),
        }
    }
}

struct LinkedList {
    head: *mut Node,
    tail: *mut Node,
    length: i32,
    debug_data: Vec<*mut Node>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            length: 0,
            debug_data: Vec::new(),
        }
    }

    fn debug_add_node(&mut self, node: *mut Node) {
        self.debug_data.push(node);
    }

    fn debug_remove_node(&mut self, node: *mut Node) {
        if let Some(index) = self.debug_data.iter().position(|&n| n == node) {
            self.debug_data.remove(index);
        } else {
            println!("Node does not exist");
        }
    }

    fn debug_print_node(&self, node: *mut Node, is_separate: bool) {
        if is_separate {
            print!("Sep: ");
        }
        if node.is_null() {
            print!("nullptr\n");
            return;
        }
        unsafe {
            print!("{} ", (*node).data);
            if (*node).next.is_null() {
                print!("X ");
            } else {
                print!("{} ", (*(*node).next).data);
            }
            if node == self.head {
                println!("head");
            } else if node == self.tail {
                println!("tail");
            } else {
                println!();
            }
        }
    }

    fn debug_print_list(&self, msg: &str) {
        if !msg.is_empty() {
            println!("{}", msg);
        }
        for node in &self.debug_data {
            self.debug_print_node(*node, false);
        }
        println!("************");
    }

    fn debug_to_string(&self) -> String {
        if self.length == 0 {
            return String::new();
        }
        let mut result = String::new();
        let mut curr = self.head;
        while !curr.is_null() {
            unsafe {
                result.push_str(&(*curr).data.to_string());
                if !(*curr).next.is_null() {
                    result.push(' ');
                }
                curr = (*curr).next;
            }
        }
        result
    }

    fn debug_verify_data_integrity(&self) {
        if self.length == 0 {
            assert_eq!(self.head, ptr::null_mut());
            assert_eq!(self.tail, ptr::null_mut());
            return;
        }

        assert_ne!(self.head, ptr::null_mut());
        assert_ne!(self.tail, ptr::null_mut());

        unsafe {
            assert_eq!((*self.tail).next, ptr::null_mut());
        }

        if self.length == 1 {
            assert_eq!(self.head, self.tail);
        } else {
            assert_ne!(self.head, self.tail);

            if self.length == 2 {
                unsafe {
                    assert_eq!((*self.head).next, self.tail);
                }
            } else if self.length == 3 {
                unsafe {
                    assert!(ptr::eq((*self.head).next, self.tail));
                }
            }
        }

        let mut len = 0;
        let mut prev = ptr::null_mut();
        let mut curr = self.head;
        while !curr.is_null() {
            assert!(len < 10_000); // Consider infinite cycle?
            len += 1;
            prev = curr;
            unsafe {
                curr = (*curr).next;
            }
        }

        assert_eq!(self.length, len);
        assert_eq!(self.length, self.debug_data.len() as i32);
        assert_eq!(prev, self.tail);
    }

    fn print(&self) {
        let mut curr = self.head;
        while !curr.is_null() {
            unsafe {
                print!("{} ", (*curr).data);
                curr = (*curr).next;
            }
        }
        println!();
    }

    fn delete_node(&mut self, node: *mut Node) {
        self.debug_remove_node(node);
        self.length -= 1;
        unsafe {
            Box::from_raw(node);
        }
    }

    fn add_node(&mut self, node: *mut Node) {
        self.debug_add_node(node);
        self.length += 1;
    }

    fn insert_end(&mut self, value: i32) {
        let item = Box::into_raw(Box::new(Node::new(value)));
        self.add_node(item);

        if self.head.is_null() {
            self.head = item;
            self.tail = item;
        } else {
            unsafe {
                (*self.tail).next = item;
            }
            self.tail = item;
        }

        self.debug_verify_data_integrity();
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut curr = self.head;
        while !curr.is_null() {
            unsafe {
                let next = (*curr).next;
                Box::from_raw(curr);
                curr = next;
            }
        }
    }
}

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut curr = self.head;
        while !curr.is_null() {
            unsafe {
                write!(f, "{} ", (*curr).data)?;
                curr = (*curr).next;
            }
        }
        Ok(())
    }
}

fn test1() {
    println!("\n\ntest1");
    let mut list = LinkedList::new();

    list.insert_end(1);
    list.insert_end(2);
    list.insert_end(3);
    // some actions
    list.print();

    let expected = "1 2 3";
    let result = list.debug_to_string();
    if expected != result {
        println!("No match:\nExpected: {}\nResult  : {}", expected, result);
        assert!(false);
    }
    list.debug_print_list("********");
}

fn test2() {
    println!("\n\ntest2");
    let mut list = LinkedList::new();

    list.insert_end(1);
    list.insert_end(2);
    list.insert_end(3);
    list.insert_end(4);
    // some actions
    list.print();

    let expected = "1 2 3 4";
    let result = list.debug_to_string();
    if expected != result {
        println!("No match:\nExpected: {}\nResult  : {}", expected, result);
        assert!(false);
    }
    list.debug_print_list("********");
}

fn main() {
    test1();
    test2();
    // must see it, otherwise RTE
    println!("\n\nNO RTE");
}
