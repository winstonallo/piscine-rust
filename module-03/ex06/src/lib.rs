use std::ops::{Index, IndexMut};

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }

    fn new_with_next(value: T, next: Box<Node<T>>) -> Self {
        Self { value, next: Some(next) }
    }
}

#[derive(Clone, Default)]
struct List<T> {
    head: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push_front(&mut self, value: T) {
        if let Some(head) = self.head.take() {
            self.head = Some(Box::new(Node::new_with_next(value, head)));
        } else {
            self.head = Some(Box::new(Node::new(value)));
        }
    }
    fn push_back(&mut self, value: T) {
        let mut it = &mut self.head;
        while let Some(node) = it {
            it = &mut node.next;
        }
        *it = Some(Box::new(Node::new(value)));
    }

    fn count(&self) -> usize {
        let mut count = 0;
        let mut it = &self.head;

        while let Some(node) = it {
            it = &node.next;
            count += 1;
        }
        count 
    }

    fn get(&self, i: usize) -> Option<&T> {
        let mut idx = 0;
        let mut it = &self.head;

        while let Some(node) = it {
            if idx == i {
                return Some(&node.value);
            }
            it = &node.next;
            idx += 1;
        }
        None
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        let mut idx = 0;
        let mut it = &mut self.head;

        while let Some(node) = it {
            if idx == i {
                return Some(&mut node.value);
            }
            it = &mut node.next;
            idx += 1;
        }
        None
    }

    fn remove_front(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.next;
            Some(head.value)
        } else {
            None
        }
    }
    fn remove_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let mut it = &mut self.head;
        while it.is_some() {
            if it.as_ref().unwrap().next.is_none() {
                break;
            }
            it = &mut it.as_mut().unwrap().next;
        }

        match it.take() {
            Some(node) => Some(node.value),
            None => None,
        }
    }

    fn clear(&mut self) {
        self.head = None;
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match self.get(idx) {
            Some(val) => val,
            None => panic!("tried to access out of bound index {idx}"),
        }
    }
}

impl<T> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match self.get_mut(idx) {
            Some(val) => val,
            None => panic!("tried to access out of bound index {idx}"),
        }
    }
}

#[cfg(test)]
#[test]
fn default_list_is_empty() {
    let list: List<i32> = Default::default();
    assert_eq!(list.count(), 0);
}

#[cfg(test)]
#[test]
fn cloned_list_are_equal() {
    let mut list = List::new();
    list.push_back(String::from("Hello"));
    list.push_back(String::from("World"));

    let cloned = list.clone();
    assert_eq!(cloned.count(), list.count());
    assert_eq!(&cloned[0], &cloned[0]);
    assert_eq!(&cloned[1], &cloned[1]);
}

#[cfg(test)]
#[test]
#[should_panic(expected = "tried to access out of bound index 10")]
fn out_of_bound_access_panics() {
    let mut list: List<u32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list[10], 42);
}