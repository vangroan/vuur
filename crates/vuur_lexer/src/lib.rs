//! Lexical analysis.
mod cursor;
mod span;
mod token;
mod unescape;

use cursor::{Cursor, LineRecorder, EOF_CHAR};
use span::BytePos;
pub use token::{Token, TokenKind};

/// Lexical analyser (tokeniser) for Vuur language.
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    lines: LineRecorder,
    /// Keep reference to the source so the parser can
    /// slice fragments from it.
    source: &'a str,
    /// Start absolute byte position of the current token
    /// in the source.
    start_pos: BytePos,
}

impl<'a> Lexer<'a> {
    pub fn from_str(source: &'a str) -> Self {
        Lexer {
            source,
            lines: LineRecorder::default(),
            cursor: Cursor::from_str(source),
            start_pos: BytePos(0),
        }
    }

    /// Retrieve the original source code that was
    /// passed into the lexer.
    pub fn source(&self) -> &'a str {
        self.source
    }

    /// Scan the source characters and construct the next token.
    ///
    /// ## Implementation
    ///
    /// The internal iteration of the lexer follows this convention:
    ///
    /// Each iteration (`next_token` call) is responsible for setting up
    /// the internal cursor to consume its own token.
    ///
    /// The previous iteration may leave the cursor at the last character
    /// included in its built token.
    pub fn next_token(&mut self) -> Token {
        // Cursor was left at the last character of the previous iteration's token.
        //
        // This iteration is responsible for setting up the cursor for
        // its own token.
        if let Some((_, next_char)) = self.next_char() {
            // Once the cursor is primed, we can start recording the current token.
            self.start_token();

            match next_char {
                EOF_CHAR => {
                    // Source can contain an \0 character but not
                    // actually be at the end of the stream.
                    self.make_token(TokenKind::EOF)
                }
                c if Self::is_whitespace(c) => self.consume_whitespace(),
                '\n' | '\r' => self.consume_newline(),
                c if Self::is_digit(c) => self.consume_number(),
                _ => self.make_token(TokenKind::Unknown),
            }
        } else {
            // The source stream has run out, so we signal
            // the caller by emitting an end-of-file token that
            // doesn't exist in the text.
            //
            // The token's span thus points to the element
            // beyond the end of the collection, and has 0 length.
            self.start_pos = self.cursor.peek_offset();
            self.make_token(TokenKind::EOF)
        }
    }

    /// Indicates whether the lexer is at the end of the source.
    ///
    /// Note that source can contain '\0' (end-of-file) characters,
    /// but not be at the actual end. It's thus important to verify
    /// with this function whenever a [`TokenKind::EOF`] is encountered.
    pub fn at_end(&self) -> bool {
        self.cursor.at_end()
    }

    /// Primes the lexer to consume the next token.
    fn start_token(&mut self) {
        self.start_pos = self.cursor.offset();
    }

    /// Advance the character cursor, and inspect the encountered
    /// characters. When newlines are encountered, a line is recorded.
    fn next_char(&mut self) -> Option<(BytePos, char)> {
        self.lines.bump(&mut self.cursor)
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        let start = self.start_pos.0 as u32;
        let end = self.cursor.peek_offset().0;

        // start and end can be equal, and a token can have 0 size.
        debug_assert!(end >= start);
        let size = end - start;

        Token {
            offset: self.start_pos,
            size,
            kind,
        }
    }
}

/// Methods for consuming specific tokens.
impl<'a> Lexer<'a> {
    /// Consume whitespace.
    fn consume_whitespace(&mut self) -> Token {
        while Self::is_whitespace(self.cursor.peek()) {
            self.next_char();
        }

        self.make_token(TokenKind::Whitespace)
    }

    /// Consumes a single newline token.
    fn consume_newline(&mut self) -> Token {
        // TODO: assert current == \n
        // Windows carriage return
        if self.cursor.peek() == '\r' {
            self.next_char();
        }
        self.make_token(TokenKind::Newline)
    }

    /// Consumes a number literal.
    fn consume_number(&mut self) -> Token {
        // debug_assert!(Self::is_digit(self.cursor.prev_char()));

        while Self::is_digit(self.cursor.peek()) {
            self.next_char();
        }
        self.make_token(TokenKind::Number)
    }
}

/// Methods for testing characters.
impl<'a> Lexer<'a> {
    /// Test whether the character is considered whitespace
    /// that should be ignored by the parser later.
    ///
    /// Doesn't include newline characters, because in Vuur
    /// newline are significant, specifying end-of-statement.
    fn is_whitespace(c: char) -> bool {
        matches!(
            c,
            '\u{0020}' // space
            | '\u{0009}' // tab
            | '\u{00A0}' // no-break space
            | '\u{FEFF}' // zero width no-break space
        )
    }

    fn is_digit(c: char) -> bool {
        matches!(c, '0'..='9')
    }
}

impl<'a> IntoIterator for Lexer<'a> {
    type Item = Token;
    type IntoIter = LexerIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LexerIter {
            lexer: self,
            done: false,
        }
    }
}

/// Convenience iterator that wraps the lexer.
pub struct LexerIter<'a> {
    // Track end so an EOF token is emitted once.
    done: bool,
    lexer: Lexer<'a>,
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lexer.at_end() {
            if self.done {
                None
            } else {
                self.done = true;
                Some(self.lexer.next_token())
            }
        } else {
            Some(self.lexer.next_token())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer_whitespace() {
        let mut lexer = Lexer::from_str(
            r"
            ",
        );

        assert_eq!(lexer.next_token().kind, TokenKind::Newline);
        assert_eq!(lexer.next_token().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().kind, TokenKind::EOF);
    }

    #[test]
    fn test_lines() {
        // NOTE: Tests assume this rust file uses \n and not \n\r
        let mut lexer = Lexer::from_str(
            r"0
        1
        23",
        );

        let source = lexer.source.to_owned();

        // for token in lexer.into_iter() {
        //     println!("'{}' - {:?}", unescape::unescape_str(token.fragment(&source)), token);
        // }

        // (kind, offset, size)
        #[rustfmt::skip]
        let expected: Vec<(TokenKind, u32, u32)> = vec![
            (TokenKind::Number,      0, 1), // 0
            (TokenKind::Newline,     1, 1),
            (TokenKind::Whitespace,  2, 8),
            (TokenKind::Number,     10, 1), // 1
            (TokenKind::Newline,    11, 1),
            (TokenKind::Whitespace, 12, 8),
            (TokenKind::Number,     20, 2), // 23
            (TokenKind::EOF,        22, 0),
        ];

        for (token, exp) in lexer.into_iter().zip(expected.into_iter()) {
            println!("{:?} == {:?}", token, exp);
            assert_eq!(token.kind, exp.0);
            assert_eq!(token.offset.to_u32(), exp.1);
            assert_eq!(token.size, exp.2);
        }

        // let token_0 = lexer.next_token();
        // assert_eq!(token_0.offset, 0);
        // assert_eq!(token_0.size, 1);
        // assert_eq!(token_0.kind, TokenKind::Number);
    }
}
