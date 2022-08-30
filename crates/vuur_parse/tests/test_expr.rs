use vuur_lexer::Lexer;
use vuur_parse::{
    expr::{BinaryOp, Expr},
    stream::TokenStream,
    Parse,
};

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
    todo!("evaluate lhs first")
}
