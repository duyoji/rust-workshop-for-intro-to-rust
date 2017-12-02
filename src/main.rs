fn main() {
  let number_1 = 1;
  println!("{}", number_1);

  let mut number_2 = 1;
  number_2 += 1;
  println!("{}", number_2);

  let number_3 = add(1, 2);
  println!("{}", number_3);

  output_string("this is input value");

  let person = Person::new(
    "Code",
    "Chrysalis",
    2
  );
  println!("{:?}", person);
  person.greet();
}

fn add(a: i32, b: i32) -> i32 {
  a + b
}
#[test]
fn it_add() {
  assert_eq!(1 + 2, add(1, 2));
}

fn output_string(input: &str) -> () {
  println!("{}", input);
}
#[test]
fn it_output_string() {
  // Expected no error.
  output_string("abc");
  assert!(true);
}

#[derive(Debug)]
struct Person {
  first_name: String,
  last_name: String,
  age: u8
}

impl Person {
  fn new(first_name: &str, last_name: &str, age: u8) -> Person {
    Person {
      first_name: first_name.to_string(),
      last_name: last_name.to_string(),
      age: age
    }
  }

  fn greet(&self) -> () {
    println!(
      "My name is {} {}. I'm {} years old.",
      &self.first_name,
      &self.last_name,
      &self.age
    );
  }
}
#[test]
fn it_person() {
  let person = Person::new("FN", "LN", 1);
  assert_eq!("FN", person.first_name);
  assert_eq!("LN", person.last_name);
  assert_eq!(1, person.age);
}
#[test]
fn it_person_greet() {
  let person = Person::new("FN", "LN", 1);

  // Expected no error.
  person.greet();
  assert!(true);
}