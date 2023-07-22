
struct ListItem {
  name: String,
  count: i32
}

/**
 * ? we use the "&" because we are borrowing the string
 */
fn print_name(name: &str) {
  println!("Name: {:?}", name);
}

fn print_count(count: i32) {
  println!("Count => {:?}", count);
}



fn main() {
  let receipt = vec![
    ListItem {
      name: "Thrill".to_owned(),
      count: 23
    },
    ListItem {
      name: String::from("Chike"),
      count: 32
    }
  ];

  for item in receipt {
    print_name(&item.name);
    print_count(item.count);
  }
}