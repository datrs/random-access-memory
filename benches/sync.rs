#![feature(test)]

mod sync {
  extern crate random_access_memory as ram;
  extern crate test;

  use self::test::Bencher;

  #[bench]
  fn write_hello_world(b: &mut Bencher) {
    b.iter(|| {
      let mut file = ram::RandomAccessMemory::default();
      file.write(0, b"hello").unwrap();
      file.write(5, b" world").unwrap();
    });
  }

  #[bench]
  fn read_hello_world(b: &mut Bencher) {
    let mut file = ram::RandomAccessMemory::default();
    file.write(0, b"hello").unwrap();
    file.write(5, b" world").unwrap();
    b.iter(|| {
      let _text = file.read(0, 11).unwrap();
    });
  }

  #[bench]
  fn read_write_hello_world(b: &mut Bencher) {
    b.iter(|| {
      let mut file = ram::RandomAccessMemory::default();
      file.write(0, b"hello").unwrap();
      file.write(5, b" world").unwrap();
      let _text = file.read(0, 11).unwrap();
    });
  }
}
