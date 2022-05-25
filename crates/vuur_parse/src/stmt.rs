//! Statements (ie. lines starting with keywords like `func`, `var`, `type`)

use vuur_lexer::{Keyword, TokenKind};

use crate::expr::Expr;
use crate::func::FuncDef;
use crate::stream::TokenStream;
use crate::{syntax_err, Parse, ParseResult};

/// Definition statement.
///
/// These are statements that declare bindings like `var` and
/// `type`, which may only appear at the top level of curly braced
/// blocks.
#[derive(Debug)]
pub enum DefStmt {
    Func(FuncDef),
    Type(),
    Simple(SimpleStmt),
}

/// Simple statement.
#[derive(Debug)]
pub enum SimpleStmt {
    Unknown,
    Expr(Expr),
}

impl Parse for DefStmt {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        use Keyword as K;
        use TokenKind as T;

        // Ignore empty lines
        input.ignore_many(T::Newline);
        input.ignore_many(T::Whitespace); // indentation

        if let Some(token) = input.peek() {
            println!("DefStmt: {:?}", token);
            if let T::Keyword(keyword) = token.kind {
                match keyword {
                    K::Func => Ok(Self::Func(FuncDef::parse(input)?)),
                    _ => Err(syntax_err(format!("unexpected keyword '{}'", keyword))),
                }
            } else {
                Ok(Self::Simple(SimpleStmt::parse(input)?))
            }
        } else {
            Err(syntax_err("unexpected end-of-file"))
        }
    }
}

impl Parse for SimpleStmt {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        if let Some(token) = input.next_token() {
            println!("unknown - {}", input.token_fragment(&token));
        }
        Ok(SimpleStmt::Unknown)
    }
}
