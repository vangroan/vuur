#[allow(dead_code)]
pub fn unescape_str(s: &str) -> String {
    let mut buf = String::new();

    for c in s.chars() {
        match c {
            '\t' => buf.push_str("\\t"),
            '\n' => buf.push_str("\\n"),
            '\r' => buf.push_str("\\r"),
            _ => buf.push(c),
        }
    }

    buf
}
