mod sync {
  extern crate random_access_memory as ram;

  #[test]
  fn can_call_new() {
    let _file = ram::Sync::new();
  }

  #[test]
  fn can_open_buffer_noop() {
    let file = ram::Sync::new();
    assert!(file.open().is_ok());
  }

  #[test]
  fn can_write() {}
}
