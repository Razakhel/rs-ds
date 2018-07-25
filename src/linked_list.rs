use std::mem;

#[derive(Debug)]
pub struct Node<T> {
  value: T,
  next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
  pub fn new(value: T, next: Option<Box<Node<T>>>) -> Self { Node { value, next } }
}

#[derive(Debug)]
pub struct List<T> {
  root: Option<Box<Node<T>>>
}

impl<T> List<T> {
  pub fn new(root: Option<Box<Node<T>>>) -> Self { List { root } }

  pub fn push(&mut self, value: T) {
    let new_node = Node {
      value,
      next: mem::replace(&mut self.root, None)
    };

    self.root = Some(Box::new(new_node))
  }
}
