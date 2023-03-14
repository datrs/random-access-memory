use criterion::{criterion_group, criterion_main, Criterion};
use random_access_memory as ram;
use random_access_storage::RandomAccess;

fn write_hello_world(c: &mut Criterion) {
  c.bench_function("write hello world", |b| {
    b.iter(|| {
      async_std::task::block_on(async {
        let mut file = ram::RandomAccessMemory::default();
        file.write(0, b"hello").await.unwrap();
        file.write(5, b" world").await.unwrap();
      })
    })
  });
}

fn read_hello_world(c: &mut Criterion) {
  c.bench_function("read hello world", |b| {
    async_std::task::block_on(async {
      let mut file = ram::RandomAccessMemory::default();
      file.write(0, b"hello").await.unwrap();
      file.write(5, b" world").await.unwrap();
      b.iter(|| {
        async_std::task::block_on(async {
          let _text = file.read(0, 11).await.unwrap();
        })
      });
    })
  });
}

fn read_write_hello_world(c: &mut Criterion) {
  c.bench_function("read/write hello world", |b| {
    b.iter(|| {
      async_std::task::block_on(async {
        let mut file = ram::RandomAccessMemory::default();
        file.write(0, b"hello").await.unwrap();
        file.write(5, b" world").await.unwrap();
        let _text = file.read(0, 11).await.unwrap();
      })
    })
  });
}

fn write_del_hello_world(c: &mut Criterion) {
  c.bench_function("write/del hello world", |b| {
    b.iter(|| {
      async_std::task::block_on(async {
        let mut file = ram::RandomAccessMemory::default();
        file.write(0, b"hello world").await.unwrap();
        file.del(0, 5).await.unwrap();
        file.del(5, 6).await.unwrap();
      })
    })
  });
}

criterion_group!(
  benches,
  write_hello_world,
  read_hello_world,
  read_write_hello_world,
  write_del_hello_world,
);
criterion_main!(benches);
