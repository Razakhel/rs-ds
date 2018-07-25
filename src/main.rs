mod linked_list;

use linked_list::List;
use linked_list::Node;

fn main() {
  let list = List::new(Some(Box::new(Node::new(3.25f32, None))));

  println!("{:?}", list);
}
