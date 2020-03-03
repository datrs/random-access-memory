use random_access_memory as ram;
use random_access_storage::RandomAccess;

#[async_std::test]
// Postmortem: looks like we were reading out of bounds by accidentally
// adding an offset to the index while reading.
async fn regress_1() {
  let mut file = ram::RandomAccessMemory::new(50);
  file.write(30, &[30]).await.unwrap();
  file.read(15, 15).await.unwrap();
}

#[async_std::test]
// Postmortem: our buffers weren't zero-filled. Intead we were relying on
// uninitialized (but claimed!) memory, which caused all sorts of weirdness.
async fn regress_2() {
  let mut file = ram::RandomAccessMemory::new(50);
  file.write(22, &[22, 22, 22, 22]).await.unwrap();
  let buf = file.read(1, 4).await.unwrap();
  assert_eq!(buf, vec![0, 0, 0, 0]);

  let mut file = ram::RandomAccessMemory::new(50);
  file.write(48, &[48, 48, 48, 48]).await.unwrap();
  let buf = file.read(39, 9).await.unwrap();
  assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[async_std::test]
// Postmortem: the way we were reading was off. We were messing up both reading
// and writing. We now keep two cursors, and compute the bounds of every loop
// ahead of time. Also simplified our allocation logic.
async fn regress_3() {
  let mut file = ram::RandomAccessMemory::new(50);
  file.write(45, &[56, 46, 14, 93, 15, 54, 2]).await.unwrap();
  let buf = file.read(42, 10).await.unwrap();
  assert_eq!(buf, vec![0, 0, 0, 56, 46, 14, 93, 15, 54, 2]);
}

#[async_std::test]
// Postmortem: we were having trouble when we were reading with an index that's
// larger than the page size. Turned out we weren't doing some math properly,
// which caused a cursor to jump.
async fn regress_4() {
  let mut file = ram::RandomAccessMemory::new(10);
  file.write(44, &[54, 59]).await.unwrap();
  file.read(13, 3).await.unwrap();
}
