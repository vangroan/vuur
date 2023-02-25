//! Interpreter bytecode
//!
//! ```text
//! | type |    24 |    16 |     8 |      0 |
//! |------|-------|-------|-------|--------|
//! | oK   |           K           | opcode |
//! ```
use std::io;

pub type OpCode = u8;

#[rustfmt::skip]
pub mod opcodes {
    use super::OpCode;

    pub const NOOP: OpCode = 0;

    // ------------------------------------------------------------------------
    // Arithmetic
    pub const ADD_I32: OpCode = 0x0A;
    pub const SUB_I32: OpCode = 0x0B;
    pub const MUL_I32: OpCode = 0x0C;
    pub const EQ_I32:  OpCode = 0x0D;

    pub const PUSH_CONST:     OpCode = 0x0E;
    pub const PUSH_CONST_IMM: OpCode = 0x0F;

    // ------------------------------------------------------------------------
    // Callables
    pub const FUNC: OpCode = 0x10;

    // ------------------------------------------------------------------------
    // Control Flow
    pub const RETURN: OpCode = 0x20;
    pub const ABORT:  OpCode = 0xFF;
}

// TODO: Fix bytecode write and use without compiler
pub(crate) trait WriteBytecode {
    fn write_simple(&mut self, op: OpCode) -> io::Result<()>;
    fn write_k(&mut self, op: OpCode, k: i32) -> io::Result<()>;
}

impl WriteBytecode for Vec<u32> {
    #[inline]
    fn write_simple(&mut self, op: OpCode) -> io::Result<()> {
        self.push(u32::from_le_bytes([op, 0, 0, 0]));
        Ok(())
    }

    #[inline]
    fn write_k(&mut self, op: OpCode, k: i32) -> io::Result<()> {
        self.push(u32::from_le_bytes([
            op,
            (k & 0xF) as u8,
            ((k >> 1) & 0xF0) as u8,
            ((k >> 2) & 0xF00) as u8,
        ]));
        Ok(())
    }
}

#[inline]
pub fn decode_opcode(instruction: u32) -> u8 {
    (instruction & 0xFF) as u8
}

/// Decode instruction of type `oK`
///
/// ```
/// # use vuur_compile::bytecode::decode_k;
/// # use vuur_compile::bytecode::opcodes::PUSH_CONST;
/// let (opcode, konst_idx) = decode_k(0x001102_0E);
/// assert_eq!((PUSH_CONST, 4354), (opcode, konst_idx));
/// ```
#[inline]
pub fn decode_k(instruction: u32) -> (u8, u32) {
    (decode_opcode(instruction), decode_arg_k(instruction))
}

#[inline]
pub fn decode_arg_k(instruction: u32) -> u32 {
    (instruction & 0xFFFFFF00) >> 8
}

#[inline]
pub fn decode_arg_a(instruction: u32) -> i32 {
    ((instruction & 0xFFFFFF00) >> 8) as i32
}

/// Encode the given 64-bit integer as two 32-bit integers.
///
/// The resulting encoding is intended to be encoded further
/// as little-endian. The lowest bytes will be in position 0,
/// and the highest bytes in position 1.
///
/// ```
/// # use vuur_compile::bytecode::encode_u64;
/// let [low, high] = encode_u64(0x200000001);
/// assert_eq!(low, 1);
/// assert_eq!(high, 2);
/// ```
#[inline]
pub fn encode_u64(value: u64) -> [u32; 2] {
    [(value & 0xFFFFFFFF) as u32, ((value & 0xFFFFFFFF00000000) >> 32) as u32]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_64() {
        let cases: &[(u64, [u32; 2])] = &[
            (1, [0b1, 0b0]),
            (
                1147797409030816545,
                [0b10000111011001010100001100100001, 0b00001111111011011100101110101001],
            ),
        ];

        for (input, output) in cases {
            assert_eq!(*output, encode_u64(*input));
        }
    }
}
