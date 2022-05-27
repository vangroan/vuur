//! Parsing (syntactic analysis).
use crate::module::VuurModule;
use crate::stream::TokenError;
use crate::stream::TokenStream;
use vuur_lexer::Lexer;

mod block;
mod cond;
pub mod delim;
mod expr;
pub mod func;
pub mod ident;
pub mod module;
mod stmt;
pub mod stream;
mod ty;

pub fn parse_str(source: &str) -> ParseResult<VuurModule> {
    let lexer = Lexer::from_str(source);
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
