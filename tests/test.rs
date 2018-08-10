extern crate random_access_memory as ram;
extern crate random_access_storage;

use random_access_storage::RandomAccess;

#[test]
fn can_call_new() {
  let _file = ram::RandomAccessMemory::default();
}

#[test]
fn can_open_buffer() {
  let mut file = ram::RandomAccessMemory::default();
  file.write(0, b"hello").unwrap();
}

#[test]
fn can_write() {
  let mut file = ram::RandomAccessMemory::default();
  file.write(0, b"hello").unwrap();
  file.write(5, b" world").unwrap();
}

#[test]
fn can_read() {
  let mut file = ram::RandomAccessMemory::default();
  file.write(0, b"hello").unwrap();
  file.write(5, b" world").unwrap();
  let text = file.read(0, 11).unwrap();
  let text = String::from_utf8(text.to_vec()).unwrap();
  assert_eq!(text, "hello world");
}
