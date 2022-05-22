//! Identifier.

use smol_str::SmolStr;
use vuur_lexer::{Token, TokenKind};

use crate::{stream::TokenStream, Parse, ParseResult};

#[derive(Debug)]
pub struct Ident {
    pub text: SmolStr,
    pub token: Token,
}

impl Parse for Ident {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        let token = input.consume(TokenKind::Ident)?;
        let text = SmolStr::from(input.token_fragment(&token));

        Ok(Ident { text, token })
    }
}
