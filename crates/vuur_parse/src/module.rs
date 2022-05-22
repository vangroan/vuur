//! Script module, not Rust module.

use vuur_lexer::TokenKind;

use crate::{stmt::DefStmt, stream::TokenStream, Parse, ParseResult};

#[derive(Debug)]
pub struct VuurModule {
    pub stmts: Vec<DefStmt>,
}

impl Parse for VuurModule {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        use TokenKind as T;

        input.reset_peek();
        let mut stmts = vec![];

        while let Some(token) = input.peek() {
            match token.kind {
                T::Newline => {
                    // When the statement starts with a newline, it's blank.
                    input.next_token();
                    continue;
                }
                T::EOF => break,
                _ => stmts.push(DefStmt::parse(input)?),
            }
        }

        Ok(VuurModule { stmts })
    }
}
