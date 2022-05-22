//! Lexical analysis.
mod cursor;
pub mod span;
mod token;
mod unescape;

use cursor::{Cursor, EOF_CHAR};
use span::BytePos;
pub use token::{Keyword, Token, TokenKind};

/// Lexical analyser (tokeniser) for Vuur language.
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    /// Keep reference to the source so the parser can
    /// slice fragments from it.
    source: &'a str,
    /// Start absolute byte position of the current token
    /// in the source.
    start_pos: BytePos,
}

impl<'a> Lexer<'a> {
    pub fn from_str(source: &'a str) -> Self {
        let mut cursor = Cursor::from_str(source);

        // Initial state of the cursor is an non-existant EOF char,
        // but the initial state of the lexer should be a valid
        // token starting character.
        //
        // Prime the cursor for the first iteration.
        cursor.bump();

        // For what it's worth, the cursor gets to decide what the
        // initial byte position is.
        let start_pos = cursor.offset();

        Lexer {
            source,
            cursor,
            start_pos,
        }
    }

    /// Retrieve the original source code that was
    /// passed into the lexer.
    pub fn source(&self) -> &'a str {
        self.source
    }

    /// Remainder of source to be consumed.
    pub fn rest(&self) -> &'a str {
        let start = self.cursor.offset().0 as usize;
        let end = self.cursor.original_length() as usize;
        // println!("start={} end={}", start, end);
        &self.source[start..end]
    }

    /// Scan the source characters and construct the next token.
    ///
    /// ## Implementation
    ///
    /// The internal iteration of the lexer follows this convention:
    ///
    /// Each iteration (`next_token` call) starts with the assumption that
    /// the internal cursor is pointing to the start of the remaining source
    /// to be consumed.
    ///
    /// Initially, the lexer must be constructed with a cursor pointing to
    /// the start of the source.
    ///
    /// When an iteration is done building a token, it must leave the cursor
    /// at the start of the next token's text. It may not finish leaving the
    /// cursor pointing into its own token.
    pub fn next_token(&mut self) -> Token {
        // Invariant of the lexer is that the cursor must
        // be pointing to the start of the remainder of the
        // source to be consumed.
        self.start_token();

        if !self.cursor.at_end() {
            match self.cursor.current() {
                '(' => self.make_token(TokenKind::LeftParen),
                ')' => self.make_token(TokenKind::RightParen),
                '[' => self.make_token(TokenKind::LeftBracket),
                ']' => self.make_token(TokenKind::RightBracket),
                '{' => self.make_token(TokenKind::LeftBrace),
                '}' => self.make_token(TokenKind::RightBrace),
                '.' => self.make_token(TokenKind::Dot),
                // '"' => self.consume_string(TokenKind::String),
                '+' => self.make_token(TokenKind::Add),
                '-' => self.make_token(TokenKind::Sub),
                '*' => self.make_token(TokenKind::Mul),
                '&' => self.make_token(TokenKind::Ampersand),
                ',' => self.make_token(TokenKind::Comma),
                ':' => self.make_token(TokenKind::Colon),
                ';' => self.make_token(TokenKind::Semicolon),

                EOF_CHAR => {
                    // Source can contain an \0 character but not
                    // actually be at the end of the stream.
                    self.make_token(TokenKind::EOF)
                }
                c if Self::is_whitespace(c) => self.consume_whitespace(),
                '\n' | '\r' => self.consume_newline(),

                c if Self::is_digit(c) => self.consume_number(),
                c if Self::is_letter(c) => self.consume_ident(),
                _ => self.make_token(TokenKind::Unknown),
            }
        } else {
            // The source stream has run out, so we signal
            // the caller by emitting an end-of-file token that
            // doesn't exist in the text.
            //
            // The token's span thus points to the element
            // beyond the end of the collection, and has 0 length.
            self.start_pos = self.cursor.peek_offset(); // TODO: Explicit string size
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

    /// Build a token, using the source text from the position
    /// stored by [`start_token`](struct.Lexer.html#fn-start_token) to the
    /// current cursor position.
    ///
    /// Also prepare the cursor for the next iteration.
    fn make_token(&mut self, kind: TokenKind) -> Token {
        let start = self.start_pos.0 as u32;
        let end = self.cursor.peek_offset().0;

        // start and end can be equal, and a token can have 0 size.
        debug_assert!(end >= start);
        let size = end - start;

        // After this token is built, the lexer's internal state
        // is no longer dedicated to this iteration, but to preparing
        // for the next iteration.
        let token = Token {
            offset: self.start_pos,
            size,
            kind,
        };

        // Position the cursor to the starting character for the
        // next token, so the lexer's internal state is primed
        // for the next iteration.
        self.cursor.bump();

        token
    }
}

/// Methods for consuming specific tokens.
impl<'a> Lexer<'a> {
    /// Consume whitespace.
    fn consume_whitespace(&mut self) -> Token {
        debug_assert!(Self::is_whitespace(self.cursor.current()));

        while Self::is_whitespace(self.cursor.peek()) {
            self.cursor.bump();
        }

        self.make_token(TokenKind::Whitespace)
    }

    /// Consumes a single newline token.
    fn consume_newline(&mut self) -> Token {
        debug_assert!(matches!(self.cursor.current(), '\n' | '\r'));

        // TODO: assert current == \n
        // Windows carriage return
        if self.cursor.peek() == '\r' {
            self.cursor.bump();
        }
        self.make_token(TokenKind::Newline)
    }

    /// Consumes a number literal.
    fn consume_number(&mut self) -> Token {
        debug_assert!(Self::is_digit(self.cursor.current()));

        while Self::is_digit(self.cursor.peek()) {
            self.cursor.bump();
        }
        self.make_token(TokenKind::Number)
    }

    fn consume_ident(&mut self) -> Token {
        debug_assert!(Self::is_letter(self.cursor.current()));

        while Self::is_letter_or_digit(self.cursor.peek()) {
            self.cursor.bump();
        }

        // Attempt to convert identifier to keyword.
        let start = self.start_pos.0 as usize;
        let end = self.cursor.peek_offset().0 as usize;
        let fragment = &self.source[start..end];
        // println!("fragment: '{}'", unescape::unescape_str(fragment));

        let token_kind = match Keyword::try_from(fragment) {
            Ok(keyword) => TokenKind::Keyword(keyword),
            Err(_) => TokenKind::Ident,
        };

        self.make_token(token_kind)
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

    fn is_letter(c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn is_letter_or_digit(c: char) -> bool {
        Self::is_letter(c) || Self::is_digit(c)
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
    fn test_remainder() {
        let mut lexer = Lexer::from_str(r"a b c d e f");
        assert_eq!(lexer.next_token().kind, TokenKind::Ident); // a
        assert_eq!(lexer.next_token().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().kind, TokenKind::Ident); // b
        assert_eq!(lexer.next_token().kind, TokenKind::Whitespace);
        assert_eq!(lexer.rest(), "c d e f");

        // Case where source is fully consumed
        assert_eq!(lexer.next_token().kind, TokenKind::Ident); // c
        assert_eq!(lexer.next_token().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().kind, TokenKind::Ident); // d
        assert_eq!(lexer.next_token().kind, TokenKind::Whitespace);
        assert_eq!(lexer.next_token().kind, TokenKind::Ident); // e
        assert_eq!(lexer.rest(), " f");
        assert_eq!(lexer.next_token().kind, TokenKind::Whitespace);
        assert_eq!(lexer.rest(), "f");
        assert_eq!(lexer.next_token().kind, TokenKind::Ident); // f
        assert_eq!(lexer.rest(), "");
        assert_eq!(lexer.next_token().kind, TokenKind::EOF);
    }

    #[test]
    fn test_lines() {
        // NOTE: Tests assume this rust file uses \n and not \n\r
        let lexer = Lexer::from_str(
            r"0
        1
        23",
        );

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
    }
}
