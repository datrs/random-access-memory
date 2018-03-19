extern crate random_access_storage as random_access;

use self::random_access::Error;

/// Main constructor.
pub struct Sync;
impl Sync {
  /// Create a new instance.
  pub fn new() -> random_access::Sync<SyncMethods> {
    let methods = SyncMethods {
      page_size: 1024 * 1024,
      buffers: Vec::new(),
      length: 0,
    };

    random_access::Sync::new(methods)
  }

  /// Create a new instance, but pass the initial buffers to the constructor.
  pub fn with_buffers(
    buffers: Vec<Vec<u8>>,
  ) -> random_access::Sync<SyncMethods> {
    let methods = SyncMethods {
      page_size: 1024 * 1024,
      buffers: buffers,
      length: 0,
    };

    random_access::Sync::new(methods)
  }
}

/// Methods that have been implemented to provide synchronous access to memory
/// buffers. These should generally be kept private, but exposed to prevent
/// leaking internals.
pub struct SyncMethods {
  /// The length length of each buffer.
  pub page_size: usize,

  /// The memory we read/write to.
  pub buffers: Vec<Vec<u8>>,

  /// Total length of the data.
  length: u64,
}

impl random_access::SyncMethods for SyncMethods {
  /// Open the instance (noop).
  fn open(&self) -> Result<(), Error> {
    Ok(())
  }

  /// Write bytes to memory.
  fn write(&self, offset: u64, data: Vec<u8>) -> Result<(), Error> {}

  /// Read bytes from memory.
  fn read(&self, offset: u64, length: u64) -> Result<Vec<u8>, Error> {}

  /// Delete bytes from memory.
  fn del(&self, offset: u64, length: u64) -> Result<(), Error> {}
}
