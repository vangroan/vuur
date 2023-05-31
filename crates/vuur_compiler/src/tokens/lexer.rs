//! Lexical analysis.

use std::ops::Range;

use super::cursor::{Cursor, EOF_CHAR};
use super::tokens::{Keyword, Token, TokenKind};
use crate::span::Span;

/// Lexical analyser (tokeniser) for the Vuur language.
pub struct Lexer<'a> {
    /// Keep an index into the source, which is advanced
    /// as characters are consumed.
    cursor: Cursor<'a>,
    /// Keep reference to the source so the parser can
    /// slice fragments from it.
    source_code: &'a str,
    /// Absolute starting byte position of the current token
    /// in the source.
    start_pos: u32,
}

impl<'a> Lexer<'a> {
    /// Create the lexer from the given source code.
    pub fn from_source(source_code: &'a str) -> Self {
        let mut cursor = Cursor::from_str(source_code);

        // Initial state of the cursor is a non-existant EOF char,
        // but the initial state of the lexer should be a valid
        // token starting character.
        //
        // Prime the cursor for the first iteration.
        cursor.bump();

        // For what it's worth, the cursor gets to decide what the
        // initial byte position is.
        let start_pos = cursor.offset();

        Lexer {
            source_code,
            cursor,
            start_pos,
        }
    }

    /// Retrieve the original source code that was
    /// passed into the lexer.
    pub fn source_code(&self) -> &'a str {
        self.source_code
    }

    /// Remainder of source to be consumed.
    pub fn rest(&self) -> &'a str {
        let start = self.cursor.offset() as usize;
        let end = self.cursor.original_length() as usize;

        &self.source_code[start..end]
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
    pub fn next_token(&mut self) -> Option<Token> {
        // Invariant of the lexer is that the cursor must
        // be pointing to the start of the remainder of the
        // source to be consumed.
        self.ignore_whitespace(); // <-- start_token

        if !self.cursor.at_end() {
            let token = match self.cursor.current() {
                '(' => self.make_token(TokenKind::LeftParen),
                ')' => self.make_token(TokenKind::RightParen),
                '[' => self.make_token(TokenKind::LeftBracket),
                ']' => self.make_token(TokenKind::RightBracket),
                '{' => self.make_token(TokenKind::LeftBrace),
                '}' => self.make_token(TokenKind::RightBrace),
                '.' => self.make_token(TokenKind::Dot),
                // '"' => self.consume_string(TokenKind::String),
                '+' => self.make_token(TokenKind::Add),
                '-' => {
                    if self.cursor.peek() == '>' {
                        self.cursor.bump();
                        self.make_token(TokenKind::ThinArrow)
                    } else {
                        self.make_token(TokenKind::Sub)
                    }
                }
                '=' => {
                    if self.cursor.peek() == '=' {
                        self.cursor.bump();
                        self.make_token(TokenKind::EqEq)
                    } else {
                        self.make_token(TokenKind::Eq)
                    }
                }
                '*' => self.make_token(TokenKind::Mul),
                '/' => self.make_token(TokenKind::Div),
                '&' => self.make_token(TokenKind::Ampersand),
                ',' => self.make_token(TokenKind::Comma),
                ':' => self.make_token(TokenKind::Colon),
                ';' => self.make_token(TokenKind::Semicolon),

                EOF_CHAR => {
                    // Source can contain an \0 character but not
                    // actually be at the end of the stream.
                    self.make_token(TokenKind::EOF)
                }
                '\n' | '\r' => self.consume_newline(),

                c if Self::is_digit(c) => self.consume_number(),
                c if Self::is_letter(c) => self.consume_ident(),
                _ => self.make_token(TokenKind::Unknown),
            };

            Some(token)
        } else {
            // The source stream has run out, so we signal
            // the caller by emitting an end-of-file token that
            // doesn't exist in the text.
            //
            // The token's span thus points to the element
            // beyond the end of the collection, and has 0 length.
            // self.start_pos = self.cursor.peek_offset(); // TODO: Explicit string size
            // self.make_token(TokenKind::EOF)

            None
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

    /// Returns a [`Span`](struct.Span.html) for the current token.
    #[inline]
    fn token_span(&self) -> Span {
        let start = self.start_pos;
        let end = self.cursor.peek_offset();

        // start and end can be equal, and a token can have 0 size.
        debug_assert!(end >= start, "invariant: token end must be past token start");
        let size = end - start;

        Span::new(start, size)
    }

    /// Range from the start of the current token (inclusive) to
    /// the position one past the current (exclusive).
    #[inline(always)]
    fn token_range(&self) -> Range<usize> {
        let start = self.start_pos as usize;
        let end = self.cursor.peek_offset() as usize;
        start..end
    }

    /// Build a token, using the source text from the position
    /// stored by [`start_token`](struct.Lexer.html#fn-start_token) to the
    /// current cursor position.
    ///
    /// Also prepare the cursor for the next iteration.
    fn make_token(&mut self, kind: TokenKind) -> Token {
        let span = self.token_span();

        // After this token is built, the lexer's internal state
        // is no longer dedicated to this iteration, but to preparing
        // for the next iteration.
        let token = Token { span, kind };

        // Position the cursor to the starting character for the
        // next token, so the lexer's internal state is primed
        // for the next iteration.
        self.cursor.bump();

        token
    }
}

/// Methods for consuming specific tokens.
impl<'a> Lexer<'a> {
    /// Skips over UTF-8 whitespace characters, and resets the token start.
    fn ignore_whitespace(&mut self) {
        while Self::is_whitespace(self.cursor.current()) {
            self.cursor.bump();
        }

        self.start_token();
    }

    /// Consume whitespace.
    #[deprecated]
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
        let start = self.start_pos as usize;
        let end = self.cursor.peek_offset() as usize;
        let fragment = &self.source_code[start..end];
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
    #[inline]
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
                self.lexer.next_token()
            }
        } else {
            self.lexer.next_token()
        }
    }
}
