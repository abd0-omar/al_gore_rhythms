struct Stack {
    size: usize,
    top1: isize,
    top2: usize,
    array: Vec<i32>,
}

impl Stack {
    fn new(size: usize) -> Stack {
        Stack {
            size,
            top1: -1,
            top2: size,
            array: vec![0; size],
        }
    }

    fn push(&mut self, id: usize, x: i32) {
        assert!(!self.is_full());
        // instead of assert we can make it return Err as rust have errors as values
        if id == 1 {
            self.top1 += 1;
            self.array[self.top1 as usize] = x;
        } else {
            self.top2 -= 1;
            self.array[self.top2] = x;
        }
    }

    fn pop(&mut self, id: usize) -> Option<i32> {
        if self.is_empty(id) {
            return None;
        }
        let result = if id == 1 {
            self.top1 -= 1;
            self.array[(self.top1 + 1) as usize]
        } else {
            let result = self.array[self.top2];
            self.top2 += 1;
            result
        };
        Some(result)
    }

    #[allow(dead_code)]
    fn peek(&self, id: usize) -> Option<i32> {
        if self.is_empty(id) {
            return None;
        }
        let result = if id == 1 {
            self.array[self.top1 as usize]
        } else {
            self.array[self.top2]
        };
        Some(result)
    }

    fn is_full(&self) -> bool {
        self.top1 + 1 >= self.top2 as isize
    }

    fn is_empty(&self, id: usize) -> bool {
        if id == 1 {
            self.top1 == -1
        } else {
            self.top2 == self.size
        }
    }

    fn display(&self) {
        for i in (0..=self.top1).rev() {
            print!("{} ", self.array[i as usize]);
        }
        println!();
        for i in self.top2..self.size {
            print!("{} ", self.array[i]);
        }
        println!();
    }
}

fn main() {
    let mut stk = Stack::new(10);
    stk.push(2, 5);
    stk.push(2, 6);
    stk.pop(2);
    stk.push(2, 7);
    stk.push(2, 9);

    stk.push(1, 4);
    stk.push(1, 6);
    stk.push(1, 8);
    stk.display();
}
