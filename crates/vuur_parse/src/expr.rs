//! Expression parsing

use std::convert::Infallible;

use vuur_lexer::{Token, TokenKind};

use crate::ident::Ident;
use crate::stream::TokenStream;
use crate::{syntax_err, Parse, ParseResult};

/// Token precedence.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Precedence {
    /// Tokens that terminate an expression
    /// should have a precedence of `None`.
    None = 0,
    Lowest = 1,
    Assignment = 2,    // =
    Conditional = 3,   // ?:
    LogicalOr = 4,     // ||
    LogicalAnd = 5,    // &&
    Equality = 6,      // == !=
    Is = 7,            // is
    Comparison = 8,    // < > <= >=
    BitwiseOr = 9,     // |
    BitwiseXor = 10,   // ^
    BitwiseAnd = 11,   // &
    BitwiseShift = 12, // << >>
    Range = 13,        // .. ...
    Term = 14,         // + -
    Factor = 15,       // * / %
    Unary = 16,        // - ! ~
    Call = 17,         // . () []
    Primary = 18,
}

impl Precedence {
    #[inline(always)]
    fn as_i32(&self) -> i32 {
        *self as i32
    }

    /// Get the precedence of the given token type in the context
    /// of the expression parser.
    fn of(kind: TokenKind) -> Precedence {
        use TokenKind as T;

        match kind {
            T::Number | T::Ident => Precedence::Lowest,
            T::Add | T::Sub => Precedence::Term,
            T::Mul | T::Div => Precedence::Factor,
            T::Eq => Precedence::Assignment,
            T::EqEq => Precedence::Equality,
            _ => Precedence::None,
        }
    }
}

impl TryFrom<i32> for Precedence {
    type Error = Infallible;

    #[rustfmt::skip]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use Precedence as P;
        match value {
            0  => Ok(P::None),
            1  => Ok(P::Lowest),
            2  => Ok(P::Assignment),
            3  => Ok(P::Conditional),
            4  => Ok(P::LogicalOr),
            5  => Ok(P::LogicalAnd),
            6  => Ok(P::Equality),
            7  => Ok(P::Is),
            8  => Ok(P::Comparison),
            9  => Ok(P::BitwiseOr),
            10 => Ok(P::BitwiseXor),
            11 => Ok(P::BitwiseAnd),
            12 => Ok(P::BitwiseShift),
            13 => Ok(P::Range),
            14 => Ok(P::Term),
            15 => Ok(P::Factor),
            16 => Ok(P::Unary),
            17 => Ok(P::Call),
            18 => Ok(P::Primary),
            _  => Ok(P::None),
        }
    }
}

impl std::fmt::Display for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.as_i32(), f)
    }
}

impl std::ops::Add<i32> for Precedence {
    type Output = Precedence;

    fn add(self, rhs: i32) -> Self::Output {
        Precedence::try_from(self.as_i32() + rhs).unwrap()
    }
}

/// Associativity is the precedence tie-breaker.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Associativity {
    Left,
    Right,
}

impl Associativity {
    /// Determine the associativity of the given token kind.
    fn of(token_ty: TokenKind) -> Associativity {
        if token_ty == TokenKind::Eq {
            Associativity::Right
        } else {
            Associativity::Left
        }
    }

    fn is_left(&self) -> bool {
        *self == Associativity::Left
    }

    fn is_right(&self) -> bool {
        *self == Associativity::Right
    }
}

#[derive(Debug)]
pub enum Expr {
    Unknown,
    Unary(UnaryOp),
    Binary(BinaryOp),
    Num(NumLit),
    Access(VarAccess),
}

/// Number literal.
#[derive(Debug)]
pub struct NumLit {
    pub token: Token,
}

/// Arithmetic operation with an expression on the right side.
#[derive(Debug)]
pub struct UnaryOp {
    pub operator: Token,
    pub rhs: Box<Expr>,
}

/// Arithmetic operation with an expression on either side.
#[derive(Debug)]
pub struct BinaryOp {
    pub operator: Token,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

/// Variable accessed/read.
#[derive(Debug)]
pub struct VarAccess {
    pub ident: Ident,
}

impl Parse for Expr {
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        Expr::parse_precedence(input, Precedence::Lowest)
    }
}

