#![no_main]

use libfuzzer_sys::fuzz_target;
use random_access_memory as ram;

fuzz_target!(|data: &[u8]| {
  let mut file = ram::Sync::default();
  file.write(0, data).unwrap();
});
