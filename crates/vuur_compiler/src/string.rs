//! String utilities.

pub fn unescape_string(s: impl AsRef<str>) -> Result<String, UnescapeError> {
    let input = s.as_ref();
    let mut buf = String::with_capacity(input.len());

    for (_index, chr) in input.char_indices() {
        match chr {
            '\n' => buf.push_str("\\n"),
            _ => buf.push(chr),
        }
    }

    Ok(buf)
}

#[derive(Debug)]
pub struct UnescapeError;

impl std::error::Error for UnescapeError {}

impl std::fmt::Display for UnescapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to unescape string")
    }
}
