//! Tokens

use crate::span::Span;

/// Represents a lexical token in the context of the compiler.
///
/// Tokens are the fundamental unit of source code produced by
/// the [Lexer](struct.Lexer.html).
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { kind, span } = self;
        write!(f, "{kind} {span}")
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
    #[deprecated]
    Whitespace,
    /// Line-feed and optionally a carriage return
    Newline,
    /// End-of-file
    EOF,
    /// Unknown character was encoutered in the source.
    Unknown,
}

impl TokenKind {
    #[rustfmt::skip]
    pub fn decode(lexeme: &str) -> Result<Self, DecodeError> {
        match lexeme {
            "left_paren"    => Ok(Self::LeftParen),
            "right_paren"   => Ok(Self::RightParen),
            "left_bracket"  => Ok(Self::LeftBracket),
            "right_bracket" => Ok(Self::RightBracket),
            "left_brace"    => Ok(Self::LeftBrace),
            "right_brace"   => Ok(Self::RightBrace),
            "dot"           => Ok(Self::Dot),
            "dot_dot"       => Ok(Self::DotDot),
            "ellipses"      => Ok(Self::Ellipses),
            "add"           => Ok(Self::Add),
            "sub"           => Ok(Self::Sub),
            "mul"           => Ok(Self::Mul),
            "div"           => Ok(Self::Div),
            "star_star"     => Ok(Self::StarStar),
            "eq"            => Ok(Self::Eq),
            "eq_eq"         => Ok(Self::EqEq),
            "not_eq"        => Ok(Self::NotEq),
            "thin_arrow"    => Ok(Self::ThinArrow),
            "ampersand"     => Ok(Self::Ampersand),
            "comma"         => Ok(Self::Comma),
            "colon"         => Ok(Self::Colon),
            "semicolon"     => Ok(Self::Semicolon),
            // ---
            "identifier"    => Ok(Self::Ident),
            "number"        => Ok(Self::Number),
            "string"        => Ok(Self::String),
            "interp_str"    => Ok(Self::Interpolated),
            // ---
            "comment_line"  => Ok(Self::CommentLine),
            "comment_left"  => Ok(Self::CommentLeft),
            "comment_right" => Ok(Self::CommentRight),
            "comment"       => Ok(Self::Comment),
            // ---
            "newline"       => Ok(Self::Newline),
            "eof"           => Ok(Self::EOF),
            "unknown"       => Ok(Self::Unknown),

            _ => {
                Keyword::try_from(lexeme).map(TokenKind::Keyword).map_err(|_| DecodeError)
            },
        }
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

/// Utility for decoding a [Token] and its [Span] from a line of text.
///
/// This is not intended to be used in the compiler itself. It is for
/// conveniently defining test cases for the lexical analysis.
///
/// ```non-rust
/// if         0 2
/// number     3 1
/// gt         5 1
/// left-brace 7 1
/// ```
#[derive(Debug)]
pub(super) struct TokenDecoder;

#[allow(dead_code)]
impl TokenDecoder {
    /// Decode multiple lines.
    pub fn decode_lines(text: &str) -> Result<Vec<Token>, DecodeError> {
        let tokens: Vec<Token> = text
            .split('\n')
            .map(|line| line.trim_matches('\r')) // windows :(
            .filter(|line| !line.is_empty())
            .map(|line| (line, TokenDecoder::decode_token(line)))
            .map(|(line, result)| result.unwrap_or_else(|_| panic!("failed to decode line: '{line}'")))
            .collect();

        Ok(tokens)
    }

    /// Decode a single line into a token.
    pub fn decode_token(line: &str) -> Result<Token, DecodeError> {
        let mut scanner = line.trim().split_whitespace();

        let token_kind = scanner.next().ok_or_else(|| DecodeError)?;
        let start = scanner.next().ok_or_else(|| DecodeError)?;
        let size = scanner.next().ok_or_else(|| DecodeError)?;

        Ok(Token::new(
            TokenKind::decode(token_kind)?,
            Span::new(
                u32::from_str_radix(start, 10).unwrap(),
                u32::from_str_radix(size, 10).unwrap(),
            ),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecodeError;

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error decoding token")
    }
}

#[cfg(test)]
mod test {
    use super::Keyword::*;
    use super::TokenKind::*;
    use super::*;

    #[test]
    fn test_token_decode() {
        const CASES: &[(&str, Token)] = &[
            ("left_paren    0  1", Token::new(LeftParen, Span::new(0, 1))),
            ("right_paren   1  1", Token::new(RightParen, Span::new(1, 1))),
            ("left_bracket  2  1", Token::new(LeftBracket, Span::new(2, 1))),
            ("right_bracket 3  1", Token::new(RightBracket, Span::new(3, 1))),
            ("left_brace    4  1", Token::new(LeftBrace, Span::new(4, 1))),
            ("right_brace   5  1", Token::new(RightBrace, Span::new(5, 1))),
            ("dot           6  1", Token::new(Dot, Span::new(6, 1))),
            ("dot_dot       7  1", Token::new(DotDot, Span::new(7, 1))),
            ("dot_dot       8  1", Token::new(DotDot, Span::new(8, 1))),
            ("ellipses      9  1", Token::new(Ellipses, Span::new(9, 1))),
            ("add          10  1", Token::new(Add, Span::new(10, 1))),
            ("sub          11  1", Token::new(Sub, Span::new(11, 1))),
            ("mul          12  1", Token::new(Mul, Span::new(12, 1))),
            ("div          13  1", Token::new(Div, Span::new(13, 1))),
            ("star_star    14  1", Token::new(StarStar, Span::new(14, 1))),
            ("eq           15  1", Token::new(Eq, Span::new(15, 1))),
            ("eq_eq        16  1", Token::new(EqEq, Span::new(16, 1))),
            ("not_eq       17  1", Token::new(NotEq, Span::new(17, 1))),
            ("thin_arrow   18  1", Token::new(ThinArrow, Span::new(18, 1))),
            ("ampersand    18  1", Token::new(Ampersand, Span::new(18, 1))),
            ("comma        19  1", Token::new(Comma, Span::new(19, 1))),
            ("colon        20  1", Token::new(Colon, Span::new(20, 1))),
            ("semicolon    21  1", Token::new(Semicolon, Span::new(21, 1))),
            // ---
            ("identifier   22  1", Token::new(Ident, Span::new(22, 1))),
            ("number       23  1", Token::new(Number, Span::new(23, 1))),
            ("string       24  1", Token::new(String, Span::new(24, 1))),
            ("interp_str   25  1", Token::new(Interpolated, Span::new(25, 1))),
            // ---
            ("comment_line   26  1", Token::new(CommentLine, Span::new(26, 1))),
            ("comment_left   27  1", Token::new(CommentLeft, Span::new(27, 1))),
            ("comment_right  28  1", Token::new(CommentRight, Span::new(28, 1))),
            ("comment        29  1", Token::new(Comment, Span::new(29, 1))),
            // ---
            ("newline      30  1", Token::new(Newline, Span::new(30, 1))),
            ("eof          31  1", Token::new(EOF, Span::new(31, 1))),
            ("unknown      32  1", Token::new(Unknown, Span::new(32, 1))),
            // keywords ---
            ("else         33  4", Token::new(Keyword(Else), Span::new(33, 4))),
            ("func         37  4", Token::new(Keyword(Func), Span::new(37, 4))),
            ("if           41  2", Token::new(Keyword(If), Span::new(41, 2))),
            ("interface    43  9", Token::new(Keyword(Interface), Span::new(43, 9))),
            ("return       52  6", Token::new(Keyword(Return), Span::new(52, 6))),
            ("struct       58  6", Token::new(Keyword(Struct), Span::new(58, 6))),
            ("type         64  4", Token::new(Keyword(Type), Span::new(64, 4))),
            ("var          68  3", Token::new(Keyword(Var), Span::new(68, 3))),
        ];

        for (line, expected) in CASES {
            assert_eq!(
                TokenDecoder::decode_token(line),
                Ok(expected.clone()),
                "case: {line} -> {expected}"
            );
        }
    }
}
