//! Expression

use crate::{stream::TokenStream, Parse, ParseResult};

#[derive(Debug)]
pub enum Expr {
    Unknown,
    Unary,
    Binary,
}

impl Parse for Expr {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        Ok(Expr::Unknown)
    }
}
