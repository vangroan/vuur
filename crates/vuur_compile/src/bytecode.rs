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
    pub const DIV_I32: OpCode = 0x0D;
    pub const NEG_I32: OpCode = 0x0E;
    pub const EQ_I32:  OpCode = 0x0F;

    pub const PUSH_CONST:     OpCode = 0x10;
    pub const PUSH_CONST_IMM: OpCode = 0x11;
    pub const PUSH_LOCAL_I32:     OpCode = 0x12;

    // ------------------------------------------------------------------------
    // Callables
    pub const FUNC: OpCode = 0x20;

    pub const SKIP_ONE: OpCode = 0x30;
    pub const SKIP_LT: OpCode = 0x31;
    pub const SKIP_LE: OpCode = 0x32;

    // ------------------------------------------------------------------------
    // Control Flow
    pub const CALL:     OpCode = 0x50; // static call
    pub const DYN_CALL: OpCode = 0x51; // dynamic call
    pub const RETURN:   OpCode = 0x52;
    pub const JUMP:     OpCode = 0x53; // unconditional jump
    pub const ABORT:    OpCode = 0xFF;
}

// TODO: Fix bytecode write and use without compiler
pub(crate) trait WriteBytecode {
    fn write_data(&mut self, data: u32) -> io::Result<()>;
    fn write_simple(&mut self, op: OpCode) -> io::Result<u32>;
    fn write_k(&mut self, op: OpCode, k: u32) -> io::Result<u32>;
    fn write_a(&mut self, op: OpCode, a: i32) -> io::Result<()>;

    fn patch_k(&mut self, addr: u32, op: OpCode, k: u32) -> io::Result<()>;
}

impl WriteBytecode for Vec<u32> {
    #[inline]
    fn write_data(&mut self, data: u32) -> io::Result<()> {
        self.push(data);
        Ok(())
    }

    #[inline]
    fn write_simple(&mut self, op: OpCode) -> io::Result<u32> {
        let addr = self.len() as u32;
        self.push(u32::from_le_bytes([op, 0, 0, 0]));
        Ok(addr)
    }

    #[inline]
    fn write_k(&mut self, op: OpCode, k: u32) -> io::Result<u32> {
        let addr = self.len() as u32;
        self.push(encode_k(op, k));
        Ok(addr)
    }

    #[inline]
    fn write_a(&mut self, op: OpCode, a: i32) -> io::Result<()> {
        self.push(encode_a(op, a));
        Ok(())
    }

    fn patch_k(&mut self, addr: u32, op: OpCode, k: u32) -> io::Result<()> {
        self[addr as usize] = encode_k(op, k);
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
/// let (opcode, konst_idx) = decode_k(0x001102_10);
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

#[inline]
pub fn encode_simple(op: OpCode) -> u32 {
    // opcode is in least significant bit
    op as u32
}

pub fn encode_k(op: OpCode, k: u32) -> u32 {
    debug_assert!(k <= 0xFFFFFF, "argument k must fit in 24 bits");
    (op as u32) | ((k & 0xFFFFFF) << 8)
}

pub fn encode_a(op: OpCode, a: i32) -> u32 {
    debug_assert!(a <= 0x7FFFFF, "argument a must fit in 24 bits");
    debug_assert!(a >= -0x7FFFFF, "argument a must fit in 24 bits");
    (op as u32) | (((a & 0xFFFFFF) as u32) << 8)
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
    #[rustfmt::skip]
    fn test_encode_k() {
        let cases: &[((OpCode, u32), [u8; 4])] = &[
            (
                (opcodes::PUSH_CONST, 0x30201),
                [0x10, 0x01, 0x02, 0x03],
            ),
        ];

        for (input, output) in cases {
            assert_eq!(*output, encode_k(input.0, input.1).to_le_bytes());
        }
    }

    #[test]
    fn test_encode_a() {}

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
