use std::fmt::Write;

use crate::span::Span;

/// An error that occurred while processing source code.
#[derive(Debug)]
pub struct CodeError {
    pub message: String,
    pub kind: ErrorKind,
    pub span: Span,
    /// The stage of the compiler pipeline where the error happened.
    pub stage: Stage,
    source: Option<Box<dyn std::error::Error + 'static>>,
}

impl CodeError {
    pub fn format(&self, _f: &mut impl Write, _source_code: &str, _colors: bool) -> std::fmt::Result {
        todo!("format code error")
    }
}

impl std::fmt::Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}

impl std::error::Error for CodeError {
    /// The error that caused this error, if any.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|inner| &**inner)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Unknown,
}

/// The stage of the compiler pipeline where the error happened.
#[derive(Debug)]
pub enum Stage {
    Lexer,
    Parser,
    Compiler,
}

impl std::fmt::Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Lexer => "lexer",
            Self::Parser => "parser",
            Self::Compiler => "compiler",
        };

        std::fmt::Display::fmt(msg, f)
    }
}
