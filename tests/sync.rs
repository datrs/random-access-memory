extern crate random_access_memory as ram;

#[test]
fn can_call_new() {
  let _file = ram::Sync::default();
}

#[test]
fn can_open_buffer() {
  let mut file = ram::Sync::default();
  file.write(0, b"hello").unwrap();
  assert!(file.opened);
}

#[test]
fn can_write() {
  let mut file = ram::Sync::default();
  file.write(0, b"hello").unwrap();
  file.write(5, b" world").unwrap();
}

#[test]
fn can_read() {
  let mut file = ram::Sync::default();
  file.write(0, b"hello").unwrap();
  file.write(5, b" world").unwrap();
  let text = file.read(0, 11).unwrap();
  let text = String::from_utf8(text.to_vec()).unwrap();
  assert_eq!(text, "hello world");
}
