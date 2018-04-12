#![deny(missing_docs)]
// #![cfg_attr(test, deny(warnings))]
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

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

/// Synchronous implementation.
mod sync;

pub use sync::*;
