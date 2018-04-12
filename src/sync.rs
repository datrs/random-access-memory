extern crate failure;
extern crate random_access_storage as random_access;

use failure::Error;
use std::cmp;

/// Main constructor.
pub struct Sync;
impl Sync {
  /// Create a new instance.
  #[cfg_attr(test, allow(new_ret_no_self))]
  pub fn new(page_size: usize) -> random_access::Sync<SyncMethods> {
    let methods = SyncMethods {
      buffers: Vec::new(),
      page_size,
      length: 0,
    };

    random_access::Sync::new(methods)
  }

  /// Create a new instance with a 1mb page size.
  // We cannot use the `Default` trait here because we aren't returning `Self`.
  pub fn default() -> random_access::Sync<SyncMethods> {
    let methods = SyncMethods {
      buffers: Vec::new(),
      page_size: 1024 * 1024,
      length: 0,
    };

    random_access::Sync::new(methods)
  }

  /// Create a new instance, but pass the initial buffers to the constructor.
  pub fn with_buffers(
    page_size: usize,
    buffers: Vec<Vec<u8>>,
  ) -> random_access::Sync<SyncMethods> {
    let methods = SyncMethods {
      page_size,
      buffers,
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
  // TODO: initialize as a sparse vector.
  pub buffers: Vec<Vec<u8>>,

  /// Total length of the data.
  length: usize,
}

impl random_access::SyncMethods for SyncMethods {
  fn open(&mut self) -> Result<(), Error> {
    Ok(())
  }

  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error> {
    let new_len = offset + data.len();
    if new_len > self.length {
      self.length = new_len;
    }

    let mut page_num = offset / self.page_size;
    let mut page_cursor = offset - (page_num * self.page_size);
    let mut data_cursor = 0;

    // Iterate over data, write to buffers. Subslice if the data is bigger than
    // what we can write in a single go.
    while data_cursor < data.len() {
      let data_bound = data.len() - data_cursor;
      let upper_bound = cmp::min(self.page_size, page_cursor + data_bound);
      let range = page_cursor..upper_bound;
      let range_len = range.len();

      // Allocate buffer if needed. Either append a new buffer to the end, or
      // set a buffer in the center.
      if self.buffers.get(page_num).is_none() {
        let buf = vec![0; self.page_size];
        if self.buffers.len() < page_num + 1 {
          self.buffers.resize(page_num + 1, buf);
        } else {
          self.buffers[page_num] = buf;
        }
      }

      // Copy data from the vec slice. This should always succeed.
      let buffer = self.buffers.get_mut(page_num).unwrap();
      for (index, buf_index) in range.enumerate() {
        buffer[buf_index] = data[data_cursor + index];
      }

      page_num += 1;
      page_cursor = 0;
      data_cursor += range_len;
    }

    Ok(())
  }

  fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, Error> {
    ensure!(
      (offset + length) <= self.length,
      "Could not satisfy length"
    );

    let mut page_num = offset / self.page_size;
    let mut page_cursor = offset - (page_num / self.page_size);

    let res_capacity = length;
    let mut res_buf = vec![0; length];
    let mut res_cursor = 0; // Keep track we read the right amount of bytes.

    while res_cursor < res_capacity {
      let res_bound = res_capacity - res_cursor;
      let page_bound = self.page_size - page_cursor;
      let relative_bound = cmp::min(res_bound, page_bound);
      let upper_bound = page_cursor + relative_bound;
      let range = page_cursor..upper_bound;

      // Fill until either we're done reading the page, or we're done
      // filling the buffer. Whichever arrives sooner.
      match self.buffers.get(page_num) {
        Some(buf) => {
          for (index, buf_index) in range.enumerate() {
            res_buf[res_cursor + index] = buf[buf_index];
          }
        }
        None => {
          for (index, _) in range.enumerate() {
            res_buf[res_cursor + index] = 0;
          }
        }
      }

      res_cursor += relative_bound;
      page_num += 1;
      page_cursor = 0;
    }

    Ok(res_buf)
  }

  fn del(&mut self, offset: usize, length: usize) -> Result<(), Error> {
    let overflow = offset % self.page_size;
    let inc = match overflow {
      0 => 0,
      _ => self.page_size - overflow,
    };

    if inc < length {
      let mut offset = offset + inc;
      let length = length - overflow;
      let end = offset + length;
      let mut i = offset - self.page_size;

      while (offset + self.page_size <= end) && i < self.buffers.capacity() {
        self.buffers.remove(i);
        offset += self.page_size;
        i += 1;
      }
    }

    Ok(())
  }
}
