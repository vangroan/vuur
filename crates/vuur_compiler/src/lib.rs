//! Vuur Compiler
#![deny(rust_2018_idioms)]

mod error;
mod span;

pub use self::{
    error::{CodeError, ErrorKind, Stage},
    span::Span,
};
