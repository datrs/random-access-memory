# random-access-memory
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Continuously read,write to memory using random offsets and lengths.

- [Documentation][8]
- [Crate][2]

## Usage
```rust
extern crate random_access_memory as ram;

let mut file = ram::Sync::new();
file.write(0, b"hello").unwrap();
file.write(5, b" world".unwrap();
let text = file.read(0, 11).unwrap();
assert_eq!(text, b"hello world");
```

## Installation
```sh
$ cargo add random-access-memory
```

## Tasks
- [x] Sync implementation.
- [ ] Async implementation (wait for futures 1.0.0).

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/random-access-memory.svg?style=flat-square
[2]: https://crates.io/crate/random-access-memory
[3]: https://img.shields.io/travis/datrs/random-access-memory.svg?style=flat-square
[4]: https://travis-ci.org/datrs/random-access-memory
[5]: https://img.shields.io/crates/d/random-access-memory.svg?style=flat-square
[6]: https://crates.io/crates/random-access-memory
[7]: https://docs.rs/random-access-memory/badge.svg
[8]: https://docs.rs/random-access-memory
