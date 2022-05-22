//! Tokens.

use crate::span::BytePos;

#[derive(Debug)]
pub struct Token {
    pub offset: BytePos,
    pub size: u32, // in bytes
    pub kind: TokenKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Dot,          // .
    Add,          // +
    Sub,          // -
    Mul,          // *
    Div,          // /
    Eq,           // =
    EqEq,         // ==
    NotEq,        // !=
    Ampersand,    // &
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;

    Ident,
    /// Reserved identifiers
    Keyword(Keyword),
    /// Number Literal
    Number,

    /// Spaces and tabs.
    Whitespace,
    /// Line-feed and optionally a carriage return
    Newline,
    /// End-of-file
    EOF,
    /// Unknown character was encoutered in the source.
    Unknown,
}

/// Reserved identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Func, // function declaration statement
    Type, // type declaration statement
}

impl Token {
    /// Slice a text fragment from the given source code.
    #[inline]
    pub fn fragment<'a>(&self, source: &'a str) -> &'a str {
        let start = self.offset.0 as usize;
        let end = start + self.size as usize;
        &source[start..end]
    }
}

/// Formatting token kind to a human readable description
/// that can be used in error messages and assorted text
/// intended for the user.
///
/// Tokens that would be difficult to discern in a terminal,
/// like whitespace or newlines, should be output as something
/// readable.
impl std::fmt::Display for TokenKind {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use TokenKind as T;

        match self {
            T::LeftParen        => write!(f, "("),
            T::RightParen       => write!(f, ")"),
            T::LeftBracket      => write!(f, "["),
            T::RightBracket     => write!(f, "]"),
            T::LeftBrace        => write!(f, "{{"),
            T::RightBrace       => write!(f, "}}"),
            T::Dot              => write!(f, "."),
            T::Add              => write!(f, "+"),
            T::Sub              => write!(f, "-"),
            T::Mul              => write!(f, "*"),
            T::Div              => write!(f, "/"),
            T::Eq               => write!(f, "="),
            T::EqEq             => write!(f, "=="),
            T::NotEq            => write!(f, "!="),
            T::Ampersand        => write!(f, "&"),
            T::Comma            => write!(f, ","),
            T::Colon            => write!(f, ":"),
            T::Semicolon        => write!(f, ";"),
            T::Ident            => write!(f, "identifier"),
            T::Keyword(keyword) => std::fmt::Display::fmt(keyword, f),
            T::Number           => write!(f, "number"),
            // T::String           => write!(f, "string"),
            // T::Interpolated     => write!(f, "interpolated"),
            // T::CommentLine      => write!(f, "//"),
            // T::CommentLeft      => write!(f, "/*"),
            // T::CommentRight     => write!(f, "*/"),
            // T::Comment          => write!(f, "comment"),
            T::Whitespace       => write!(f, "whitespace"),
            T::Newline          => write!(f, "newline"),
            T::EOF              => write!(f, "end-of-file"),
            T::Unknown          => write!(f, "unknown"),
        }
    }
}

impl<'a> TryFrom<&'a str> for Keyword {
    type Error = ();

    #[rustfmt::skip]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        use Keyword as K;
        match value {
            "func"        => Ok(K::Func),
            "type"        => Ok(K::Type),
            // "break"      => Ok(K::Break),
            // "class"      => Ok(K::Class),
            // "construct"  => Ok(K::Construct),
            // "continue"   => Ok(K::Continue),
            // "false"      => Ok(K::False),
            // "for"        => Ok(K::For),
            // "foreign"    => Ok(K::Foreign),
            // "if"         => Ok(K::If),
            // "import"     => Ok(K::Import),
            // "is"         => Ok(K::Is),
            // "return"     => Ok(K::Return),
            // "static"     => Ok(K::Static),
            // "super"      => Ok(K::Super),
            // "this"       => Ok(K::This),
            // "true"       => Ok(K::True),
            // "var"        => Ok(K::Var),
            // "while"      => Ok(K::While),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Keyword {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Keyword as K;
        match self {
            K::Func         => write!(f, "func"),
            K::Type         => write!(f, "type"),
            // K::Break        => write!(f, "break"),
            // K::Class        => write!(f, "class"),
            // K::Construct    => write!(f, "construct"),
            // K::Continue     => write!(f, "continue"),
            // K::False        => write!(f, "false"),
            // K::For          => write!(f, "for"),
            // K::Foreign      => write!(f, "foreign"),
            // K::If           => write!(f, "if"),
            // K::Import       => write!(f, "import"),
            // K::Is           => write!(f, "is"),
            // K::Return       => write!(f, "return"),
            // K::Static       => write!(f, "static"),
            // K::Super        => write!(f, "super"),
            // K::This         => write!(f, "this"),
            // K::True         => write!(f, "true"),
            // K::Var          => write!(f, "var"),
            // K::While        => write!(f, "while"),
        }
    }
}
