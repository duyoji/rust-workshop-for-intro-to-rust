extern crate cc2_rust_workshop;
extern crate serde_json;

use cc2_rust_workshop::models::person::{Person};
use cc2_rust_workshop::helpers::math;
use cc2_rust_workshop::helpers::output;

fn main() {
  // Basic requirement parts.
  let number_1 = 1;
  println!("{}", number_1);

  let mut number_2 = 1;
  number_2 += 1;
  println!("{}", number_2);

  let number_3 = math::add(1, 2);
  println!("{}", number_3);

  output::string("this is input value");

  let person = Person::new(
    "Code",
    "Chrysalis",
    2
  );
  println!("{:?}", person);
  person.greet();

  // Advanced requirements parts.

  // Convert the Point to a JSON string.
  let serialized_person = serde_json::to_string(&person).unwrap();
  println!("serialized_person = {}", serialized_person);

  // Convert the JSON string back to a Person.
  let deserialized_person: Person = serde_json::from_str(&serialized_person).unwrap();
  println!("deserialized_person = {:?}", deserialized_person);
}