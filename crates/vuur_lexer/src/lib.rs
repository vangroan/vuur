//! Lexical analysis.
mod cursor;
mod token;

use cursor::{Cursor, EOF_CHAR};
pub use token::{Token, TokenKind};

pub struct Lexer<'a> {
    cursor: Cursor<'a>,

    /// Keep reference to the source so the parser can
    /// slice fragments from it.
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn from_str(source: &'a str) -> Self {
        Lexer {
            source,
            cursor: Cursor::from_str(source),
        }
    }

    pub fn next_token(&mut self) -> Token {
        // self.start_token();

        match self.cursor.peek() {
            EOF_CHAR => {
                // Source can contain an \0 character but not
                // actually be at the end of the stream.
                self.cursor.bump();
                self.make_token(TokenKind::EOF)
            }
            c if is_whitespace(c) => self.whitespace(),
            '\n' | '\r' => self.newline(),
            c if is_digit(c) => self.number_literal(),
            _ => {
                // println!("Unknown: {:?}", c);
                self.cursor.bump();
                self.make_token(TokenKind::Unknown)
            }
        }
    }

    pub fn is_eof(&self) -> bool {
        self.cursor.is_eof()
    }

    fn start_token(&mut self) {
        todo!()
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        Token { kind }
    }

    /// Consume whitespace.
    fn whitespace(&mut self) -> Token {
        while is_whitespace(self.cursor.peek()) {
            self.cursor.bump();
        }

        self.make_token(TokenKind::Whitespace)
    }

    fn newline(&mut self) -> Token {
        self.cursor.bump();
        // Windows carriage return
        if self.cursor.peek() == '\r' {
            self.cursor.bump();
        }
        self.make_token(TokenKind::Newline)
    }

    fn number_literal(&mut self) -> Token {
        while is_digit(self.cursor.peek()) {
            self.cursor.bump();
        }
        self.make_token(TokenKind::Number)
    }
}

fn is_whitespace(c: char) -> bool {
    // Doesn't include new line characters, because
    // they specify end-of-statement elsewhere.
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
        if self.lexer.is_eof() {
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
}
