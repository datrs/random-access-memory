use random_access_memory as ram;
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

#[test]
fn can_len() {
  let mut file = ram::RandomAccessMemory::default();
  assert_eq!(file.len().unwrap(), 0);
  file.write(0, b"hello").unwrap();
  assert_eq!(file.len().unwrap(), 5);
  file.write(5, b" world").unwrap();
  assert_eq!(file.len().unwrap(), 11);
}

#[test]
fn can_is_empty() {
  let mut file = ram::RandomAccessMemory::default();
  assert_eq!(file.is_empty().unwrap(), true);
  file.write(0, b"hello").unwrap();
  assert_eq!(file.is_empty().unwrap(), false);
}
