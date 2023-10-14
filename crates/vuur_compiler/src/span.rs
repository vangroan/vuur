use std::ops::Range;

use unicode_width::UnicodeWidthChar;

/// A region of source code, stored as a position and size
/// of the bytes insize a UTF-8 string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub index: u32,
    pub size: u32,
}

// Span is size optimised for 64-bit machines.
#[cfg(target_pointer_width = "64")]
static_assertions::assert_eq_size!(Span, u64);

impl Span {
    /// Returns the exclusive range for the span, which
    /// can be used to slice a string.
    pub fn to_range(&self) -> Range<usize> {
        let start = self.index as usize;
        let end = start + self.size as usize;
        start..end
    }

    /// Scan the given text to determine the lines and columns
    /// of this span.
    ///
    /// This assumes the span belongs to the text. If unrelated
    /// text is passed in then the wrong stuff will be returned.
    ///
    /// # Panic
    ///
    /// Panics if the span does not fit inside the given text.
    pub fn line_column(&self, text: &str) -> CodeSpan {
        let mut line_span: (u32, u32) = (0, 0);
        let mut column_span: (u32, u32) = (0, 0);

        let Self { index, size } = self.clone();
        let start = index as usize;
        let end = start + size as usize;
        assert!(text.len() >= end, "span end is outside of text");

        let mut line = 1;
        let mut column = 1;

        for (index, chr) in text.char_indices().take(end) {
            if index == start {
                // Begin code span
                line_span.0 = line;
                column_span.0 = column;
            } else if index > start {
                line_span.1 = line.max(line_span.1);

                // Move column left in case span crosses line
                column_span.0 = column.min(column_span.0);
                column_span.1 = column.max(column_span.1); // end column, converted to size later
            }

            if matches!(chr, '\n') {
                line += 1;
                column = 1;
            } else {
                let width = UnicodeWidthChar::width(chr).unwrap_or(0) as u32;
                column += width;
            }
        }

        // Convert end line position to y-size.
        line_span.1 = line_span.1 + 1 - line_span.0;

        // Convert end column position to x-size.
        column_span.1 = column_span.1 + 1 - column_span.0;

        CodeSpan {
            line: line_span,
            column: column_span,
        }
    }
}

impl Span {
    pub const fn new(index: u32, size: u32) -> Self {
        Self { index, size }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { index, size } = self.clone();
        write!(f, "({index}, {size})")
    }
}

/// A region of code describing the start and size, by line and column.
pub struct CodeSpan {
    /// Line start and size
    pub line: (u32, u32),
    /// Column start and size
    pub column: (u32, u32),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_assert_span_size() {
        assert_eq!(std::mem::size_of::<Span>(), 8);
    }

    #[test]
    fn test_span_range() {
        let text = "abcdefg";
        //          ..^^^..
        let span = Span::new(2, 3);
        let range = span.to_range();
        println!("span: {span}; range: {range:?}");
        assert_eq!(&text[range], "cde");
    }

    #[test]
    fn test_line_column() {
        let text = concat!("abc\n", "  def\n", "ghi\n",);
        let span = Span::new(6, 3);

        assert_eq!("def", &text[span.to_range()]);

        let code_span = span.line_column(text);
        assert_eq!(2, code_span.line.0, "line start");
        assert_eq!(1, code_span.line.1, "line size");
        assert_eq!(3, code_span.column.0, "column start");
        assert_eq!(3, code_span.column.1, "column size");
    }

    #[test]
    fn test_line_column_windows_crlf() {
        let text = concat!("abc\r\n", "  def\r\n", "ghi\r\n",);
        let span = Span::new(7, 3);

        assert_eq!("def", &text[span.to_range()]);

        let code_span = span.line_column(text);
        assert_eq!(2, code_span.line.0);
        assert_eq!(1, code_span.line.1);
        assert_eq!(3, code_span.column.0);
        assert_eq!(3, code_span.column.1);
    }

    /// Ensure that a Span at the end of a string doesn't overflow.
    #[test]
    fn test_line_column_end() {
        let text = concat!("abc\n", "def");
        let span = Span::new(4, 3);

        assert_eq!("def", &text[span.to_range()]);

        let code_span = span.line_column(text);
        assert_eq!(2, code_span.line.0);
        assert_eq!(1, code_span.line.1);
        assert_eq!(1, code_span.column.0);
        assert_eq!(3, code_span.column.1);
    }

    /// Test case where a span crosses multiple lines
    /// until the end of string.
    ///
    /// This ensures the `CodeSpan` is build and finished correctly.
    ///
    /// Practically during tokenisation a `Span` probably won't
    /// ever cross a newline, but we ensure that it works to avoid
    /// surprises if spans were added together.
    #[test]
    fn test_line_column_end_of_string() {
        let text = concat!("abc\n", "def\n", "ghi");
        let span = Span::new(4, 7);

        assert_eq!("def\nghi", &text[span.to_range()]);

        let code_span = span.line_column(text);
        assert_eq!(2, code_span.line.0);
        assert_eq!(2, code_span.line.1);
        assert_eq!(1, code_span.column.0);
        // FIXME: Newline should be excluded
        assert_eq!(4, code_span.column.1);
    }

    // TODO: Test case: Span multiple lines and ends at end of string.
}
