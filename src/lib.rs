#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate failure;
extern crate random_access_storage;

use failure::Error;
use random_access_storage::RandomAccess;
use std::cmp;
use std::io;

/// Main constructor.
#[derive(Debug)]
pub struct RandomAccessMemory {
  /// The length length of each buffer.
  page_size: usize,

  /// The memory we read/write to.
  // TODO: initialize as a sparse vector.
  buffers: Vec<Vec<u8>>,

  /// Total length of the data.
  length: u64,
}

impl RandomAccessMemory {
  /// Create a new instance.
  pub fn new(page_size: usize) -> Self {
    RandomAccessMemory {
      buffers: Vec::new(),
      page_size,
      length: 0,
    }
  }

  /// Create a new instance with a 1mb page size.
  // We cannot use the `Default` trait here because we aren't returning `Self`.
  pub fn default() -> Self {
    RandomAccessMemory {
      buffers: Vec::new(),
      page_size: 1024 * 1024,
      length: 0,
    }
  }

  /// Create a new instance, but pass the initial buffers to the constructor.
  pub fn with_buffers(page_size: usize, buffers: Vec<Vec<u8>>) -> Self {
    RandomAccessMemory {
      page_size,
      buffers,
      length: 0,
    }
  }
}

impl RandomAccess for RandomAccessMemory {
  type Error = Error;

  fn write(&mut self, offset: u64, data: &[u8]) -> Result<(), Self::Error> {
    let new_len = offset + data.len() as u64;
    if new_len > self.length {
      self.length = new_len;
    }

    let mut page_num = (offset / self.page_size as u64) as usize;
    let mut page_cursor =
      (offset - (page_num * self.page_size) as u64) as usize;
    let mut data_cursor = 0;

    // Iterate over data, write to buffers. Subslice if the data is bigger than
    // what we can write in a single go.
    while data_cursor < data.len() {
      let data_bound = data.len() - data_cursor;
      let upper_bound = cmp::min(self.page_size, page_cursor + data_bound);
      let range = page_cursor..upper_bound;
      let range_len = (page_cursor as usize..upper_bound as usize).len();

      // Allocate buffer if needed. Either append a new buffer to the end, or
      // set a buffer in the center.
      if self.buffers.get(page_num).is_none() {
        let buf = vec![0; self.page_size as usize];
        if self.buffers.len() < page_num + 1 {
          self.buffers.resize(page_num + 1, buf);
        } else {
          self.buffers[page_num] = buf;
        }
      }

      // Copy data from the vec slice.
      // TODO: use a batch operation such as `.copy_from_slice()` so it can be
      // optimized.
      let buffer = &mut self.buffers[page_num as usize];
      for (index, buf_index) in range.enumerate() {
        buffer[buf_index as usize] = data[data_cursor + index];
      }

      page_num += 1;
      page_cursor = 0;
      data_cursor += range_len;
    }

    Ok(())
  }

  fn sync_all(&mut self) -> Result<(), Self::Error> {
    Ok(())
  }

  fn read(&mut self, offset: u64, length: u64) -> Result<Vec<u8>, Self::Error> {
    ensure!(
      (offset + length) <= self.length,
      format!(
        "Read bounds exceeded. {} < {}..{}",
        self.length,
        offset,
        offset + length
      )
    );

    let mut page_num = (offset / self.page_size as u64) as usize;
    let mut page_cursor =
      (offset - (page_num * self.page_size) as u64) as usize;

    let mut res_buf = vec![0; length as usize];
    let mut res_cursor = 0; // Keep track we read the right amount of bytes.
    let res_capacity = length;

    while res_cursor < res_capacity {
      let res_bound = res_capacity - res_cursor;
      let page_bound = self.page_size - page_cursor;
      let relative_bound = cmp::min(res_bound, page_bound as u64);
      let upper_bound = page_cursor + relative_bound as usize;
      let range = page_cursor..upper_bound;

      // Fill until either we're done reading the page, or we're done
      // filling the buffer. Whichever arrives sooner.
      match self.buffers.get(page_num as usize) {
        Some(buf) => {
          for (index, buf_index) in range.enumerate() {
            res_buf[res_cursor as usize + index] = buf[buf_index as usize];
          }
        }
        None => {
          for (index, _) in range.enumerate() {
            res_buf[res_cursor as usize + index] = 0;
          }
        }
      }

      res_cursor += relative_bound;
      page_num += 1;
      page_cursor = 0;
    }

    Ok(res_buf)
  }

  fn read_to_writer(
    &mut self,
    _offset: u64,
    _length: u64,
    _buf: &mut impl io::Write,
  ) -> Result<(), Self::Error> {
    unimplemented!()
  }

  fn del(&mut self, offset: u64, length: u64) -> Result<(), Self::Error> {
    let overflow = offset % self.page_size as u64;
    let inc = match overflow {
      0 => 0,
      _ => self.page_size as u64 - overflow,
    };

    if inc < length {
      let mut offset = offset + inc;
      let length = length - overflow;
      let end = offset + length;
      let mut i = offset - self.page_size as u64;

      while (offset + self.page_size as u64 <= end)
        && i < self.buffers.capacity() as u64
      {
        self.buffers.remove(i as usize);
        offset += self.page_size as u64;
        i += 1;
      }
    }

    Ok(())
  }

  fn truncate(&mut self, _length: u64) -> Result<(), Self::Error> {
    unimplemented!()
  }

  fn len(&self) -> Result<u64, Self::Error> {
    Ok(self.length)
  }

  fn is_empty(&mut self) -> Result<bool, Self::Error> {
    Ok(self.length == 0)
  }
}
