//! Expression parsing

use std::convert::Infallible;

use vuur_lexer::{Keyword, Token, TokenKind};

use crate::block::Block;
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
            T::Dot | T::LeftParen | T::LeftBracket => Precedence::Call,
            // Terminators
            T::RightParen | T::RightBracket => Precedence::None,
            T::Comma => Precedence::None,
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
}

#[derive(Debug)]
pub enum Expr {
    Unknown,
    Unary(UnaryOp),
    Binary(BinaryOp),
    Assign(Assign),
    Num(NumLit),
    Group(Group),
    NameAccess(NameAccess),
    MemberAccess(MemberAccess),
    MemberAssign(MemberAssign),
    Call(Call),
    /// Raw inlined bytecode.
    Bytecode(Vec<u32>),
}

/// Arithmetic operator kind.
#[derive(Debug, PartialEq, Eq)]
pub enum OperatorKind {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Equals,
}

/// Number literal.
#[derive(Debug)]
pub struct NumLit {
    pub token: Token,
    pub value: i32,
}

/// Grouped expression between parentises "(expr)"
#[derive(Debug)]
pub struct Group {
    pub expr: Box<Expr>,
}

/// Arithmetic operator
#[derive(Debug)]
pub struct Operator {
    pub kind: OperatorKind,
    pub token: Token,
}

/// Arithmetic operation with an expression on the right side.
#[derive(Debug)]
pub struct UnaryOp {
    // pub operator: Token,
    pub operator: Operator,
    pub rhs: Box<Expr>,
}

/// Arithmetic operation with an expression on either side.
#[derive(Debug)]
pub struct BinaryOp {
    pub operator: Operator,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

/// Assignment expression.
///
/// A bit special relative to out usual binary operation.
///
/// Left-hand-side of this expression is an l-value, and cannot be evaluated
/// to a value like a regular r-value expression. It only produces an address
/// to be used for a store instruction.
#[derive(Debug)]
pub struct Assign {
    pub operator: Token,
    pub lhs: Ident,
    pub rhs: Box<Expr>,
}

/// Variable accessed/read.
#[derive(Debug)]
pub struct NameAccess {
    pub ident: Ident,
}

#[derive(Debug)]
pub enum MemberPath {
    Name(Ident),
    Path(Box<MemberAccess>),
}

/// Member of object accessed/read.
///
/// ```not-rust
/// foo.bar
///
///        delim:"."
///     ┌──────┴──────┐
///     │             │
/// path:"foo"    name:"bar"
/// ```
///
/// `path` can either be a simple identifier, or a
/// tree of further member access nodes forming a chain.
///
/// ```non-rust
/// foo.bar.baz
///
///             delim:"."
///           ┌─────┴─────┐
///           │           │
///          path     name:"baz"
///           │
///       delim:"."
///     ┌─────┴─────┐
///     │           │
/// path:"foo"  name:"bar"
/// ```
///
/// Parts to the left are deeper down the tree, so that they
/// are evaluated first.
#[derive(Debug)]
pub struct MemberAccess {
    pub delim: Token,
    pub path: MemberPath,
    pub name: Ident,
}

/// Member setter assignment.
///
/// ```not-rust
/// foo.bar = 42
/// ```
#[derive(Debug)]
pub struct MemberAssign {
    /// Either the owner of the member, or a chain of [`MemberAccess`].
    pub path: MemberPath,
    // Delimiter separating path and member name.
    pub delim: Token,
    /// Member name being accessed
    pub name: Ident,
    pub operator: Token,
    pub rhs: Box<Expr>,
}

/// Call to a function.
///
/// ```non-rust
/// callee("a", 2, true)
/// ```
#[derive(Debug)]
pub struct Call {
    /// Expression that evaluates to a callable.
    pub callee: Box<Expr>,
    pub args: Vec<CallArg>,
}

/// Call argument.
#[derive(Debug)]
pub enum CallArg {
    /// Argument to a call without name specified.
    Simple(Expr),
    /// Argument to a call with the callee's argument
    /// name explicitly specified.
    ///
    /// ```not-rust
    /// foo.bar(a: "baz", b: 1, c: false)
    /// ```
    Named { name: Ident, rhs: Expr },
    /// Special syntax to pass a callable block to a function
    /// that takes one argument.
    ///
    /// ```not-rust
    /// foo.bar {
    ///     return "hello callback"
    /// }
    /// ```
    Block(Block),
}

/// Seperator for member access.
#[derive(Debug)]
pub struct MemberSeparator;

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
        println!("Expr::parse_precedence(_, {:?})", precedence);

