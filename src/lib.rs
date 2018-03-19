#![deny(warnings, missing_docs)]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

//! Continuously read,write to memory using random offsets and lengths. Adapted
//! from
//! [mafintosh/random-access-memory](https://github.com/mafintosh/random-access-memory).

/// Synchronous implementation.
pub mod sync;
