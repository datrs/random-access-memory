#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

pub use intmap::IntMap;

use random_access_storage::{RandomAccess, RandomAccessError};
use std::cmp;

/// Main constructor.
#[derive(Debug)]
pub struct RandomAccessMemory {
  /// The length length of each buffer.
  page_size: usize,

  /// The memory we read/write to.
  buffers: IntMap<Vec<u8>>,

  /// Total length of the data.
  length: u64,
}

impl Default for RandomAccessMemory {
  /// Create a new instance with a 1mb page size.
  fn default() -> Self {
    RandomAccessMemory::new(1024 * 1024)
  }
}

impl RandomAccessMemory {
  /// Create a new instance.
  pub fn new(page_size: usize) -> Self {
    RandomAccessMemory::with_buffers(page_size, IntMap::new())
  }

  /// Create a new instance, but pass the initial buffers to the constructor.
  pub fn with_buffers(page_size: usize, buffers: IntMap<Vec<u8>>) -> Self {
    RandomAccessMemory {
      page_size,
      buffers,
      length: 0,
    }
  }

  /// Returns the page number and index within that page for a given offset.
  /// If `exclusive_end` is true, when hitting the exact border of two pages
  /// gives the previous page and page size as index.
  fn page_num_and_index(
    &self,
    offset: u64,
    exclusive_end: bool,
  ) -> (usize, usize) {
    let page_num = (offset / (self.page_size as u64)) as usize;
    let page_index = (offset % (self.page_size as u64)) as usize;
    if page_index == 0 && exclusive_end {
      (if page_num > 0 { page_num - 1 } else { 0 }, self.page_size)
    } else {
      (page_num, page_index)
    }
  }

  fn zero(&mut self, offset: u64, length: u64) {
    let (first_page_num, first_page_start) =
      self.page_num_and_index(offset, false);
    let (last_page_num, last_page_end) =
      self.page_num_and_index(offset + length, true);

    // Check if we need to zero bytes in the first page
    if first_page_start > 0
      || (first_page_num == last_page_num && last_page_end > 0)
    {
      if let Some(page) = self.buffers.get_mut(first_page_num as u64) {
        // Need to zero part of the first page
        let begin_page_end = first_page_start
          + cmp::min(
            length as usize,
            self.page_size - first_page_start as usize,
          );
        for index in first_page_start..begin_page_end {
          page[index] = 0;
        }
      }
    }

    // Delete intermediate pages
    if last_page_num > first_page_num + 1
      || (first_page_start == 0 && last_page_num == first_page_num + 1)
    {
      let first_page_to_drop = if first_page_start == 0 {
        first_page_num
      } else {
        first_page_num + 1
      };

      for index in first_page_to_drop..last_page_num {
        self.buffers.remove(index as u64);
      }
    }

    // Finally zero the last page
    if last_page_num > first_page_num && last_page_end > 0 {
      if let Some(page) = self.buffers.get_mut(last_page_num as u64) {
        // Need to zero part of the final page
        for index in 0..last_page_end {
          page[index] = 0;
        }
      }
    }
  }
}

#[async_trait::async_trait]
impl RandomAccess for RandomAccessMemory {
  async fn write(
    &mut self,
    offset: u64,
    data: &[u8],
  ) -> Result<(), RandomAccessError> {
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
      if self.buffers.get(page_num as u64).is_none() {
        let buf = vec![0; self.page_size as usize];
        self.buffers.insert(page_num as u64, buf);
      }

      // Copy data from the vec slice.
      // TODO: use a batch operation such as `.copy_from_slice()` so it can be
      // optimized.
      let buffer = &mut self.buffers.get_mut(page_num as u64).unwrap();
      for (index, buf_index) in range.enumerate() {
        buffer[buf_index as usize] = data[data_cursor + index];
      }

      page_num += 1;
      page_cursor = 0;
      data_cursor += range_len;
    }

    Ok(())
  }

  async fn sync_all(&mut self) -> Result<(), RandomAccessError> {
    Ok(())
  }

  async fn read(
    &mut self,
    offset: u64,
    length: u64,
  ) -> Result<Vec<u8>, RandomAccessError> {
    if (offset + length) > self.length {
      return Err(RandomAccessError::OutOfBounds {
        offset,
        end: Some(offset + length),
        length: self.length,
      });
    };

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
      match self.buffers.get(page_num as u64) {
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

  async fn del(
    &mut self,
    offset: u64,
    length: u64,
  ) -> Result<(), RandomAccessError> {
    if offset > self.length {
      return Err(RandomAccessError::OutOfBounds {
        offset,
        end: None,
        length: self.length,
      });
    };

    if length == 0 {
      // No-op
      return Ok(());
    }

    // Delete is truncate if up to the current length or more is deleted
    if offset + length >= self.length {
      return self.truncate(offset).await;
    }

    // Deleting means zeroing
    Ok(self.zero(offset, length))
  }

  async fn truncate(&mut self, length: u64) -> Result<(), RandomAccessError> {
    let (current_last_page_num, _) = self.page_num_and_index(self.length, true);

    if self.length < length {
      let truncate_page_num = (length / self.page_size as u64) as usize;
      // Remove all of the pages between the old length and this newer
      // length that might have been left behind.
      for index in current_last_page_num + 1..truncate_page_num + 1 {
        self.buffers.remove(index as u64);
      }
    } else if self.length > length {
      let delete_length =
        ((current_last_page_num + 1) * self.page_size) - length as usize;
      // Make sure to zero the remainder to not leave anything but
      // zeros lying around.
      self.zero(length, delete_length as u64);
    }

    // Set new length
    self.length = length;

    Ok(())
  }

  async fn len(&mut self) -> Result<u64, RandomAccessError> {
    Ok(self.length)
  }

  async fn is_empty(&mut self) -> Result<bool, RandomAccessError> {
    Ok(self.length == 0)
  }
}
