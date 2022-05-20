use std::str::Chars;

pub(crate) const EOF_CHAR: char = '\0';

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub(crate) fn from_str(source: &'a str) -> Self {
        Cursor { chars: source.chars() }
    }

    pub(crate) fn peek(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn peek2(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        let mut iter = self.chars.clone();
        iter.next().is_none()
    }

    /// Advances the cursor to the next character.
    ///
    /// Returns `None` if the cursor is end-of-file.
    pub(crate) fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_peek() {
        let mut cursor = Cursor::from_str("abcd");
        assert_eq!(cursor.peek(), 'a');
        assert_eq!(cursor.peek2(), 'b');

        assert_eq!(cursor.bump(), Some('a'));
        assert_eq!(cursor.bump(), Some('b'));

        assert_eq!(cursor.peek(), 'c');
        assert_eq!(cursor.peek2(), 'd');

        assert_eq!(cursor.bump(), Some('c'));

        assert_eq!(cursor.peek(), 'd');
        assert_eq!(cursor.peek2(), EOF_CHAR);

        assert_eq!(cursor.bump(), Some('d'));

        assert_eq!(cursor.peek(), EOF_CHAR);
        assert_eq!(cursor.peek2(), EOF_CHAR);
    }

    #[test]
    fn test_eof() {
        assert_eq!(Cursor::from_str("").is_eof(), true);
        assert_eq!(Cursor::from_str("abc").is_eof(), false);
    }
}
