mod linked_list;

use linked_list::List;
use linked_list::Node;

fn main() {
  let mut list = List::new(Some(Box::new(Node::new(3.25f32, None))));

  list.push(34.0f32);
  list.push(2.134f32);
  list.push(12.04f32);
  list.push(85.01f32);
  list.push(1.91f32);
  list.push(43723.0f32);
  list.push(321.2f32);
  list.push(853.654f32);
  list.push(59271.4f32);

  let val = list.pop();
  println!("Popped value: {}", val.unwrap());
  let val = list.pop();
  println!("Popped value: {}", val.unwrap());
  let val = list.pop();
  println!("Popped value: {}", val.unwrap());
  let val = list.pop();
  println!("Popped value: {}", val.unwrap());
  let val = list.pop();
  println!("Popped value: {}", val.unwrap());
  let val = list.pop();
  println!("Popped value: {}", val.unwrap());
  let val = list.pop();
  println!("Popped value: {}", val.unwrap());
  let val = list.pop();
  println!("Popped value: {}", val.unwrap());

  println!("{:?}", list);
}
