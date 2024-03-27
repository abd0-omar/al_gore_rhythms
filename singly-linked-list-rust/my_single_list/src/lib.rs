// resources used
// https://rust-unofficial.github.io/too-many-lists
// didn't know linked list can be complicated like that
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    // adding prev is not problematic
    // idx I got segfaults playing aroung with it by making pop utilizing prev
    #[allow(unused)]
    prev: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// it pops from the head still but tail is the end of it
impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }
    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }
    pub fn pop_from_head(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }

    // to get the element value we use box
    pub fn pop_from_tail_using_prev(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            let mut current = self.head;
            let mut prev = ptr::null_mut();

            // Traverse the list until the end
            while !(*current).next.is_null() {
                prev = current;
                current = (*current).next;
            }

            // Remove the last element
            if prev.is_null() {
                // If there's only one element in the list
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
            } else {
                (*prev).next = ptr::null_mut();
                self.tail = prev;
            }

            // Get the element and free the node
            let elem = Box::from_raw(current).elem;
            Some(elem)
        }
    }

    pub fn pop_from_tail_using_next_next(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            let mut current = self.head;
            let mut next = (*current).next;

            // Special case: If there's only one element in the list
            if next.is_null() {
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
                let elem = Box::from_raw(current).elem;
                return Some(elem);
            }

            // Traverse the list until the end
            while !(*next).next.is_null() {
                current = next;
                next = (*next).next;
            }

            // Remove the last element
            (*current).next = ptr::null_mut();
            self.tail = current;

            // Get the element and free the node
            let elem = Box::from_raw(next).elem;
            Some(elem)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_from_head() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_from_head()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        // Check empty list behaves right
        assert_eq!(list.pop_from_head(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop_from_head(), Some(1));
        assert_eq!(list.pop_from_head(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop_from_head(), Some(3));
        assert_eq!(list.pop_from_head(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_from_head(), Some(5));
        assert_eq!(list.pop_from_head(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop_from_head(), Some(6));
        assert_eq!(list.pop_from_head(), Some(7));
        assert_eq!(list.pop_from_head(), None);

        // extra fn
        list.push(1);
        list.push(2);
        assert_eq!(list.pop_from_tail_using_next_next(), Some(2));
        assert_eq!(list.pop_from_tail_using_prev(), Some(1));
        assert_eq!(list.pop_from_tail_using_next_next(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop_from_head() == Some(1));
        list.push(4);
        assert!(list.pop_from_head() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&30));
        assert!(list.pop_from_head() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop_from_head() == Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&5000));
        list.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}