/// Recursive parsing methods
impl Expr {
    /// Entrypoint for the top-down precedence parser.
    ///
    /// The implementation is a straight forward Pratt parser.
    fn parse_precedence(input: &mut TokenStream, precedence: Precedence) -> ParseResult<Expr> {
        println!("Expr::parse_precedence");

        input.ignore_many(TokenKind::Whitespace);
        let token = input.next_token().ok_or_else(|| syntax_err("unexpected end-of-file"))?;

        // The current expression node is wrapped in `Option`
        // so that it can be moved into the recursive parser,
        // and the stack value replaced with the parsing result.
        let mut left = Some(Self::parse_prefix(input, token)?);

        input.ignore_many(TokenKind::Whitespace);
        input.reset_peek();

        while precedence <= input.peek().map(|t| Precedence::of(t.kind)).unwrap_or(Precedence::None) {
            // Option is so we can swap the stack value,
            // but doesn't make sense for the algorithm.
            debug_assert!(left.is_some(), "left hand side token is None");

            // Peeking advances a peek pointer inside the token stream,
            // so it needs to be reset otherwise we are inadvertently
            // looking further ahead.
            input.reset_peek();

            // There is no expression right of the last one, so we just return what we have.
            if let None = input.peek().map(|token| token.kind) {
                println!("Expr::parse_precedence; end-of-expression");
                return Ok(left.take().unwrap());
            }

            let token = input.next_token().ok_or_else(|| syntax_err("expression expected"))?;
            left = Some(Self::parse_infix(input, left.take().unwrap(), token)?);

            input.ignore_many(TokenKind::Whitespace);
        }

        input.reset_peek();
        println!(
            "Expr::parse_precedence; precedence high; {} <= {}; peek() -> {:?}",
            precedence,
            input.peek().map(|t| Precedence::of(t.kind)).unwrap_or(Precedence::None),
            {
                input.reset_peek();
                input.peek().map(|t| t.kind)
            }
        );
        Ok(left.take().unwrap())
    }

    /// Parse a prefix token in an expression.
    ///
    /// This function is analogous to a parselet.
    fn parse_prefix(input: &mut TokenStream, token: Token) -> ParseResult<Expr> {
        use TokenKind as T;

        println!("Expr::parse_prefix(_, {:?})", token.kind);

        match token.kind {
            T::Number => Expr::parse_number_literal(token).map(Expr::Num),
            T::Ident => Expr::parse_var_access(input, token).map(Expr::Access),
            T::Sub => {
                // Negate
                Expr::parse_precedence(input, Precedence::Unary)
                    .map(|right| UnaryOp {
                        operator: token,
                        rhs: Box::new(right),
                    })
                    .map(Expr::Unary)
            }
            // When this match fails, it means there is no parselet for the token, meaning
            // some invalid token is in an unexpected position.
            _ => Err(syntax_err("expression expected")),
        }
    }

    /// Parse an infix, postfix or mixfix operator.
    ///
    /// Includes non-obvious tokens like opening parentheses `(`.
    fn parse_infix(input: &mut TokenStream, left: Expr, token: Token) -> ParseResult<Expr> {
        use TokenKind as T;

        println!("Expr::parse_infix(_, {:?})", token.kind);

        let precedence = Precedence::of(token.kind);

        // Associativity is handled by adjusting the precedence.
        // Left associativity is achieved by increasing the precedence
        // by 1. This increases the threshold that any infix expressions
        // to our right must exceed.
        //
        // Right associativity can be achieved by keeping
        // the precedence the same, thus keeping the threshold any
        // subsequent infix expression need to exceed to be parsed.
        let binding_power = if Associativity::of(token.kind).is_left() { 1 } else { 0 };

        // Recurse back into expression parser to handle
        // the right hand side.
        //
        // The left hand side will wait for us here on
        // the call stack.
        let right = Self::parse_precedence(input, precedence + binding_power)?;

        match token.kind {
            T::Add | T::Sub | T::Mul | T::Div | T::Eq | T::EqEq => {
                // Binary Operator
                Ok(Expr::Binary(BinaryOp {
                    operator: token,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                }))
            }
            _ => Err(syntax_err("infix expression expected")),
        }
    }
}

impl Expr {
    fn parse_number_literal(token: Token) -> ParseResult<NumLit> {
        // TODO: Different number formats. binary, octal, decimal, hex, scientific
        Ok(NumLit { token })
    }

    fn parse_var_access(input: &mut TokenStream, token: Token) -> ParseResult<VarAccess> {
        println!("Expr::parse_var_access(_, {:?})", token.kind);
        debug_assert_eq!(token.kind, TokenKind::Ident, "expected identifier");

        // TODO: Parse member access or function call. eg. foo.bar; foo.bar()
        Ok(VarAccess {
            ident: Ident {
                text: input.token_fragment(&token).into(),
                token,
            },
        })
    }
}
