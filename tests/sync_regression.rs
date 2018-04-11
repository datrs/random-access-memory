extern crate random_access_memory as ram;

#[test]
fn regress_1() {
  let mut file = ram::Sync::new(50);
  file.write(30, &[30]).unwrap();
  file.read(15, 15).unwrap();
  assert!(file.opened);
}
