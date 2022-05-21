use std::str::CharIndices;

use crate::span::BytePos;

pub(crate) const EOF_CHAR: char = '\0';

pub(crate) struct Cursor<'a> {
    chars: CharIndices<'a>,
    /// Previous character returned by the internal iterator.
    ///
    /// Store the result of the previous iteration so it's
    /// available on demand as the "current" state of the cursor.
    prev: (u32, char),
    /// Original size of source passed in.
    orig_size: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn from_str(source: &'a str) -> Self {
        Cursor {
            chars: source.char_indices(),
            prev: (0, EOF_CHAR),
            orig_size: source.len(),
        }
    }

    /// Byte offset of the current character.
    pub(crate) fn offset(&self) -> BytePos {
        BytePos(self.prev.0)
    }

    /// Current character in the iteration.
    ///
    /// If iteration has not started, will return end-of-file character.
    pub(crate) fn current(&self) -> char {
        self.prev.1
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

    /// Peek the byte position of the next character.
    pub(crate) fn peek_offset(&self) -> BytePos {
        // Byte position of next character is determined by number
        // of byts taken up by the current character.
        //
        // Because of UTF-8 encoding, there is no easy way
        // to know the size of the current character except
        // advancing the iterator.
        let mut iter = self.chars.clone();
        iter.next()
            .map(|(index, _)| BytePos(index as u32))
            .unwrap_or_else(|| BytePos(self.orig_size as u32))
    }

    // Indicates whether the cursor is at the end of the source.
    pub(crate) fn at_end(&self) -> bool {
        let mut iter = self.chars.clone();
        iter.next().is_none()
    }

    /// Advances the cursor to the next character.
    ///
    /// Returns `None` if the cursor is end-of-file.
    pub(crate) fn bump(&mut self) -> Option<(BytePos, char)> {
        match self.chars.next() {
            Some((i, c)) => {
                // Convert index to smaller integer so
                // tuple fits into 64-bits.
                let i = i as u32;
                self.prev = (i, c);
                Some((BytePos(i), c))
            }
            None => None,
        }
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
        assert_eq!(Cursor::from_str("").at_end(), true);
        assert_eq!(Cursor::from_str("abc").at_end(), false);
    }
}
