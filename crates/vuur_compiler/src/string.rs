//! String utilities.

/// Escapes the ASCII control characters in the given string-like value.
pub fn escape_ascii(s: impl AsRef<str>) -> Result<String, UnescapeError> {
    let input = s.as_ref();
    let mut buf = String::with_capacity(input.len());

    for (_index, chr) in input.char_indices() {
        match chr {
            '\0' => buf.push_str(r"\0"),
            '\n' => buf.push_str(r"\n"),
            '\r' => buf.push_str(r"\r"),
            '\t' => buf.push_str(r"\t"),
            _ => buf.push(chr),
        }
    }

    Ok(buf)
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnescapeError;

impl std::error::Error for UnescapeError {}

impl std::fmt::Display for UnescapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to unescape string")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_escape_string() {
        assert_eq!(Ok(r"a\nb".to_string()), escape_ascii("a\nb"));
    }
}