        input.ignore_many(TokenKind::Whitespace);
        let token = input.next_token().ok_or_else(|| syntax_err("unexpected end-of-file"))?;

        let mut left = Self::parse_prefix(input, token)?;

        input.ignore_many(TokenKind::Whitespace);
        input.reset_peek();

        while precedence <= input.peek().map(|t| Precedence::of(t.kind)).unwrap_or(Precedence::None) {
            // Option is so we can swap the stack value,
            // but doesn't make sense for the algorithm.
            // debug_assert!(left.is_some(), "left hand side token is None");

            // Peeking advances a peek pointer inside the token stream,
            // so it needs to be reset otherwise we are inadvertently
            // looking further ahead.
            input.reset_peek();

            // There is no expression right of the last one, so we just return what we have.
            if input.peek().map(|t| t.kind).is_none() {
                println!("Expr::parse_precedence; end-of-expression");
                return Ok(left);
            }

            let token = input.next_token().ok_or_else(|| syntax_err("expression expected"))?;
            left = Self::parse_infix(input, left, token)?;

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
        Ok(left)
    }

    /// Parse a prefix token in an expression.
    ///
    /// This function is analogous to a parselet.
    fn parse_prefix(input: &mut TokenStream, token: Token) -> ParseResult<Expr> {
        use Keyword as K;
        use TokenKind as T;

        println!("Expr::parse_prefix(_, {:?})", token.kind);

        match token.kind {
            T::Number => Expr::parse_number_literal(input, token).map(Expr::Num),
            T::LeftParen => Expr::parse_group(input).map(Expr::Group),
            T::Ident => Expr::parse_postfix(input, token),
            T::Keyword(K::Func) => todo!("anonymous function"),
            T::Sub => {
                // Negate
                let kind = match token.kind {
                    T::Sub => OperatorKind::Neg,
                    _ => unreachable!("all outer match cases must be covered in inner match"),
                };

                let operator = Operator { kind, token };

                Expr::parse_precedence(input, Precedence::Unary)
                    .map(|right| UnaryOp {
                        operator,
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
            // Binary Operators
            T::Add | T::Sub | T::Mul | T::Div | T::Eq | T::EqEq => {
                // FIXME: Does `Eq` for assignment belong here when it's covered by postfix?
                let kind = match token.kind {
                    T::Add => OperatorKind::Add,
                    T::Sub => OperatorKind::Sub,
                    T::Mul => OperatorKind::Mul,
                    T::Div => OperatorKind::Div,
                    T::Eq => OperatorKind::Assign,
                    T::EqEq => OperatorKind::Equals,
                    _ => unreachable!("all outer match cases must be covered in inner match"),
                };

                let operator = Operator { kind, token };

                let binary_op = BinaryOp {
                    operator,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                };

                Ok(Expr::Binary(binary_op))
            }
            _ => Err(syntax_err("infix expression expected")),
        }
    }
}

impl Expr {
    fn parse_number_literal(input: &mut TokenStream, token: Token) -> ParseResult<NumLit> {
        // TODO: Different number formats. binary, octal, decimal, hex, scientific
        let fragment = input.token_fragment(&token);
        let value = i32::from_str_radix(fragment, 10)
            .map_err(|err| syntax_err(format!("failed to parse number literal: {}", err)))?;
        Ok(NumLit { token, value })
    }

    /// Parse expression contained in parentheses.
    fn parse_group(input: &mut TokenStream) -> ParseResult<Group> {
        println!("Expr::parse_group(_)");
        let expr = Box::new(Expr::parse(input)?);
        input.consume(TokenKind::RightParen)?;
        Ok(Group { expr })
    }

    /// Parse a postfix expression, triggered by encoutering a variable name.
    ///
    /// Depending on what follows the variable's identifier, the bare
    /// name can be part of the following:
    ///
    /// - dot delimited member access
    /// - function call
    fn parse_postfix(input: &mut TokenStream, token: Token) -> ParseResult<Expr> {
        use TokenKind as T;

        println!("Expr::parse_name(_, {:?})", token.kind);
        debug_assert_eq!(token.kind, T::Ident, "expected identifier");

        // Start the parser with an initial expression.
        let mut expr = Expr::NameAccess(NameAccess {
            ident: Ident {
                text: input.token_fragment(&token).into(),
                token,
            },
        });

        loop {
            input.ignore_many(TokenKind::Whitespace);

            expr = match input.peek().map(|t| t.kind) {
                Some(T::Eq) => {
                    // Assignment expression.
                    //
                    // LHS of assignment is special, because it is an l-value
                    // and does not evaluate to a resulting value.
                    //
                    // When we encounter an equality token we need to "rewind"
                    // and change the previous expression.
                    match expr {
                        Expr::NameAccess(NameAccess { ident }) => {
                            // Simple assignment where the LHS is a variable name.
                            // foobar = 42
                            let operator = input.consume(T::Eq)?;
                            let lhs = ident;
                            let rhs = Expr::parse(input).map(Box::new)?;
                            Expr::Assign(Assign { operator, lhs, rhs })
                        }
                        Expr::MemberAccess(MemberAccess { delim, path, name }) => {
                            // Member setter where the LHS is a member of an object.
                            // foo.bar.baz = 42
                            let operator = input.consume(T::Eq)?;
                            let rhs = Expr::parse(input).map(Box::new)?;
                            Expr::MemberAssign(MemberAssign {
                                path,
                                delim,
                                name,
                                operator,
                                rhs,
                            })
                        }
                        _ => {
                            // Previous expression is a type that is not
                            // supported as the LHS of an assignment.
                            return Err(syntax_err("lhs of assignment must be identifier or member path"));
                        }
                    }
                }
                Some(T::LeftBracket) => todo!("parse subscript"),
                Some(T::LeftParen) => {
                    println!("Expr::parse_name(_, _) - parse call");
                    input.consume(T::LeftParen)?;
                    input.ignore_many(TokenKind::Whitespace);
                    let args = Expr::parse_call_arguments(input)?;
                    let callee = Box::new(expr);
                    input.ignore_many(TokenKind::Whitespace);
                    input.consume(T::RightParen)?;
                    Expr::Call(Call { callee, args })
                }
                Some(T::Dot) => {
                    let delim = input.consume(T::Dot)?;
                    let lhs = match expr {
                        Expr::NameAccess(NameAccess { ident }) => MemberPath::Name(ident),
                        Expr::MemberAccess(member_access) => MemberPath::Path(Box::new(member_access)),
                        _ => return Err(syntax_err("member access not valid")),
                    };
                    let rhs = Ident::parse(input)?;
                    Expr::MemberAccess(MemberAccess {
                        delim,
                        path: lhs,
                        name: rhs,
                    })
                }
                Some(_) | None => {
                    println!("Expr::parse_name(_, _) - end");
                    // End
                    break;
                }
            };
        }

        Ok(expr)
    }

    /// Parse a variable name.
    ///
    /// Depending on what follows the variable's identifier, the bare
    /// name can be part of the following:
    ///
    /// - dot delimited member access
    /// - function call
    // fn parse_bare_name(input: &mut TokenStream, token: Token) -> ParseResult<Expr> {
    //     use TokenKind as T;

    //     println!("Expr::parse_bare_name(_, {:?})", token.kind);
    //     debug_assert_eq!(token.kind, T::Ident, "expected identifier");

    //     input.ignore_many(TokenKind::Whitespace);
    //     input.reset_peek();

    //     // Depending on the next token, pick the type of variable name access.
    //     let expr = match input.peek().map(|t| t.kind) {
    //         Some(T::Eq) => {
    //             // Name followed by equal sign is an assignment in an expression.
    //             todo!("assignment expr");
    //         }
    //         Some(T::Dot) => {
    //             // Name followed by a dot is treated as a member access.
    //             input.next_token(); // consume dot

    //             // Parse rest of possibly chain member access; insert first element later.
    //             //
    //             //     ┌─ Delimited::parse
    //             //     ├─────┐
    //             // foo.bar.baz
    //             // │
    //             // └ insert(0, ...)
    //             //
    //             let mut parts = Delimited::<Expr, MemberSeparator>::parse(input)?;

    //             // Because the bare name and first dot have been consumed,
    //             // they need to be put back.
    //             parts.pairs.insert(
    //                 0,
    //                 Pair {
    //                     item: Expr::NameAccess(NameAccess {
    //                         ident: Ident {
    //                             text: input.token_fragment(&token).into(),
    //                             token,
    //                         },
    //                     }),
    //                     delimiter: Some(MemberSeparator),
    //                 },
    //             );

    //             todo!()
    //         }
    //         Some(T::LeftParen) => todo!("function call"),
    //         Some(T::LeftBrace) => todo!("function block argument"),
    //         Some(_) | None => {
    //             // Simple name with one identifier
    //             Expr::NameAccess(NameAccess {
    //                 ident: Ident {
    //                     text: input.token_fragment(&token).into(),
    //                     token,
    //                 },
    //             })
    //         }
    //     };

    //     Ok(expr)
    // }

    /// Parse call argument list.
    ///
    /// Cannot be parsed using [`Delimited`] because each element
    /// is a whole recursive expression that needs to terminate on
    /// right parentheses.
    fn parse_call_arguments(input: &mut TokenStream) -> ParseResult<Vec<CallArg>> {
        use TokenKind as T;

        println!("Expr::parse_call_arguments(_)");

        let mut args = vec![];

        loop {
            input.ignore_many(T::Whitespace);

            match input.peek().map(|t| t.kind) {
                Some(T::RightParen) | Some(T::EOF) | None => {
                    // Termination condition
                    break;
                }
                Some(T::Comma) => {
                    // Skip separator
                    input.next_token();
                }
                _ => {
                    args.push(CallArg::Simple(Expr::parse(input)?));
                }
            }
        }

        Ok(args)
    }
}

/// Utility methods for unwrapping expression.
impl Expr {
    pub fn expr_call(&self) -> Option<&Call> {
        match self {
            Expr::Call(e) => Some(e),
            _ => None,
        }
    }

    /// Binary operation expression.
    pub fn expr_bin_op(&self) -> Option<&BinaryOp> {
        match self {
            Expr::Binary(e) => Some(e),
            _ => None,
        }
    }

    /// Assignment operation expression.
    pub fn expr_assign(&self) -> Option<&Assign> {
        match self {
            Expr::Assign(e) => Some(e),
            _ => None,
        }
    }

    pub fn expr_name_access(&self) -> Option<&NameAccess> {
        match self {
            Expr::NameAccess(e) => Some(e),
            _ => None,
        }
    }

    pub fn expr_member_access(&self) -> Option<&MemberAccess> {
        match self {
            Expr::MemberAccess(e) => Some(e),
            _ => None,
        }
    }

    pub fn expr_member_assign(&self) -> Option<&MemberAssign> {
        match self {
            Expr::MemberAssign(e) => Some(e),
            _ => None,
        }
    }

    /// Number literal expression.
    pub fn expr_num_lit(&self) -> Option<&NumLit> {
        match self {
            Expr::Num(e) => Some(e),
            _ => None,
        }
    }

    pub fn expr_group(&self) -> Option<&Group> {
        match self {
            Expr::Group(e) => Some(e),
            _ => None,
        }
    }
}

impl MemberPath {
    pub fn name(&self) -> Option<&Ident> {
        match self {
            MemberPath::Name(ident) => Some(ident),
            MemberPath::Path(_) => None,
        }
    }

    pub fn path(&self) -> Option<&MemberAccess> {
        match self {
            MemberPath::Path(member_access) => Some(&*member_access),
            MemberPath::Name(_) => None,
        }
    }
}

impl CallArg {
    pub fn simple(&self) -> Option<&Expr> {
        match self {
            CallArg::Simple(e) => Some(e),
            _ => None,
        }
    }
}
