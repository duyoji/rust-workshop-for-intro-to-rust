pub fn string(input: &str) -> () {
  println!("{}", input);
}
#[test]
fn it_output_string() {
  // Expected no error.
  string("abc");
  assert!(true);
}