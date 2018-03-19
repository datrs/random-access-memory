extern crate random_access_memory as ram;

#[test]
fn can_call_new() {
  let _file = ram::sync::new();
}
