use vuur_parse::expr::Expr;
use vuur_parse::pprint::PrettyExpr;
use vuur_parse::stmt::{DefStmt, SimpleStmt};

fn parse_expr(source_code: &str) -> Option<Expr> {
    let mut module = vuur_parse::parse_str(source_code).expect("parsing test module");
    if let Some(DefStmt::Simple(stmt)) = module.stmts.pop() {
        if let SimpleStmt::Expr(expr) = stmt {
            return Some(expr);
        }
    }
    None
}

const CASES: &[&str] = &[
    "1 + 2 * 3",
    "(1 + 2) * 3",
    "1 * 2 * 3",
    "1 + 2 - 3 * 4",
    "1 + (2 - 3) * 4 / 5",
];

#[test]
fn test_expr_pretty_print() {
    for case in CASES {
        let expr = parse_expr(case).unwrap();
        let pprint = PrettyExpr::new(&expr);
        println!("{pprint}");
    }
}
