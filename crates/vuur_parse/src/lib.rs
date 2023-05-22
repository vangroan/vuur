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
pub mod ty;
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

/// Convenience macro for declaring type safe identifiers.
///
/// ```
/// # use vuur_parse::declare_id;
/// declare_id!(struct FuncId);
/// let func_id = FuncId::new(42);
/// assert_eq!(func_id.as_u32(), 42);
/// ```
///
/// Supports a visibility modifier.
///
/// ```
/// # use vuur_parse::declare_id;
/// declare_id!(pub(crate) struct LocalId);
/// declare_id!(pub struct TypeId);
/// # let id = LocalId::new(42);
/// # (id.as_u32(), 42);
/// # let id = TypeId::new(42);
/// # (id.as_u32(), 42);
/// ```
#[macro_export]
macro_rules! declare_id {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(transparent)]
        $vis struct $name(u32);

        impl $name {
            #[inline]
            $vis const fn new(value: u32) -> Self {
                Self(value)
            }

            #[inline]
            $vis const fn as_u32(self) -> u32 {
                self.0 as u32
            }

            #[inline]
            $vis const fn as_usize(self) -> usize {
                self.0 as usize
            }
        }

        impl Into<u32> for $name {
            fn into(self) -> u32 {
                self.as_u32()
            }
        }

        impl Into<usize> for $name {
            fn into(self) -> usize {
                self.as_usize()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}
