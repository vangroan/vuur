use vuur_lexer::span::BytePos;
use vuur_lexer::Lexer;
use vuur_parse::delim::Delimited;
use vuur_parse::func::{FuncArg, Separator};
use vuur_parse::stream::TokenStream;
use vuur_parse::{parse_str, Parse};

#[test]
fn test_func_parse() {
    let source = include_str!("func.vu");
    let module = parse_str(source).unwrap();
    println!("{:#?}", module);
}

#[test]
fn test_delimited() {
    let source = "a: i32, b: i32, c: &i32";
    let mut stream = TokenStream::new(Lexer::from_source(source));
    let delimited = Delimited::<FuncArg, Separator>::parse(&mut stream).unwrap();
    println!("{:#?}", delimited);

    let pair1 = &delimited.pairs[0];
    assert_eq!(pair1.item.name.token.offset, BytePos::from_u32(0));
    assert_eq!(pair1.item.ty.token.offset, BytePos::from_u32(3));

    let pair2 = &delimited.pairs[1];
    assert_eq!(pair2.item.name.token.offset, BytePos::from_u32(8));
    assert_eq!(pair2.item.ty.token.offset, BytePos::from_u32(11));

    let pair3 = &delimited.pairs[2];
    assert_eq!(pair3.item.name.token.offset, BytePos::from_u32(16));
    assert_eq!(pair3.item.ty.token.offset, BytePos::from_u32(20));
}
