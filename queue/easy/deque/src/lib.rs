#[derive(Default, Debug)]
struct Queue {
    size: usize,
    front: usize,
    rear: usize,
    added_elements: usize,
    vector: Vec<i32>,
}

impl Queue {
    fn new(size: usize) -> Self {
        Self {
            size,
            vector: vec![0; size],
            ..Queue::default()
        }
    }

    fn next(&self, mut pos: usize) -> usize {
        pos += 1;
        if pos == self.size {
            pos = 0;
        }
        pos
    }

    fn enqueue(&mut self, value: i32) {
        assert!(!self.is_full());
        self.vector[self.rear] = value;
        self.rear = self.next(self.rear);
        self.added_elements += 1;
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        let value = self.vector[self.front];
        self.front = self.next(self.front);
        self.added_elements -= 1;
        Some(value)
    }

    fn display(&self) {
        println!("Front {} - Rear {}", self.front, self.rear);
        if self.is_full() {
            println!("Full");
        } else if self.is_empty() {
            println!("Empty");
        } else {
            let mut cur = self.front;
            for _ in 0..self.added_elements {
                print!("{} ", self.vector[cur]);
                cur = self.next(cur);
            }
            println!();
        }
    }

    fn is_empty(&self) -> bool {
        self.added_elements == 0
    }

    fn is_full(&self) -> bool {
        self.added_elements == self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_operations() {
        let mut qu = Queue::new(6);
        assert!(qu.is_empty());
        qu.display();

        for i in 1..=6 {
            assert!(!qu.is_full());
            qu.enqueue(i);
            qu.display();
        }
        assert!(qu.is_full());

        for _ in 1..=6 {
            assert!(!qu.is_empty());
            qu.dequeue();
            // qu.display();
        }

        for i in 1..=6 {
            assert!(!qu.is_full());
            qu.enqueue(i);
            qu.display();
        }

        qu.dequeue();
        assert!(!qu.is_full());
        qu.enqueue(7);
        assert!(qu.is_full());
        qu.display();

        qu.dequeue();
        qu.dequeue();
        assert!(!qu.is_full());
        qu.enqueue(8);
        assert!(!qu.is_full());
        qu.display();
        qu.enqueue(9);
        assert!(qu.is_full());
        qu.display();

        println!("{:?}", qu);

        for _ in 1..=6 {
            assert!(!qu.is_empty());
            qu.dequeue();
            qu.display();
        }
    }
}
