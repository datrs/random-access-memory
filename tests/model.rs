use self::Op::*;
use quickcheck::{quickcheck, Arbitrary, Gen};
use rand::Rng;
use random_access_memory as ram;
use random_access_storage::RandomAccess;
use std::u8;

const MAX_FILE_SIZE: u64 = 5 * 10; // 5mb

#[derive(Clone, Debug)]
enum Op {
  Read { offset: u64, length: u64 },
  Write { offset: u64, data: Vec<u8> },
  Delete { offset: u64, length: u64 },
}

impl Arbitrary for Op {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let offset: u64 = g.gen_range(0, MAX_FILE_SIZE);
    let length: u64 = g.gen_range(0, MAX_FILE_SIZE / 3);

    let op = g.gen_range(0_u8, 3_u8);

    if op == 0 {
      Read { offset, length }
    } else if op == 1 {
      let mut data = Vec::with_capacity(length as usize);
      for _ in 0..length {
        data.push(u8::arbitrary(g));
      }
      Write { offset, data }
    } else {
      Delete { offset, length }
    }
  }
}

quickcheck! {
  fn implementation_matches_model(ops: Vec<Op>) -> bool {
    async_std::task::block_on(async {
      let mut implementation = ram::RandomAccessMemory::new(10);
      let mut model = vec![];

      for op in ops {
        match op {
          Read { offset, length } => {
            let end = offset + length;
            if model.len() >= end as usize {
              assert_eq!(
                &*implementation.read(offset, length).await.expect("Reads should be successful."),
                &model[offset as usize..end as usize]
              );
            } else {
              assert!(implementation.read(offset, length).await.is_err());
            }
          },
          Write { offset, ref data } => {
            let end = offset + data.len() as u64;
            if model.len() < end as usize {
              model.resize(end as usize, 0);
            }
            implementation.write(offset, data).await.expect("Writes should be successful.");
            model[offset as usize..end as usize].copy_from_slice(data);
          },
          Delete { offset, length } => {
            if model.len() >= offset as usize {
              implementation.del(offset, length).await.expect("Deletes should be successful.");
              if offset + length < model.len() as u64 {
                model[offset as usize..(offset + length) as usize].fill(0);
              } else {
                model.resize(offset as usize, 0);
              };
            } else {
              assert!(implementation.del(offset, length).await.is_err());
            }
          }
        }
      }
      true
    })
  }
}
