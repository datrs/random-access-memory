extern crate failure;
extern crate random_access_storage as random_access;

use failure::Error;
use std::cmp;

/// Main constructor.
pub struct Sync;
impl Sync {
  /// Create a new instance.
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
  length: usize,
}

impl random_access::SyncMethods for SyncMethods {
  fn open(&mut self) -> Result<(), Error> {
    Ok(())
  }

  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error> {
    if (offset + data.len()) > self.length {
      self.length = offset + data.len();
    }

    let mut data = data;
    let mut i = offset / self.page_size;
    let mut rel = offset - (i * self.page_size);

    // Iterate over data, write to buffers.
    while data.len() > 0 {
      let next = if (rel + data.len()) > self.page_size {
        &data[..(self.page_size - rel)]
      } else {
        data
      };

      // Allocate buffer if none matches
      if let &None = &self.buffers.get(i) {
        let buf = if (rel == 0) && (next.len() == self.page_size) {
          next.to_vec()
        } else {
          calloc(self.page_size)
        };

        // Grow self.buffers if needed.
        if self.buffers.len() < i + 1 {
          self.buffers.resize(i + 1, buf);
        } else {
          self.buffers[i] = buf;
        }
      }

      // NOTE: we need to match Some in this case,
      // alternatively we could use the `unsafe`
      // `get_unchecked` method, but yeah nah.
      if let Some(buffer) = self.buffers.get_mut(i) {
        for i in 0..next.len() {
          let byte = next[i];
          &buffer.push(byte);
        }
      }

      i += 1;
      rel = 0;
      data = &data[next.len()..];
    }

    Ok(())
  }

  fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, Error> {
    ensure!(
      (offset + length) <= self.length,
      "Could not satisfy length"
    );

    let mut data = Vec::with_capacity(length);
    let mut ptr = 0;
    let mut i = offset / self.page_size;
    let mut rel = offset - (i / self.page_size);

    while ptr < data.capacity() {
      let len = self.page_size - rel;
      match &self.buffers.get(i) {
        &Some(buf) => for i in ptr..buf.len() {
          data.push(buf[rel + i]);
        },
        &None => {
          let max = cmp::min(data.capacity(), ptr + len);
          for i in ptr..max {
            data[i] = 0;
          }
        }
      }

      ptr += len;
      i += 1;
      rel = 0;
    }

    Ok(data)
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

#[inline]
fn calloc(len: usize) -> Vec<u8> {
  Vec::with_capacity(len)
}
