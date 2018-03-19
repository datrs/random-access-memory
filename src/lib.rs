#![deny(warnings, missing_docs)]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

//! Continuously read,write to memory using random offsets and lengths. Adapted
//! from
//! [mafintosh/random-access-memory](https://github.com/mafintosh/random-access-memory).

/// Main constructor. Creates a new `RandomAccessMemory` instance.
pub struct RandomAccessMemory {
  /// The length length of each buffer.
  pub page_size: usize,

  /// Memory we read/write to.
  pub buffers: Vec<Vec<u8>>,

  /// Total length of the data.
  pub length: u64,
}

impl RandomAccessMemory {
  /// Create a new `RandomAccessMemory` instance.
  pub fn new() {}

  /// Read bytes from memory.
  pub fn read() {}

  /// Write bytes to memory.
  pub fn write() {}

  /// Delete bytes from memory.
  pub fn del() {}
}
