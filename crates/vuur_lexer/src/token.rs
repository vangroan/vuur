pub struct Token {
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
