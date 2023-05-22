use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Span {
    pub index: u32,
    pub size: u32,
}

impl Span {
    /// Returns the exclusive range for the span, which
    /// can be used to slice a string.
    pub fn to_range(&self) -> Range<usize> {
        let start = self.index as usize;
        let end = start + self.size as usize;
        start..end
    }
}

impl Span {
    pub fn new(index: u32, size: u32) -> Self {
        Self { index, size }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { index, size } = self.clone();
        write!(f, "({index}, {size})")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_span_range() {
        let text = "abcdefg";
        //                ..^^^..
        let span = Span::new(2, 3);
        let range = span.to_range();
        println!("span: {span}; range: {range:?}");
        assert_eq!(&text[range], "cde");
    }
}
