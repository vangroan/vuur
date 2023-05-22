//! Tests for arithmetic correctness.
use vuur_compile::bytecode::{encode_a, encode_simple, opcodes::*};
use vuur_compile::Chunk;
use vuur_parse::expr::Expr;

type PROGRAM<'a> = &'a [u32];
type RESULT = Option<u32>;

const TEST_PROGRAM: &str = r#"
func Main() -> int {
    return 0
}
"#;

fn create_program(bytecode: &[u32]) -> Chunk {
    let bytecode_expr = Expr::Bytecode(bytecode.iter().copied().collect());

    let mut module = vuur_parse::parse_str(TEST_PROGRAM).expect("parsing test program");

    // patch the value of the return with the raw bytecode
    let return_expr = module.stmts[0].func_mut().unwrap().body.stmts[0].return1_mut().unwrap();
    *return_expr = bytecode_expr;

    vuur_compile::compile(&module).expect("failed to compile test program")
}

#[test]
fn test_arithmetic() {
    let cases: &[(PROGRAM, RESULT)] = &[
        (
            &[
                // 1 + 2 * 3
                encode_a(PUSH_CONST_IMM, 1),
                encode_a(PUSH_CONST_IMM, 2),
                encode_a(PUSH_CONST_IMM, 3),
                encode_simple(MUL_I32),
                encode_simple(ADD_I32),
            ],
            Some(7),
        ),
        (
            &[
                // (1 + 2) * 3
                encode_a(PUSH_CONST_IMM, 1),
                encode_a(PUSH_CONST_IMM, 2),
                encode_simple(ADD_I32),
                encode_a(PUSH_CONST_IMM, 3),
                encode_simple(MUL_I32),
            ],
            Some(9),
        ),
        (
            &[
                // -4 + 6
                encode_a(PUSH_CONST_IMM, 4),
                encode_simple(NEG_I32),
                encode_a(PUSH_CONST_IMM, 6),
                encode_simple(ADD_I32),
            ],
            Some(2),
        ),
        (
            &[
                // 6 + (-4)
                encode_a(PUSH_CONST_IMM, 6),
                encode_a(PUSH_CONST_IMM, 4),
                encode_simple(NEG_I32),
                encode_simple(ADD_I32),
            ],
            Some(2),
        ),
        (
            &[
                // 3 * 8 / 4
                encode_a(PUSH_CONST_IMM, 3),
                encode_a(PUSH_CONST_IMM, 8),
                encode_simple(MUL_I32),
                encode_a(PUSH_CONST_IMM, 4),
                encode_simple(DIV_I32),
            ],
            Some(6),
        ),
    ];

    for (index, (code, expected)) in cases.iter().cloned().enumerate() {
        let mut vm = vuur_vm::VM::new();
        // let chunk = Chunk::new(format!("case_{}", index), code.iter().cloned().collect());
        let chunk = create_program(code);
        println!("test arithmetic case-{index}");
        assert_eq!(
            vm.run(&chunk),
            expected,
            "unexpected result from arithmetic case-{index}"
        );
    }
}

// TODO: Retrieve error from VM
#[test]
fn test_arithmetic_error() {
    let cases: &[(PROGRAM, RESULT)] = &[(
        // divide by zero
        &[
            encode_a(PUSH_CONST_IMM, 42),
            encode_a(PUSH_CONST_IMM, 0),
            encode_simple(DIV_I32),
        ],
        None,
    )];

    for (index, (code, expected)) in cases.iter().cloned().enumerate() {
        let mut vm = vuur_vm::VM::new();
        // let chunk = Chunk::new(format!("case_{}", index), code.iter().cloned().collect());
        let chunk = create_program(code);
        println!("test arithmetic case-{index}");
        assert_eq!(vm.run(&chunk), expected);
        let fiber = vm.fiber();
        assert!(fiber.has_error());
        assert_eq!(fiber.error(), Some("divide by zero"));
    }
}
