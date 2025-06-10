//! Vuur Compiler

mod error;
mod cursor;
mod lexer;
mod limits;
mod span;
mod stack;
mod string;
mod tokens;

pub use self::{
    error::{CodeError, ErrorKind, Stage},
    lexer::Lexer,
    span::Span,
    tokens::{Token, TokenKind},
};
