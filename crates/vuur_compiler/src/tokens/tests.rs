use std::fmt;

use crate::string;
use crate::Span;

use super::lexer::Lexer;
use super::tokens::{Token, TokenDecoder};

use itertools::{EitherOrBoth::*, Itertools};
use serde::Deserialize;
use serde_yaml as yaml;

const FIXTURES: &str = include_str!("lexer_fixtures.yaml");

struct TokenFormatter<'a>(Option<&'a Token>);

impl<'a> fmt::Display for TokenFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(token) => {
                let Token {
                    kind,
                    span: Span { index, size },
                } = token;
                write!(f, "{kind} {index} {size}")
            }
            None => write!(f, "              "),
        }
    }
}

fn print_diff_line(index: usize, pass: bool, fragment: &str, expected: Option<&Token>, actual: Option<&Token>) {
    let mark = if pass { "pass" } else { "fail" };
    println!(
        "| {index:3} | {mark} | \"{fragment}\" {} | {} |",
        TokenFormatter(expected),
        TokenFormatter(actual)
    );
}

#[derive(Deserialize)]
struct TestCase {
    input: String,
    output: String,
}

#[test]
fn test_lexer() {
    let fixtures: Vec<TestCase> = yaml::from_str(FIXTURES).expect("failed to deserialize lexer fixtures");

    for case in fixtures {
        println!("input: \n{}", case.input);

        let lexer = Lexer::from_source(&case.input);
        let actual_tokens = lexer.into_iter().collect::<Vec<_>>();
        let expected_tokens = TokenDecoder::decode_lines(&case.output).unwrap();

        let zip = expected_tokens.into_iter().zip_longest(actual_tokens.into_iter());
        let mut failed = 0;

        for (index, pair) in zip.enumerate() {
            match pair {
                Both(expected, actual) => {
                    let fragment = string::unescape_string(expected.fragment(&case.input)).unwrap();
                    let are_equal = expected == actual;
                    if !are_equal {
                        failed += 1;
                    }
                    print_diff_line(index, are_equal, fragment.as_str(), Some(&expected), Some(&actual))
                }
                Left(expected) => {
                    let fragment = expected.fragment(&case.input);
                    failed += 1;
                    print_diff_line(index, false, fragment, Some(&expected), None)
                }
                Right(actual) => {
                    failed += 1;
                    print_diff_line(index, false, "", None, Some(&actual))
                }
            }
        }

        println!();

        if failed > 0 {
            panic!("lexer output unexpected tokens");
        }
    }
}
