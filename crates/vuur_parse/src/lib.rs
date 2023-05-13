//! Parsing (syntactic analysis).
use crate::module::VuurModule;
use crate::stream::TokenError;
use crate::stream::TokenStream;
use vuur_lexer::span::BytePos;
use vuur_lexer::Lexer;

mod block;
pub mod cond;
pub mod delim;
pub mod error;
pub mod expr;
pub mod func;
pub mod ident;
pub mod module;
pub mod pprint;
pub mod stmt;
pub mod stream;
mod ty;
pub mod var;

pub fn parse_str(source: &str) -> ParseResult<VuurModule> {
    let lexer = Lexer::from_source(source);
    let mut stream = TokenStream::new(lexer);
    VuurModule::parse(&mut stream)
}

pub trait Parse {
    type Output;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output>;
}

type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    Syntax { msg: String },
    Token(crate::stream::TokenError),
}

/// Convenience function for creating syntax errors.
// TODO: Replace with macro that supports formatting.
#[inline(always)]
pub(crate) fn syntax_err(msg: impl ToString) -> ParseError {
    ParseError::Syntax { msg: msg.to_string() }
}

impl ParseError {
    /// Byte position (offset) and size of token in source
    /// related to this error.
    pub fn span(&self) -> (BytePos, u32) {
        todo!("span for syntax error")
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseError as E;

        match self {
            E::Syntax { msg } => write!(f, "syntax error: {}", msg),
            E::Token(err) => std::fmt::Display::fmt(err, f),
        }
    }
}

impl From<TokenError> for ParseError {
    fn from(token_error: TokenError) -> Self {
        ParseError::Token(token_error)
    }
}
