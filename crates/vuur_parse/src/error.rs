use crate::ParseError;

/// Pretty format parsing error.
pub fn format_error(source: &str, err: ParseError) -> String {
    let mut s = String::new();

    // TODO: Error must have a token with a byte pos and size.
    // TODO: Scan string to count columns and rows.

    s
}

pub struct Error {
    pub kind: ParseError,
    pub(crate) inner: Box<ErrorInner>,
}

pub(crate) struct ErrorInner {
    span: vuur_lexer::span::Pos,
    filepath: Option<String>,
    cause: Option<Box<dyn std::error::Error + 'static>>,
}
