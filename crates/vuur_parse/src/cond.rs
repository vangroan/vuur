//! Conditionals

use vuur_lexer::{Keyword, TokenKind};

use crate::{block::Block, expr::Expr, stream::TokenStream, Parse, ParseResult};

#[derive(Debug)]
pub struct IfStmt {
    pub cond: Expr,
    pub body: Block,
    pub else_: ElseStmt,
}

#[derive(Debug)]
pub enum ElseStmt {
    Empty,
    Else { body: Block },
    ElseIf(Box<IfStmt>),
}

impl Parse for IfStmt {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        println!("IfStmt::parse");

        use Keyword as K;
        use TokenKind as T;

        input.ignore_many(T::Whitespace);

        // keyword
        input.consume(T::Keyword(K::If))?;
        input.ignore_many(T::Whitespace);

        // conditional expression
        let cond = Expr::parse(input)?;
        input.ignore_many(T::Whitespace);

        // body
        let body = Block::parse(input)?;
        input.ignore_many(T::Whitespace);

        // optional else
        let else_ = if let Some(T::Keyword(K::Else)) = input.peek().map(|t| t.kind) {
            // first peek advances an internal peek token to second lookahead character
            if let Some(T::Keyword(K::If)) = input.peek().map(|t| t.kind) {
                ElseStmt::ElseIf(Box::new(IfStmt::parse(input)?))
            } else {
                ElseStmt::Else {
                    body: Block::parse(input)?,
                }
            }
        } else {
            ElseStmt::Empty
        };

        Ok(IfStmt { cond, body, else_ })
    }
}
