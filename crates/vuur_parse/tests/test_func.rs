use vuur_parse::parse_str;

#[test]
fn test_func_parse() {
    let source = include_str!("func.vu");
    let module = parse_str(source).unwrap();
    println!("{:#?}", module);
}
