pub fn add(a: i32, b: i32) -> i32 {
  a + b
}
#[test]
fn it_add() {
  assert_eq!(1 + 2, add(1, 2));
}