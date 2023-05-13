// Variable definition statement.
use vuur_lexer::{Keyword, TokenKind};

use crate::expr::Expr;
use crate::ident::Ident;
use crate::stream::TokenStream;
use crate::ty::Type;
use crate::{Parse, ParseResult};

#[derive(Debug)]
pub struct VarDef {
    pub name: Ident,
    pub ty: Option<Type>,
    pub rhs: Expr,
}

impl Parse for VarDef {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        use Keyword as K;
        use TokenKind as T;

        input.ignore_many(T::Whitespace);

        assert!(matches!(input.peek_kind(), Some(T::Keyword(K::Var))));

        // keyword (var)
        input.consume(T::Keyword(K::Var))?;
        input.ignore_many(T::Whitespace);

        // name
        let name = Ident::parse(input)?;
        input.ignore_many(T::Whitespace);

        // operator (eq)
        input.consume(T::Eq)?;
        input.ignore_many(T::Whitespace);

        // rhs
        let rhs = Expr::parse(input)?;

        Ok(VarDef {
            name,
            // TODO: type expression
            ty: None,
            rhs,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::expr::OperatorKind;
    use vuur_lexer::Lexer;

    #[test]
    fn test_var_def_parse() {
        let lexer = Lexer::from_source("var a = b + c");
        let mut stream = TokenStream::new(lexer);

        let var_def = VarDef::parse(&mut stream).expect("parsing variable definition statement");

        assert_eq!(var_def.name.text, "a");

        // TODO: assert type expression

        let add_expr = var_def.rhs.expr_bin_op().unwrap();
        assert_eq!(add_expr.operator.kind, OperatorKind::Add);
        assert_eq!(add_expr.lhs.expr_name_access().unwrap().ident.text, "b");
        assert_eq!(add_expr.rhs.expr_name_access().unwrap().ident.text, "c");
    }
}
