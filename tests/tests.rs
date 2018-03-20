mod sync {
  extern crate random_access_memory as ram;

  #[test]
  fn can_call_new() {
    let _file = ram::Sync::new();
  }

  #[test]
  fn can_open_buffer() {
    let mut file = ram::Sync::new();
    file.write(0, b"hello").unwrap();
    assert!(file.opened);
  }

  #[test]
  fn can_write() {
    let mut file = ram::Sync::new();
    file.write(0, b"hello").unwrap();
    file.write(5, b" world").unwrap();
  }
}
