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

  pub fn pop(&mut self) -> Option<T> {
    match mem::replace(&mut self.root, None) {
      Some(node) => {
        let temp_node = *node;
        self.root = temp_node.next;
        Some(temp_node.value)
      },
      None => None
    }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut curr_node = mem::replace(&mut self.root, None);

    while let Some(mut node) = curr_node {
      curr_node = mem::replace(&mut node.next, None);
    }
  }
}

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn tests() {
    let mut list: List<f32> = List::new(None);

    assert_eq!(list.pop(), None);

    list.push(0.0f32);
    list.push(43723.0f32);
    list.push(2.134f32);
    list.push(42.0f32);

    assert_eq!(list.pop(), Some(42.0f32));
    assert_eq!(list.pop(), Some(2.134f32));

    list.push(123.456f32);
    list.push(0.00001f32);

    assert_eq!(list.pop(), Some(0.00001f32));
    assert_eq!(list.pop(), Some(123.456f32));
    assert_eq!(list.pop(), Some(43723.0f32));
    assert_eq!(list.pop(), Some(0.0f32));
    assert_eq!(list.pop(), None);
  }
}
