/*
https://github.com/datrs/random-access-storage/blob/master/src/lib.rs
http://siciarz.net/24-days-of-rust-calling-rust-from-other-languages/
*/

extern crate failure;

use failure::Error;

pub trait RandomAccessMethods {
  /// Open the backend.
  fn open(&mut self) -> Result<(), Error>;

  /// Write bytes at an offset to the backend.
  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error>;

  /// Read a sequence of bytes at an offset from the backend.
  fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, Error>;

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&mut self, offset: usize, length: usize) -> Result<(), Error>;
}

struct Bar {
  n: usize,
}

impl Bar {
  fn new() -> Bar {
    Bar { n: 0 }
  }
}

impl RandomAccessMethods for Bar {
  fn open(&mut self) -> Result<(), Error> {
    Ok(())
  }

  /// Write bytes at an offset to the backend.
  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error> {
    println!("write(offset: {}, data: {:?})", offset, data);
    Ok(())
  }

  /// Read a sequence of bytes at an offset from the backend.
  fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, Error> {
    println!("read(offset: {}, length: {})", offset, length);
    Ok(vec![])
  }

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&mut self, offset: usize, length: usize) -> Result<(), Error> {
    println!("del(offset: {}, length: {})", offset, length);
    Ok(())
  }
}

#[no_mangle]
pub extern "C" fn dat_write(
  _ctx: *const u8,
  offset: usize,
  length: usize,
  array: *const u8,
) -> i32 {
  println!("dat_write()");
  let slice = unsafe { ::std::slice::from_raw_parts(array, length) };
  let mut bar = Bar::new();
  match bar.write(offset, slice) {
    Ok(_) => 0,
    Err(_) => 1,
  }
}

#[no_mangle]
pub extern "C" fn dat_read(
  _ctx: *const u8,
  offset: usize,
  length: usize,
  array: *mut u8,
) -> i32 {
  println!("dat_read()");
  let slice = unsafe { ::std::slice::from_raw_parts_mut(array, length) };
  let mut bar = Bar::new();
  if let Ok(v) = bar.read(offset, length) {
    //got Result<Vec<u8>, Error>
    slice.copy_from_slice(&v[..]);
    return 0;
  }
  return 1;
}
