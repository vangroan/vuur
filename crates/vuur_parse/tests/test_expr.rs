use vuur_lexer::{Lexer, TokenKind};
use vuur_parse::{
    expr::{BinaryOp, Expr},
    stream::TokenStream,
    Parse,
};

/// Test parentheses groups.
#[test]
fn test_parentheses_group() {
    let source = "(1 - 2) * (3 + 4)";
    let lexer = Lexer::from_source(source);
    let mut stream = TokenStream::new(lexer);
    let expr = Expr::parse(&mut stream).unwrap();
    println!("{:#?}", expr);

    let mul_op = expr.expr_bin_op().expect("binary op");
    assert_eq!(mul_op.operator.fragment(source), "*");
    assert_eq!(mul_op.operator.kind, TokenKind::Mul);

    {
        let left_op = mul_op.lhs.expr_group().expect("group").expr.expr_bin_op().expect("binary op");
        assert_eq!(left_op.operator.fragment(source), "-");
        assert_eq!(left_op.operator.kind, TokenKind::Sub);
        assert_eq!(
            left_op.lhs.expr_num_lit().expect("number literal").token.fragment(source),
            "1"
        );
        assert_eq!(
            left_op.rhs.expr_num_lit().expect("number literal").token.fragment(source),
            "2"
        );
    }

    {
        let right_op = mul_op.rhs.expr_group().expect("group").expr.expr_bin_op().expect("binary op");
        assert_eq!(right_op.operator.fragment(source), "+");
        assert_eq!(right_op.operator.kind, TokenKind::Add);
        assert_eq!(
            right_op.lhs.expr_num_lit().expect("number literal").token.fragment(source),
            "3"
        );
        assert_eq!(
            right_op.rhs.expr_num_lit().expect("number literal").token.fragment(source),
            "4"
        );
    }
}

/// Function call with zero agruments
#[test]
fn test_empty_call_args() {
    let lexer = Lexer::from_source("foobar()");
    let mut stream = TokenStream::new(lexer);
    let expr = Expr::parse(&mut stream).unwrap();
    println!("{:#?}", expr);
    assert_eq!(
        expr.expr_call().unwrap().callee.expr_name_access().unwrap().ident.text,
        "foobar"
    );
    assert!(expr.expr_call().unwrap().args.is_empty());
}

/// Test simple function call with one simple argument.
#[test]
fn test_simple_call_one_arg() {
    let lexer = Lexer::from_source("foobar(a)");
    let mut stream = TokenStream::new(lexer);
    let expr = Expr::parse(&mut stream).unwrap();
    println!("{:#?}", expr);
    assert_eq!(
        expr.expr_call().unwrap().callee.expr_name_access().unwrap().ident.text,
        "foobar"
    );
    assert_eq!(expr.expr_call().unwrap().args.len(), 1);
    assert_eq!(
        expr.expr_call().unwrap().args[0]
            .simple()
            .expect("simple call arg")
            .expr_name_access()
            .unwrap()
            .ident
            .text,
        "a"
    );
}

/// Test simple function call with multiple simple arguments.
#[test]
fn test_simple_call_multi_arg() {
    let lexer = Lexer::from_source("foobar ( a, b, c )");
    let mut stream = TokenStream::new(lexer);
    let expr = Expr::parse(&mut stream).unwrap();
    println!("{:#?}", expr);
    assert_eq!(
        expr.expr_call().unwrap().callee.expr_name_access().unwrap().ident.text,
        "foobar"
    );
    assert_eq!(expr.expr_call().unwrap().args.len(), 3);
    for (idx, name) in ["a", "b", "c"].into_iter().enumerate() {
        assert_eq!(
            expr.expr_call().unwrap().args[idx]
                .simple()
                .expect("simple call arg")
                .expr_name_access()
                .unwrap()
                .ident
                .text,
            name
        );
    }
}

