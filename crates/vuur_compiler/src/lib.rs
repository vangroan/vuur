//! Vuur Compiler
#![deny(rust_2018_idioms)]

mod ast;
mod bytecode;
mod error;
mod limits;
mod span;
mod stack;
mod string;
mod tokens;

pub use self::{
    error::{CodeError, ErrorKind, Stage},
    span::Span,
    tokens::lexer::Lexer,
    tokens::tokens::{Token, TokenKind},
};
