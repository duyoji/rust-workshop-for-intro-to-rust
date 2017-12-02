pub fn string(input: &str) -> () {
  println!("{}", input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_output_string() {
    // Expected no error.
    string("abc");
    assert!(true);
  }
}