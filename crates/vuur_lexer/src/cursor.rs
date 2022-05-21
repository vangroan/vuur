use std::str::CharIndices;

use crate::span::BytePos;

pub(crate) const EOF_CHAR: char = '\0';

pub(crate) struct Cursor<'a> {
    chars: CharIndices<'a>,
    /// Byte offset of the next character.
    ///
    /// Function for retrieving the current
    /// offset is nightly only, so we need
    /// our own book keeping.
    prev_offset: u32,

    /// The previous character, which can be used
    /// by the lexer to validate its invariants.
    #[cfg(debug)]
    prev_char: char,
}

impl<'a> Cursor<'a> {
    pub(crate) fn from_str(source: &'a str) -> Self {
        Cursor {
            chars: source.char_indices(),
            prev_offset: 0,
        }
    }

    /// Previous offset
    pub(crate) fn offset(&self) -> BytePos {
        BytePos(self.prev_offset)
    }

    /// Peek the next character without advancing the cursor.
    pub(crate) fn peek(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next().map(|(_, c)| c).unwrap_or(EOF_CHAR)
    }

    /// Peek two characters ahead without advancing the cursor.
    pub(crate) fn peek2(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().map(|(_, c)| c).unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        let mut iter = self.chars.clone();
        iter.next().is_none()
    }

    /// Advances the cursor to the next character.
    ///
    /// Returns `None` if the cursor is end-of-file.
    pub(crate) fn bump(&mut self) -> Option<(BytePos, char)> {
        match self.chars.next() {
            Some((i, c)) => {
                let i = i as u32;
                self.prev_offset = i;
                Some((BytePos(i), c))
            }
            None => None,
        }
    }

    #[cfg(debug)]
    pub(crate) fn prev_char(&self) -> char {
        self.prev_char
    }
}

pub(crate) struct LineRecorder {
    /// Positions in source code where each line starts.
    ///
    /// The position is the first character after the last
    /// newline token ended. Column number can be calculated
    /// given a character position.
    ///
    /// Implicitly the index of the element in the vector
    /// is the line number, zero-indexed.
    lines: Vec<BytePos>,
}

impl LineRecorder {
    pub(crate) fn new() -> Self {
        LineRecorder { lines: Vec::new() }
    }

    /// Advance the cursor and record encountered lines.
    pub(crate) fn bump(&mut self, cursor: &mut Cursor) -> Option<(BytePos, char)> {
        match cursor.bump() {
            Some((pos, c)) => {
                if c == '\n' {
                    self.lines.push(pos);
                }
                Some((pos, c))
            }
            None => None,
        }
    }

    /// Calculate the character's line and column given its
    /// byte position.
    pub(crate) fn pos(&self, byte_pos: BytePos, source: &str) -> Option<(u32, u32)> {
        todo!()
    }
}

impl Default for LineRecorder {
    fn default() -> Self {
        LineRecorder::new()
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

        assert_eq!(cursor.bump(), Some((BytePos(0), 'a')));
        assert_eq!(cursor.bump(), Some((BytePos(1), 'b')));

        assert_eq!(cursor.peek(), 'c');
        assert_eq!(cursor.peek2(), 'd');

        assert_eq!(cursor.bump(), Some((BytePos(2), 'c')));

        assert_eq!(cursor.peek(), 'd');
        assert_eq!(cursor.peek2(), EOF_CHAR);

        assert_eq!(cursor.bump(), Some((BytePos(3), 'd')));

        assert_eq!(cursor.peek(), EOF_CHAR);
        assert_eq!(cursor.peek2(), EOF_CHAR);
    }

    #[test]
    fn test_eof() {
        assert_eq!(Cursor::from_str("").is_eof(), true);
        assert_eq!(Cursor::from_str("abc").is_eof(), false);
    }
}
