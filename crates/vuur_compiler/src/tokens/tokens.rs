//! Tokens

use crate::span::Span;

/// Represents a lexical token in the context of the compiler.
///
/// Tokens are the fundamental unit of source code produced by
/// the [Lexer](struct.Lexer.html).
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Slice a text fragment from the given source code.
    #[inline]
    pub fn fragment<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.to_range()]
    }
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
    DotDot,       // ..
    Ellipses,     // ...
    Add,          // +
    Sub,          // -
    Mul,          // *
    Div,          // /
    StarStar,     // **
    Eq,           // =
    EqEq,         // ==
    NotEq,        // !=
    ThinArrow,    // ->
    Ampersand,    // &
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;

    Ident,
    /// Reserved identifiers
    Keyword(Keyword),
    /// Number Literal
    Number,
    /// String Literal
    String,
    /// Part of an interpolated string, to the left or right of the expression.
    Interpolated,

    CommentLine,  // //
    CommentLeft,  // /*
    CommentRight, // */
    /// Content of a block comment.
    Comment,

    /// Spaces and tabs.
    Whitespace,
    /// Line-feed and optionally a carriage return
    Newline,
    /// End-of-file
    EOF,
    /// Unknown character was encoutered in the source.
    Unknown,
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind as T;

        match self {
            T::LeftParen        => write!(f, "("),
            T::RightParen       => write!(f, ")"),
            T::LeftBracket      => write!(f, "["),
            T::RightBracket     => write!(f, "]"),
            T::LeftBrace        => write!(f, "{{"),
            T::RightBrace       => write!(f, "}}"),
            T::Dot              => write!(f, "."),
            T::DotDot           => write!(f, ".."),
            T::Ellipses         => write!(f, "..."),
            T::Add              => write!(f, "+"),
            T::Sub              => write!(f, "-"),
            T::Mul              => write!(f, "*"),
            T::Div              => write!(f, "/"),
            T::StarStar         => write!(f, "**"),
            T::Eq               => write!(f, "="),
            T::EqEq             => write!(f, "=="),
            T::NotEq            => write!(f, "!="),
            T::ThinArrow        => write!(f, "->"),
            T::Ampersand        => write!(f, "&"),
            T::Comma            => write!(f, ","),
            T::Colon            => write!(f, ":"),
            T::Semicolon        => write!(f, ";"),
            T::Ident            => write!(f, "identifier"),
            T::Keyword(keyword) => std::fmt::Display::fmt(keyword, f),
            T::Number           => write!(f, "number"),
            T::String           => write!(f, "string"),
            T::Interpolated     => write!(f, "interpolated"),
            T::CommentLine      => write!(f, "//"),
            T::CommentLeft      => write!(f, "/*"),
            T::CommentRight     => write!(f, "*/"),
            T::Comment          => write!(f, "comment"),
            T::Whitespace       => write!(f, "whitespace"),
            T::Newline          => write!(f, "newline"),
            T::EOF              => write!(f, "end-of-file"),
            T::Unknown          => write!(f, "unknown"),
        }
    }
}

/// Reserved identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Else,      // if conditional else statement
    Func,      // function declaration statement
    If,        // if conditional statement
    Interface, // interface type declaration
    Return,    // block return statement
    Struct,    // struct type declaration
    Type,      // type declaration statement
    Var,       // variable declaration statement
}

impl<'a> TryFrom<&'a str> for Keyword {
    type Error = ();

    #[rustfmt::skip]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        use Keyword as K;
        match value {
            "else"        => Ok(K::Else),
            "func"        => Ok(K::Func),
            "if"          => Ok(K::If),
            "interface"   => Ok(K::Interface),
            "return"      => Ok(K::Return),
            "struct"      => Ok(K::Struct),
            "type"        => Ok(K::Type),
            // "break"      => Ok(K::Break),
            // "class"      => Ok(K::Class),
            // "construct"  => Ok(K::Construct),
            // "continue"   => Ok(K::Continue),
            // "false"      => Ok(K::False),
            // "for"        => Ok(K::For),
            // "foreign"    => Ok(K::Foreign),
            // "import"     => Ok(K::Import),
            // "is"         => Ok(K::Is),
            // "return"     => Ok(K::Return),
            // "static"     => Ok(K::Static),
            // "super"      => Ok(K::Super),
            // "this"       => Ok(K::This),
            // "true"       => Ok(K::True),
            "var"        => Ok(K::Var),
            // "while"      => Ok(K::While),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Keyword {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Keyword as K;
        let name = match self {
            K::Else         => "else",
            K::Func         => "func",
            K::If           => "if",
            K::Interface    => "interface",
            K::Return       => "return",
            K::Struct       => "struct",
            K::Type         => "type",
            // K::Break        => "break",
            // K::Class        => "class",
            // K::Construct    => "construct",
            // K::Continue     => "continue",
            // K::False        => "false",
            // K::For          => "for",
            // K::Foreign      => "foreign",
            // K::Import       => "import",
            // K::Is           => "is",
            // K::Return       => "return",
            // K::Static       => "static",
            // K::Super        => "super",
            // K::This         => "this",
            // K::True         => "true",
            K::Var          => "var",
            // K::While        => "while",
        };

        std::fmt::Display::fmt(name, f)
    }
}
