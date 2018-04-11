extern crate random_access_memory as ram;

#[test]
// Postmortem: looks like we were reading out of bounds by accidentally
// adding an offset to the index while reading.
fn regress_1() {
  let mut file = ram::Sync::new(50);
  file.write(30, &[30]).unwrap();
  file.read(15, 15).unwrap();
  assert!(file.opened);
}

#[test]
// Postmortem: probably a bug in our model.
fn regress_2() {
  let mut file = ram::Sync::new(50);
  file.write(22, &[22, 22, 22, 22]).unwrap();
  file.read(1, 4).unwrap();
  assert!(file.opened);

  let mut file = ram::Sync::new(50);
  file.write(48, &[48, 48, 48, 48]).unwrap();
  file.read(39, 9).unwrap();
  assert!(file.opened);
}

#[test]
// Postmortem: read is out of bounds.
fn regress_3() {
  let mut file = ram::Sync::new(50);
  file.write(21, &[]).unwrap();
  file.read(9, 3).unwrap();
  assert!(file.opened);
}