/// Test arithmetic in function call.
#[test]
fn test_simple_call_arithm_arg() {
    let source = "foobar ( 1 + 2, b, 2 - 4 * 5 )";
    let lexer = Lexer::from_source(source);
    let mut stream = TokenStream::new(lexer);
    let expr = Expr::parse(&mut stream).unwrap();
    println!("{:#?}", expr);
    assert_eq!(
        expr.expr_call()
            .expect("call")
            .callee
            .expr_name_access()
            .expect("name access")
            .ident
            .text,
        "foobar"
    );

    // arg 1: "1 + 2"
    {
        let BinaryOp { operator, lhs, rhs } = &expr.expr_call().unwrap().args[0]
            .simple()
            .expect("simple call arg")
            .expr_bin_op()
            .expect("binary operation");

        assert_eq!(operator.fragment(source), "+");
        assert_eq!(lhs.expr_num_lit().expect("number literal").token.fragment(source), "1");
        assert_eq!(rhs.expr_num_lit().expect("number literal").token.fragment(source), "2");
    }

    // arg 2: "b"
    assert_eq!(
        expr.expr_call().unwrap().args[1]
            .simple()
            .expect("simple call arg")
            .expr_name_access()
            .expect("name access")
            .ident
            .text,
        "b"
    );

    // arg 3: "2 - 4 * 5"
    {
        // "2 - ..."
        let BinaryOp { operator, lhs, rhs } = &expr.expr_call().unwrap().args[2]
            .simple()
            .expect("simple call arg")
            .expr_bin_op()
            .unwrap();

        assert_eq!(operator.fragment(source), "-");
        assert_eq!(lhs.expr_num_lit().unwrap().token.fragment(source), "2");

        // "... 4 * 5"
        let BinaryOp { operator, lhs, rhs } = rhs.expr_bin_op().unwrap();

        assert_eq!(operator.fragment(source), "*");
        assert_eq!(lhs.expr_num_lit().unwrap().token.fragment(source), "4");
        assert_eq!(rhs.expr_num_lit().unwrap().token.fragment(source), "5");
    }
}

/// Test field access
#[test]
fn test_field_access() {
    let source = "foo.bar.baz";
    let lexer = Lexer::from_source(source);
    let mut stream = TokenStream::new(lexer);
    let expr = Expr::parse(&mut stream).expect("expr parse");
    println!("{:#?}", expr);

    let access2 = expr.expr_member_access().expect("member access");
    assert_eq!(access2.delim.fragment(source), ".");
    assert_eq!(access2.delim.kind, TokenKind::Dot);
    assert_eq!(access2.name.text, "baz");

    let access1 = access2.path.path().expect("member access");
    assert_eq!(access1.delim.fragment(source), ".");
    assert_eq!(access1.delim.kind, TokenKind::Dot);
    assert_eq!(access1.path.name().expect("name").text, "foo");
    assert_eq!(access1.name.text, "bar");
}

/// Simple assignment expression.
#[test]
fn test_assign() {
    let source = "foobar  =  42";
    let lexer = Lexer::from_source(source);
    let mut stream = TokenStream::new(lexer);
    let expr = Expr::parse(&mut stream).expect("expr parse");
    println!("{:#?}", expr);

    let assign = expr.expr_assign().expect("assign");
    assert_eq!(assign.operator.fragment(source), "=");
    assert_eq!(assign.operator.kind, TokenKind::Eq);
    assert_eq!(assign.lhs.text, "foobar");
    assert_eq!(
        assign.rhs.expr_num_lit().expect("number literal").token.fragment(source),
        "42"
    );
}

/// Test assignment to a setter member, where the owner is a simple name.
#[test]
fn test_field_setter_name() {
    let source = "foo.bar = 42";
    let lexer = Lexer::from_source(source);
    let mut stream = TokenStream::new(lexer);
    let expr = Expr::parse(&mut stream).expect("expr parse");
    println!("{:#?}", expr);

    let assign = expr.expr_member_assign().expect("member assign");
    assert_eq!(assign.path.name().expect("name").text, "foo");
    assert_eq!(assign.delim.fragment(source), ".");
    assert_eq!(assign.delim.kind, TokenKind::Dot);
    assert_eq!(assign.name.text, "bar");
    assert_eq!(assign.operator.fragment(source), "=");
    assert_eq!(assign.operator.kind, TokenKind::Eq);
    assert_eq!(
        assign.rhs.expr_num_lit().expect("number literal").token.fragment(source),
        "42"
    );
}

/// Test assignment to a setter member, where the owner is another member access.
#[test]
fn test_field_setter_path() {
    let source = "foo.bar.baz = 128";
    let lexer = Lexer::from_source(source);
    let mut stream = TokenStream::new(lexer);
    let expr = Expr::parse(&mut stream).expect("expr parse");
    println!("{:#?}", expr);

    let assign = expr.expr_member_assign().expect("member assign");
    let path = assign.path.path().expect("member access");
    assert_eq!(path.delim.fragment(source), ".");
    assert_eq!(path.delim.kind, TokenKind::Dot);
    assert_eq!(path.path.name().expect("name").text, "foo");
    assert_eq!(path.name.text, "bar");

    assert_eq!(assign.delim.fragment(source), ".");
    assert_eq!(assign.delim.kind, TokenKind::Dot);
    assert_eq!(assign.name.text, "baz");
    assert_eq!(assign.operator.fragment(source), "=");
    assert_eq!(assign.operator.kind, TokenKind::Eq);
    assert_eq!(
        assign.rhs.expr_num_lit().expect("number literal").token.fragment(source),
        "128"
    );
}
