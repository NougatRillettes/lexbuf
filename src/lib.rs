//! Hling buffers are buffers with a notion of *highlight*.
//!
//! #Description
//! This crates provides a single Trait: `HlBuf` and structs implementing it.
//!
//! This trait intends to ease the handwritting of hlers or parser by providing
//! buffers with a notion of "current highlight". It should help you write
//! anything that needs to look forward in a collection to make some decision.
//!
//! More details are given in the `HlBuf` trait documentation.
//!

mod iter_hlbuf;
mod read_hlbuf;
mod hlbuf;

pub use hlbuf::*;
