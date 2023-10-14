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
    pub fn new(message: impl ToString, kind: ErrorKind, span: Span, stage: Stage) -> Self {
        Self {
            message: message.to_string(),
            kind,
            span,
            stage,
            source: None,
        }
    }

    /// Add a source error to this code error.
    pub fn with_source(mut self, other: impl std::error::Error + 'static) -> Self {
        let existing = self.source.replace(Box::new(other));
        if let Some(err) = existing {
            log::warn!("original source error replaced: {err}");
        }
        self
    }

    pub fn format(&self, _f: &mut impl Write, _source_code: &str, _colors: bool) -> std::fmt::Result {
        todo!("format code error")
    }
}

impl std::fmt::Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            message, stage, source, ..
        } = self;

        write!(f, "{stage} error: {message}")?;

        if let Some(err) = source {
            write!(f, "; caused by: {err}")?;
        }

        Ok(())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_error_source() {
        let inner = CodeError::new("inner", ErrorKind::Unknown, Span::new(0, 1), Stage::Lexer);
        let err = CodeError::new("testcase", ErrorKind::Unknown, Span::new(0, 1), Stage::Parser).with_source(inner);

        assert_eq!(
            err.source.map(|e| format!("{e}")),
            Some("lexer error: inner".to_string())
        )
    }
}
