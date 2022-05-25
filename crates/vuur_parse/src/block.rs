use vuur_lexer::TokenKind;

use crate::{stmt::DefStmt, syntax_err, Parse};

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<DefStmt>,
}

impl Parse for Block {
    type Output = Self;

    fn parse(input: &mut crate::stream::TokenStream) -> crate::ParseResult<Self::Output> {
        use TokenKind as T;

        // TODO: Single line block containing one expression

        input.ignore_many(T::Whitespace);
        input.consume(T::LeftBrace)?;

        let mut stmts = vec![];

        while let Some(token) = input.peek() {
            match token.kind {
                T::Newline => {
                    // When the statement starts with a newline, it's blank.
                    input.next_token();
                    continue;
                }
                T::RightBrace => break,
                _ => stmts.push(DefStmt::parse(input)?),
            }
        }

        input.ignore_many(T::Whitespace);
        input.consume(T::RightBrace)?;
        input.ignore_many(T::Whitespace);

        match input.next_token().map(|t| t.kind) {
            Some(T::Newline | T::Semicolon | T::EOF) | None => {
                // Valid block termination
                Ok(Block { stmts })
            }
            Some(kind) => Err(syntax_err(
                format!(
                    "unexpected token {}; block closing brace must be followed by newline, semicolon or eof.",
                    kind
                )
                .to_owned(),
            )),
        }
    }
}
