//! Tokens.

use crate::span::BytePos;

#[derive(Debug)]
pub struct Token {
    pub offset: BytePos,
    pub size: u32, // in bytes
    pub kind: TokenKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// End-of-file
    EOF,
    /// Spaces and tabs.
    Whitespace,
    /// Line-feed and optionally a carriage return
    Newline,
    /// Unknown character was encoutered in the source.
    Unknown,

    // Number Literal
    Number,
}

impl Token {
    /// Slice a text fragment from the given source code.
    pub fn fragment<'a>(&self, source: &'a str) -> &'a str {
        let start = self.offset.0 as usize;
        let end = start + self.size as usize;
        &source[start..end]
    }
}
