/// A synchronous implementation of `random_access_memory`.
pub struct Sync {
  /// The length length of each buffer.
  pub page_size: usize,

  /// Memory we read/write to.
  pub buffers: Vec<Vec<u8>>,

  /// Total length of the data.
  pub length: u64,
}

impl Sync {
  /// Create a new `Sync` instance.
  pub fn new() -> Self {
    Sync {
      page_size: 1024 * 1024,
      buffers: Vec::new(),
      length: 0,
    }
  }

  /// Write bytes to memory.
  // pub fn write(offset: u64, data: Vec<u8>) {}

  /// Read bytes from memory.
  pub fn read() {}

  /// Delete bytes from memory.
  pub fn del() {}
}
