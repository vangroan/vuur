//! Function declarations.

use vuur_lexer::{Keyword, TokenKind};

use crate::stream::TokenStream;
use crate::{Parse, ParseResult};

/// Function definition statement.
#[derive(Debug)]
pub struct FuncDef {
    // TODO: Ident
    pub name: String,
    pub args: Vec<FuncArg>,
}

#[derive(Debug)]
pub struct FuncArg {
    // TODO: Ident
    pub name: String,
    pub ty: String,
    pub is_ref: String,
}

impl Parse for FuncDef {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        use Keyword as K;
        use TokenKind as T;

        input.match_kind(T::Whitespace);
        input.consume(T::Keyword(K::Func))?;
        input.match_kind(T::Whitespace);
        input.match_kind(T::LeftParen);

        input.match_kind(T::RightParen);

        todo!()
    }
}
