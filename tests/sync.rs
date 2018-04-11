#[macro_use]
extern crate quickcheck;
extern crate random_access_memory as ram;

use quickcheck::{Arbitrary, Gen};

#[test]
fn can_call_new() {
  let _file = ram::Sync::default();
}

#[test]
fn can_open_buffer() {
  let mut file = ram::Sync::default();
  file.write(0, b"hello").unwrap();
  assert!(file.opened);
}

#[test]
fn can_write() {
  let mut file = ram::Sync::default();
  file.write(0, b"hello").unwrap();
  file.write(5, b" world").unwrap();
}

#[test]
fn can_read() {
  let mut file = ram::Sync::default();
  file.write(0, b"hello").unwrap();
  file.write(5, b" world").unwrap();
  let text = file.read(0, 11).unwrap();
  let text = String::from_utf8(text.to_vec()).unwrap();
  assert_eq!(text, "hello world");
}

#[derive(Clone, Debug)]
enum Op {
  Read { offset: usize, length: usize },
  Write { offset: usize, data: Vec<u8> },
}

#[test]
fn regress_1() {
  let mut file = ram::Sync::new(50);
  file.write(30, &[30]).unwrap();
  file.read(15, 15).unwrap();
  assert!(file.opened);
}

use self::Op::*;
const MAX_FILE_SIZE: usize = 5 * 10; // 5mb

impl Arbitrary for Op {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let offset: usize = g.gen_range(0, MAX_FILE_SIZE);
    let length: usize = g.gen_range(0, MAX_FILE_SIZE / 3);

    if g.gen::<bool>() {
      Op::Read { offset, length }
    } else {
      // TODO: randomize data even further. Unicode?
      let data = vec![offset as u8; length];
      Write { offset, data }
    }
  }
}

quickcheck! {
  fn implementation_matches_model(ops: Vec<Op>) -> bool {
    let mut implementation = ram::Sync::new(10);
    let mut model = vec![];

    for op in ops {
      match op {
        Read { offset, length } => {
          let end = offset + length;
          if model.len() >= end {
            assert_eq!(
              &*implementation.read(offset, length).unwrap(),
              &model[offset..end]
            );
          } else {
            assert!(implementation.read(offset, length).is_err());
          }
        },
        Write { offset, ref data } => {
          let end = offset + data.len();
          if model.len() < end {
            model.resize(end, 0);
          }
          implementation.write(offset, &*data).unwrap();
          model[offset..end].copy_from_slice(data);
        },
      }
    }
    true
  }
}
