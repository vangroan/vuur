//! Function declarations.

use vuur_lexer::{Keyword, TokenKind};

use crate::block::Block;
use crate::delim::Delimited;
use crate::ident::Ident;
use crate::stream::TokenStream;
use crate::ty::Type;
use crate::{Parse, ParseResult};

/// Function definition statement.
#[derive(Debug)]
pub struct FuncDef {
    pub name: Ident,
    pub args: Delimited<FuncArg, Separator>,
    pub rtn: Option<FuncRtn>,
    pub body: Block,
}

#[derive(Debug)]
pub struct FuncArg {
    pub name: Ident,
    pub ty: Ident,
    pub is_ref: bool,
}

#[derive(Debug)]
pub struct Separator;

#[derive(Debug)]
pub struct FuncRtn {
    pub ty: Type,
}

impl Parse for FuncDef {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        use Keyword as K;
        use TokenKind as T;

        input.ignore_many(T::Whitespace);

        // keyword
        input.consume(T::Keyword(K::Func))?;
        input.ignore_many(T::Whitespace);

        // name
        let name = Ident::parse(input)?;
        input.ignore_many(T::Whitespace);

        // arguments
        input.consume(T::LeftParen)?;
        let args = Delimited::<FuncArg, Separator>::parse(input)?;
        input.ignore_many(T::Whitespace);
        input.consume(T::RightParen)?;
        input.ignore_many(T::Whitespace);

        // optional return
        let rtn = if input.consume(T::ThinArrow).is_ok() {
            let ty = Type::parse(input)?;
            Some(FuncRtn { ty })
        } else {
            None
        };

        // body
        let body = Block::parse(input)?;

        Ok(FuncDef { name, args, rtn, body })
    }
}

impl Parse for FuncArg {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        use TokenKind as T;

        input.ignore_many(T::Whitespace);
        let name = Ident::parse(input)?;
        input.ignore_many(T::Whitespace);
        input.consume(T::Colon)?;
        input.ignore_many(T::Whitespace);
        let is_ref = input.consume(TokenKind::Ampersand).is_ok();
        input.ignore_many(T::Whitespace);
        let ty = Ident::parse(input)?;

        Ok(FuncArg { name, ty, is_ref })
    }
}

impl Parse for Separator {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        input.consume(TokenKind::Comma)?;
        Ok(Separator)
    }
}
