//! Tests for arithmetic correctness.
use vuur_compile::bytecode::{encode_a, encode_simple, opcodes::*};
use vuur_compile::Chunk;

type PROGRAM<'a> = &'a [u32];
type RESULT = Option<u32>;

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
        let chunk = Chunk::new(format!("case_{}", index), code.iter().cloned().collect());
        println!("test arithmetic {}", chunk.name());
        assert_eq!(vm.run(&chunk), expected);
    }
}

// TODO: Retrieve error from VM
#[test]
fn test_arithmetic_error() {
    let cases: &[(PROGRAM, RESULT)] = &[(
        &[
            encode_a(PUSH_CONST_IMM, 42),
            encode_a(PUSH_CONST_IMM, 0),
            encode_simple(DIV_I32),
        ],
        None,
    )];

    for (index, (code, expected)) in cases.iter().cloned().enumerate() {
        let mut vm = vuur_vm::VM::new();
        let chunk = Chunk::new(format!("case_{}", index), code.iter().cloned().collect());
        println!("test arithmetic {}", chunk.name());
        assert_eq!(vm.run(&chunk), expected);
    }
}
