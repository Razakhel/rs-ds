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

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
  pub fn new(root: Option<Box<Node<T>>>) -> Self { List { root } }

  pub fn push(&mut self, value: T) {
    let new_node = Node {
      value,
      next: self.root.take()
    };

    self.root = Some(Box::new(new_node))
  }

  pub fn pop(&mut self) -> Option<T> {
    self.root.take().map(|node| {
      let temp_node = *node;
      self.root = temp_node.next;
      temp_node.value
    })
  }

  pub fn peek(&self) -> Option<&T> {
    self.root.as_ref().map(|node| {
      &node.value
    })
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.root.as_mut().map(|node| {
      &mut node.value
    })
  }

  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut curr_node = self.root.take();

    while let Some(mut node) = curr_node {
      curr_node = node.next.take();
    }
  }
}

#[cfg(test)]
mod test {
  use super::List;
  use super::Node;

  #[test]
  fn push_pop() {
    // Testing List<f32>
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

    // Testing List<&str>
    let mut list = List::new(Some(Box::new(Node::new("Hello",
                                                     Some(Box::new(Node::new("world",
                                                                             None)))))));

    let mut text = String::from(list.pop().unwrap());
    text.push_str(" ");
    text.push_str(list.pop().unwrap());
    assert_eq!(text, "Hello world");
  }

  #[test]
  fn peek() {
    let mut list: List<i32> = List::new(Some(Box::new(Node::new(42, None))));

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.peek_mut(), Some(&mut 42));
  }

  #[test]
  fn into_iter() {
    let mut list: List<i32> = List::new(None);
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
  }
}
