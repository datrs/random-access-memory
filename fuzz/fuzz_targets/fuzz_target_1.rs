#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate random_access_memory as ram;

fuzz_target!(|data: &[u8]| {
  let mut file = ram::Sync::default();
  file.write(0, data).unwrap();
});
