#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
  first_name: String,
  last_name: String,
  age: u8
}

impl Person {
  pub fn new(first_name: &str, last_name: &str, age: u8) -> Person {
    Person {
      first_name: first_name.to_string(),
      last_name: last_name.to_string(),
      age: age
    }
  }

  pub fn greet(&self) -> () {
    println!(
      "My name is {} {}. I'm {} years old.",
      &self.first_name,
      &self.last_name,
      &self.age
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
}