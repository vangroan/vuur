use vuur_lexer::Lexer;
use vuur_parse::{expr::Expr, stream::TokenStream, Parse};

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
