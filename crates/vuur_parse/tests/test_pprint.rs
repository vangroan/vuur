use vuur_parse::expr::Expr;
use vuur_parse::pprint::PrettyExpr;
use vuur_parse::stmt::{DefStmt, SimpleStmt};

const FG_GREEN: &str = "\x1b[32m";
const FG_WHITE: &str = "\x1b[37m";

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
    "-1",
    "1 + (-2)",
    "x = 1 + 2",
    "z = x + y * 32",
    "one.two.three",
    "one.two.three = x * y",
];

#[test]
fn test_expr_pretty_print() {
    for case in CASES {
        let expr = parse_expr(case).unwrap();
        let pprint = PrettyExpr::new(&expr);
        println!("{FG_GREEN}{case}{FG_WHITE}");
        println!("{pprint}");
    }
}
