use self::Op::*;
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;
use proptest_derive::Arbitrary;
use random_access_memory as ram;
use random_access_storage::RandomAccess;
use std::u8;

const MAX_FILE_SIZE: u64 = 50000;

#[derive(Clone, Debug, Arbitrary)]
enum Op {
  Read {
    #[proptest(strategy(offset_length_strategy))]
    offset: u64,
    #[proptest(strategy(offset_length_strategy))]
    length: u64,
  },
  Write {
    #[proptest(strategy(offset_length_strategy))]
    offset: u64,
    #[proptest(regex(data_regex))]
    data: Vec<u8>,
  },
  Delete {
    #[proptest(strategy(offset_length_strategy))]
    offset: u64,
    #[proptest(strategy(offset_length_strategy))]
    length: u64,
  },
}

fn offset_length_strategy() -> impl Strategy<Value = u64> {
  0..MAX_FILE_SIZE
}

fn data_regex() -> &'static str {
  // Write 0..5000 byte chunks of ASCII characters as dummy data
  "([ -~]{1,1}\n){0,5000}"
}

proptest! {
  #![proptest_config(ProptestConfig {
    failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
    ..Default::default()
  })]

  #[test]
  fn implementation_matches_model(ops: Vec<Op>) {
    assert!(async_std::task::block_on(async {
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
    }));
  }
}
