extern crate cc2_rust_workshop;

use cc2_rust_workshop::models::person::{Person};
use cc2_rust_workshop::helpers::math;
use cc2_rust_workshop::helpers::output;

fn main() {
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
}