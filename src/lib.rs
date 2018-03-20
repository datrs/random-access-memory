// #![deny(warnings, missing_docs)]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

//! Continuously read,write to memory using random offsets and lengths. Adapted
//! from
//! [mafintosh/random-access-memory](https://github.com/mafintosh/random-access-memory).
//!
//! ## Usage
//! ```rust,ignore
//! extern crate random_access_memory as ram;
//!
//! let mut file = ram::Sync::new();
//! file.write(0, b"hello")?;
//! file.write(5, b" world")?;
//! let text = file.read(0, 11)?;
//! assert_eq!(text, b"hello world");
//! ```

#[macro_use]
extern crate failure;

#[macro_use]
extern crate failure_derive;

/// Synchronous implementation.
mod sync;

pub use sync::*;
