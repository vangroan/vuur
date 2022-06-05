//! Statements (ie. lines starting with keywords like `func`, `var`, `type`)

use vuur_lexer::{Keyword, TokenKind};

use crate::cond::IfStmt;
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
    Return(Expr),
    Type(),
    Simple(SimpleStmt),
}

/// Simple statement.
#[derive(Debug)]
pub enum SimpleStmt {
    Unknown,
    If(IfStmt),
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
                    K::Func => FuncDef::parse(input).map(DefStmt::Func),
                    K::Return => DefStmt::parse_return_stmt(input),
                    _ => SimpleStmt::parse(input).map(DefStmt::Simple),
                }
            } else {
                SimpleStmt::parse(input).map(DefStmt::Simple)
            }
        } else {
            Err(syntax_err("unexpected end-of-file"))
        }
    }
}

impl Parse for SimpleStmt {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        use Keyword as K;
        use TokenKind as T;

        input.reset_peek();
        if let Some(token) = input.peek() {
            println!("SimpleStmt: {:?}", token);
            if let T::Keyword(keyword) = token.kind {
                match keyword {
                    K::If => IfStmt::parse(input).map(SimpleStmt::If),
                    _ => Ok(SimpleStmt::Unknown),
                }
            } else {
                SimpleStmt::parse_expr_stmt(input)
            }
        } else {
            Err(syntax_err("unexpected end-of-file"))
        }
    }
}

impl DefStmt {
    fn parse_return_stmt(input: &mut TokenStream) -> ParseResult<DefStmt> {
        input.ignore_many(TokenKind::Whitespace);
        input.consume(TokenKind::Keyword(Keyword::Return))?;
        Expr::parse(input).map(DefStmt::Return)
    }
}

impl SimpleStmt {
    fn parse_expr_stmt(input: &mut TokenStream) -> ParseResult<SimpleStmt> {
        let expr = Expr::parse(input)?;

        // Expression statement is terminated by newline, eof or right-brace.
        input.ignore_many(TokenKind::Newline);

        Ok(SimpleStmt::Expr(expr))
    }
}
