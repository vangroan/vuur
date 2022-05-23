use vuur_lexer::{Keyword, Token, TokenKind};

use crate::ident::Ident;
use crate::stream::TokenStream;
use crate::{syntax_err, Parse};

#[derive(Debug)]
pub struct Type {
    pub ref_: Option<Token>,
    pub kind: TypeKind,
}

#[derive(Debug)]
pub enum TypeKind {
    /// Type referred to by identifier.
    Ident(Ident),
    // TODO: interface
    // TODO: struct
    // TODO: func
}

impl Parse for Type {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> crate::ParseResult<Self::Output> {
        use Keyword as K;
        use TokenKind as T;

        input.ignore_many(T::Whitespace);
        let ref_ = input.consume(T::Ampersand).ok();
        input.ignore_many(T::Whitespace);

        input.reset_peek();
        // TODO: Replace syntax error with unexpected token error
        if let Some(kind) = input.peek().map(|t| t.kind) {
            println!("Type token after ref: {:?}", kind);
            match kind {
                T::Ident => {
                    let kind = TypeKind::Ident(Ident::parse(input)?);
                    Ok(Type { ref_, kind })
                }
                T::Keyword(keyword) => match keyword {
                    K::Interface => {
                        todo!("interface type declaration")
                    }
                    K::Struct => {
                        todo!("struct type declaration")
                    }
                    K::Func => {
                        todo!("function type declaration")
                    }
                    _ => Err(syntax_err("expected type declaration")),
                },
                T::EOF => Err(syntax_err("unexpected end-of-file")),
                _ => Err(syntax_err("expected type declaration")),
            }
        } else {
            Err(syntax_err("unexpected end-of-file"))
        }
    }
}
