//! Lexical analysis.

/// Lexical analyser (tokeniser) for the Vuur language.
pub struct Lexer<'a> {
    /// Keep reference to the source so the parser can
    /// slice fragments from it.
    source_code: &'a str,
    /// Start absolute byte position of the current token
    /// in the source.
    start_pos: usize,
}

impl<'a> Lexer<'a> {
    /// Create the lexer from the given source code.
    pub fn from_source(source_code: &'a str) -> Self {
        todo!()
    }

    /// Retrieve the original source code that was
    /// passed into the lexer.
    pub fn source_code(&self) -> &'a str {
        self.source_code
    }
}
