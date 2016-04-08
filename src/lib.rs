//! Lexing buffers are buffers with a notion of *highlight*.
//!
//! #Description
//! This crates provides a single Trait: `LexBuf` and structs implementing it.
//!
//! This trait intends to ease the handwritting of lexers or parser as it carries a notion
//! of "current highlight".
//!
//! #Caveat
//!
//! The "current highlight" may not be larger than 4096 item. If it is, internal functions may panic.
//!
//!

mod iter_lexbuf;
mod read_lexbuf;
mod lexbuf;

pub use lexbuf::*;
