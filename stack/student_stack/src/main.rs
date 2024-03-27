#[derive(Debug)]
struct Stack {
    size: usize,
    added_elements: usize,
    array: Vec<i32>,
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.array.iter() {
            write!(f, "{} ", element)?;
        }
        Ok(())
    }
}

impl Stack {
    fn new(size: usize) -> Stack {
        Stack {
            size,
            added_elements: 0,
            array: vec![0; size],
        }
    }

    fn push(&mut self, x: i32) -> bool {
        if self.is_full() {
            return false;
        }

        for i in (0..self.added_elements).rev() {
            self.array[i + 1] = self.array[i];
        }
        self.array[0] = x;
        self.added_elements += 1;
        true
    }

    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let element = self.array.get(0).copied();

        for i in 0..self.added_elements - 1 {
            self.array[i] = self.array[i + 1];
        }

        self.added_elements -= 1;
        element
    }

    fn peek(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        Some(self.array[0])
    }

    fn is_full(&self) -> bool {
        self.added_elements == self.size
    }

    fn is_empty(&self) -> bool {
        self.added_elements == 0
    }
}

fn main() {
    let mut stk = Stack::new(3);
    stk.push(10);
    stk.push(20);
    stk.push(30);

    if !stk.push(50) {
        println!("Full Stack");
    }

    println!("{:?}", stk); // 30 20 10, size: 3, added_elements: 3
    println!("{}", stk); // 30 20 10

    if let Some(result) = stk.peek() {
        println!("{}", result); // 30
    }

    while let Some(result) = stk.pop() {
        print!("{} ", result);
        // 30 20 10
    }
    println!("{:?}", stk.pop()); // None
    stk.push(40);
    println!("{:?}", stk); // 30 20 10, size: 3, added_elements: 3
}
