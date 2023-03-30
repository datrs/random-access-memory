use random_access_memory as ram;
use random_access_storage::RandomAccess;

#[async_std::test]
async fn can_call_new() {
  let _file = ram::RandomAccessMemory::default();
}

#[async_std::test]
async fn can_open_buffer() {
  let mut file = ram::RandomAccessMemory::default();
  file.write(0, b"hello").await.unwrap();
}

#[async_std::test]
async fn can_write() {
  let mut file = ram::RandomAccessMemory::default();
  file.write(0, b"hello").await.unwrap();
  file.write(5, b" world").await.unwrap();
}

#[async_std::test]
async fn can_read() {
  let mut file = ram::RandomAccessMemory::default();
  file.write(0, b"hello").await.unwrap();
  file.write(5, b" world").await.unwrap();
  let text = file.read(0, 11).await.unwrap();
  let text = String::from_utf8(text.to_vec()).unwrap();
  assert_eq!(text, "hello world");
}

#[async_std::test]
async fn can_len() {
  let mut file = ram::RandomAccessMemory::default();
  assert_eq!(file.len().await.unwrap(), 0);
  file.write(0, b"hello").await.unwrap();
  assert_eq!(file.len().await.unwrap(), 5);
  file.write(5, b" world").await.unwrap();
  assert_eq!(file.len().await.unwrap(), 11);
}

#[async_std::test]
async fn can_is_empty() {
  let mut file = ram::RandomAccessMemory::default();
  assert!(file.is_empty().await.unwrap());
  file.write(0, b"hello").await.unwrap();
  assert!(!file.is_empty().await.unwrap());
}

#[async_std::test]
async fn can_delete() {
  assert_delete(100).await;
  assert_delete(10).await;
  assert_delete(5).await;
  assert_delete(2).await;
  assert_delete(1).await;
}

async fn assert_delete(page_size: usize) {
  let mut file = ram::RandomAccessMemory::new(page_size);
  file.write(0, b"hello").await.unwrap();
  file.write(5, b" world").await.unwrap();
  file.write(11, b" people").await.unwrap();
  assert_eq!(file.len().await.unwrap(), 18);
  file.del(6, 2).await.unwrap();
  assert_eq!(file.len().await.unwrap(), 18);
  let text = file.read(0, 6).await.unwrap();
  assert_eq!(String::from_utf8(text.to_vec()).unwrap(), "hello ");
  let zeros = file.read(6, 2).await.unwrap();
  assert_eq!(zeros, [0, 0]);
  let text = file.read(8, 10).await.unwrap();
  assert_eq!(String::from_utf8(text.to_vec()).unwrap(), "rld people");
  file.del(8, 4).await.unwrap();
  assert_eq!(file.len().await.unwrap(), 18);
  file.del(10, 8).await.unwrap();
  assert_eq!(file.len().await.unwrap(), 10);
}

#[async_std::test]
async fn can_truncate_lt() {
  assert_truncate_lt(100).await;
  assert_truncate_lt(10).await;
  assert_truncate_lt(5).await;
  assert_truncate_lt(2).await;
  assert_truncate_lt(1).await;
}

async fn assert_truncate_lt(page_size: usize) {
  let mut file = ram::RandomAccessMemory::new(page_size);
  file.write(0, b"hello").await.unwrap();
  file.write(5, b" world").await.unwrap();
  file.write(11, b" people").await.unwrap();
  assert_eq!(file.len().await.unwrap(), 18);
  file.truncate(7).await.unwrap();
  assert_eq!(file.len().await.unwrap(), 7);
  let text = file.read(0, 7).await.unwrap();
  assert_eq!(String::from_utf8(text.to_vec()).unwrap(), "hello w");
  if file.read(0, 8).await.is_ok() {
    panic!("storage is too big. read past the end should have failed");
  };
  file.write(11, b" people").await.unwrap();
  assert_eq!(file.len().await.unwrap(), 18);
  let zeros = file.read(7, 4).await.unwrap();
  assert_eq!(zeros, [0, 0, 0, 0]);
}

#[async_std::test]
async fn can_truncate_gt() {
  assert_truncate_gt(100).await;
  assert_truncate_gt(10).await;
  assert_truncate_gt(5).await;
  assert_truncate_gt(2).await;
  assert_truncate_gt(1).await;
}

async fn assert_truncate_gt(page_size: usize) {
  let mut file = ram::RandomAccessMemory::new(page_size);
  file.write(0, b"hello").await.unwrap();
  file.write(5, b" world").await.unwrap();
  file.write(11, b" people").await.unwrap();
  assert_eq!(file.len().await.unwrap(), 18);
  file.truncate(22).await.unwrap();
  assert_eq!(file.len().await.unwrap(), 22);
  let zeros = file.read(18, 4).await.unwrap();
  assert_eq!(zeros, [0, 0, 0, 0]);
  file.write(19, &[1]).await.unwrap();
  let written = file.read(18, 4).await.unwrap();
  assert_eq!(written, [0, 1, 0, 0]);
}

#[async_std::test]
async fn assert_truncate_eq() {
  let mut file = ram::RandomAccessMemory::new(5);
  file.write(0, b"hello").await.unwrap();
  file.write(5, b" world").await.unwrap();
  file.write(11, b" people").await.unwrap();
  assert_eq!(file.len().await.unwrap(), 18);
  file.truncate(18).await.unwrap();
  assert_eq!(file.len().await.unwrap(), 18);
}
