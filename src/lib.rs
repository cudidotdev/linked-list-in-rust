#![allow(dead_code)]
use std::rc::Rc;

#[derive(Debug)]
pub struct LinkedList<T> {
  head: Option<Rc<Node<T>>>,
  len: usize,
}

#[derive(Debug)]
struct Node<T> {
  data: T,
  next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
  pub fn insert_ahead(&mut self, data: T) {
    let new_node = Node {
      data,
      next: self.next.clone(),
    };

    self.next = Some(Rc::new(new_node))
  }

  pub fn delete_ahead(&mut self) {
    if let Some(next_node) = &self.next {
      self.next = next_node.next.clone()
    }
  }
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    LinkedList { head: None, len: 0 }
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn head(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.data)
  }

  pub fn push(&mut self, data: T) {
    let new_node = Node {
      data,
      next: self.head.clone(),
    };

    self.head = Some(Rc::new(new_node));

    self.len += 1;
  }

  pub fn pop(&mut self) {
    if let Some(head_node) = &self.head {
      self.head = head_node.next.clone();
      self.len -= 1;
    }
  }

  pub fn iter(&self) -> Iter<T> {
    Iter {
      current: &self.head,
    }
  }

  pub fn contains(&self, element: T) -> bool
  where
    T: PartialEq,
  {
    self.iter().any(|item| item == &element)
  }

  pub fn insert(&mut self, pos: usize, data: T) {
    if pos == 0 {
      return self.push(data);
    }

    if pos > self.len {
      return;
    }

    let left_node = self.find_node_as_mut(pos - 1);

    if let Some(node) = left_node {
      node.insert_ahead(data);
      self.len += 1;
    }
  }

  pub fn remove(&mut self, pos: usize) {
    if pos == 0 {
      return self.pop();
    }

    if pos >= self.len {
      return;
    }

    let left_node = self.find_node_as_mut(pos - 1);

    if let Some(node) = left_node {
      node.delete_ahead();
      self.len -= 1;
    }
  }

  fn find_node_as_mut(&mut self, pos: usize) -> Option<&mut Node<T>> {
    if pos >= self.len {
      return None;
    }

    let mut i = pos;
    let mut current = &mut self.head;

    while i > 0 {
      let rc = current.as_mut()?;

      current = &mut Rc::get_mut(rc)?.next;

      i -= 1;
    }

    Rc::get_mut(current.as_mut()?)
  }

  pub fn reverse(&self) -> LinkedList<T>
  where
    T: Clone,
  {
    let mut list = LinkedList::new();

    self.iter().for_each(|data| list.push(data.clone()));

    list
  }
}

pub struct Iter<'a, T> {
  current: &'a Option<Rc<Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    let current = self.current.as_ref();

    if let Some(node) = current {
      self.current = &node.next;
    }

    current.map(|node| &node.data)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn new() {
    let list: LinkedList<i32> = LinkedList::new();

    assert_eq!(list.len(), 0);
    assert_eq!(list.head(), None);
  }

  #[test]
  fn push() {
    let mut list = LinkedList::new();

    list.push(1);

    assert_eq!(list.head(), Some(&1));
    assert_eq!(list.len(), 1);

    println!("{list:#?}");

    list.push(2);

    assert_eq!(list.head(), Some(&2));
    assert_eq!(list.len(), 2);

    println!("{list:#?}");

    list.push(3);

    assert_eq!(list.head(), Some(&3));
    assert_eq!(list.len(), 3);

    println!("{list:#?}");
  }

  #[test]
  fn pop() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.head(), Some(&3));
    assert_eq!(list.len(), 3);

    println!("{list:#?}");

    list.pop();

    assert_eq!(list.head(), Some(&2));
    assert_eq!(list.len(), 2);

    println!("{list:#?}");

    list.pop();

    assert_eq!(list.head(), Some(&1));
    assert_eq!(list.len(), 1);

    println!("{list:#?}");

    list.pop();

    assert_eq!(list.head(), None);
    assert_eq!(list.len(), 0);

    println!("{list:#?}");

    list.pop();

    assert_eq!(list.head(), None);
    assert_eq!(list.len(), 0);

    println!("{list:#?}");
  }

  #[test]
  fn iter() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    println!("{list:#?}");

    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&3));

    println!("{list:#?}");

    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    println!("{list:#?}");
  }

  #[test]
  fn contains() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    assert!(list.contains(4));
    assert!(list.contains(1));
    assert!(!list.contains(0));
  }

  #[test]
  fn insert() {
    let mut list = LinkedList::new();

    list.push(2);
    list.push(4);

    println!("{list:#?}");

    list.insert(1, 3);
    list.insert(3, 1);
    list.insert(0, 5);

    println!("{list:#?}");

    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn remove() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);

    println!("{list:#?}");

    list.remove(2);

    println!("{list:#?}");

    list.remove(0);

    println!("{list:#?}");

    list.remove(2);

    println!("{list:#?}");

    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn reverse() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    let new_list = list.reverse();

    let mut iter = new_list.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);

    println!("{list:#?}");
    println!("{new_list:#?}");
  }
}
