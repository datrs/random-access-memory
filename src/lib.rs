#![deny(warnings, missing_docs)]
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
//! let file = ram::sync::new();
//! file.write(0, b"hello")?;
//! file.write(0, b" world")?;
//! let text = file.read(0, 11,)?;
//! assert!(text, b"hello world");
//! ```

/// Synchronous implementation.
pub mod sync_impl;

pub use sync_impl::RandomAccessMemory as sync;
