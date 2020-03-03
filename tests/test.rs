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
  assert_eq!(file.is_empty().await.unwrap(), true);
  file.write(0, b"hello").await.unwrap();
  assert_eq!(file.is_empty().await.unwrap(), false);
}
