//788
use std::fmt;
use std::ptr;
use std::vec::Vec;

#[derive(Debug)]
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
    unsafe fn destroy(&self) {
        println!("Destroy value: {} at address {:?}", self.data, self);
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

    fn debug_print_address(&self) {
        let mut cur = self.head;
        while !cur.is_null() {
            unsafe {
                print!("{:?}, {}\t", cur, (*cur).data);
                cur = (*cur).next;
            }
        }
        println!();
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

    fn debug_verify_data_integrity(&self) -> Result<(), String> {
        if self.length == 0 {
            if self.head != ptr::null_mut() || self.tail != ptr::null_mut() {
                return Err(String::from("Head and tail should be null"));
            }
            return Ok(());
        }

        if self.head == ptr::null_mut() || self.tail == ptr::null_mut() {
            return Err(String::from("Head and tail should not be null"));
        }

        unsafe {
            if (*self.tail).next != ptr::null_mut() {
                return Err(String::from("Tail next should be null"));
            }
        }

        if self.length == 1 {
            if self.head != self.tail {
                return Err(String::from("Head and tail should be the same"));
            }
            return Ok(());
        }

        if self.head == self.tail {
            return Err(String::from("Head and tail should be different"));
        }

        if self.length == 2 {
            unsafe {
                if (*self.head).next != self.tail {
                    return Err(String::from("Head next should be tail"));
                }
            }
        } else if self.length == 3 {
            unsafe {
                if !ptr::eq((*self.head).next, self.tail) {
                    return Err(String::from("Head next should be tail"));
                }
            }
        }

        let mut len = 0;
        let mut prev = ptr::null_mut();
        let mut curr = self.head;
        while !curr.is_null() {
            if len >= 10_000 {
                return Err(String::from("Possible infinite cycle detected"));
            }
            len += 1;
            prev = curr;
            unsafe {
                curr = (*curr).next;
            }
        }

        if self.length != len {
            return Err(String::from("Length mismatch"));
        }

        if self.length != self.debug_data.len() as i32 {
            return Err(String::from("Debug data length mismatch"));
        }

        if prev != self.tail {
            return Err(String::from("Last node should be tail"));
        }

        Ok(())
    }

    // curr = head, while curr print(curr),
    // curr = curr -> next
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

    // if tail.isEmpty, new_node = head = tail
    // new_node(value), tail -> next = new_node,
    // new_node = tail
    fn insert_end(&mut self, value: i32) {
        let item = Box::into_raw(Box::new(Node::new(value)));
        self.add_node(item);

        if self.head.is_null() {
            self.head = item;
            self.tail = item;
        } else {
            unsafe {
                (*self.tail).next = item;
                (*item).next = ptr::null_mut(); // Set the next pointer of the new tail to null
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
