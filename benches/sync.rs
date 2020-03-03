#![feature(test)]

mod sync {
  extern crate test;

  use random_access_memory as ram;
  use random_access_storage::RandomAccess;
  use test::Bencher;

  #[bench]
  fn write_hello_world(b: &mut Bencher) {
    b.iter(|| {
      async_std::task::block_on(async {
        let mut file = ram::RandomAccessMemory::default();
        file.write(0, b"hello").await.unwrap();
        file.write(5, b" world").await.unwrap();
      })
    });
  }

  #[bench]
  fn read_hello_world(b: &mut Bencher) {
    async_std::task::block_on(async {
      let mut file = ram::RandomAccessMemory::default();
      file.write(0, b"hello").await.unwrap();
      file.write(5, b" world").await.unwrap();
      b.iter(|| {
        async_std::task::block_on(async {
          let _text = file.read(0, 11).await.unwrap();
        })
      });
    });
  }

  #[bench]
  fn read_write_hello_world(b: &mut Bencher) {
    b.iter(|| {
      async_std::task::block_on(async {
        let mut file = ram::RandomAccessMemory::default();
        file.write(0, b"hello").await.unwrap();
        file.write(5, b" world").await.unwrap();
        let _text = file.read(0, 11).await.unwrap();
      })
    });
  }
}
