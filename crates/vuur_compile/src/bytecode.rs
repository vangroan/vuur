pub type FnSymbol = u16;
pub type Address = u16;
pub type RelativeAddr = i16;
// TODO: Should Register be renamed to RegisterAddr?
pub type Register = u8;
pub type ConstOffset = u8;
pub type LiteralInt = i16;
pub const RegisterCount: usize = std::u8::MAX as usize;

/// Bytecode operation.
// #[derive(Debug, Copy, Clone)]
// #[rustfmt::skip]
// #[allow(non_camel_case_types)]
// #[deprecated]
// pub enum ByteOp {
//     /// Only advances program counter.
//     Noop,

//     /// Unconditional jump
//     Jmp         { addr: RelativeAddr },

//     // ------------------------------------------------------------------------
//     // Arithmetic
//     Add_I32     { dest: Register, a: Register, b: Register },
//     Sub_I32     { dest: Register, a: Register, b: Register },
//     Mul_I32     { dest: Register, a: Register, b: Register },
//     Eq_I32      { dest: Register, a: Register, b: Register },
//     LoadConst   { dest: Register, konst: ConstOffset },
    
//     // ------------------------------------------------------------------------
//     // Functions
//     Call        { func: FnSymbol },
//     Return,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u8)]
// #[rustfmt::skip]
// #[allow(non_camel_case_types)]
// #[deprecated]
// pub enum Instruction {
//     Noop = 0,

//     // ------------------------------------------------------------------------
//     // Arithmetic
//     Add_I32 = 0x0A,
//     Sub_I32,
//     Mul_I32,
//     Eq_I32,
//     PushConst = 0x0E,
//     PushConst_Imm = 0x0F,

//     // ------------------------------------------------------------------------
//     // Callables
//     Return = 0x20,
// }

#[rustfmt::skip]
pub mod opcodes {
    pub type OpCode = u8;

    pub const NOOP: OpCode = 0;

    pub const ADD_I32: OpCode = 0x0A;
    pub const SUB_I32: OpCode = 0x0B;
    pub const MUL_I32: OpCode = 0x0C;
    pub const EQ_I32:  OpCode = 0x0D;

    pub const PUSH_CONST:     OpCode = 0x0E;
    pub const PUSH_CONST_IMM: OpCode = 0x0F;

    pub const FUNC: OpCode = 0x10;

    pub const RETURN: OpCode = 0x20;
    pub const ABORT:  OpCode = 0xFF;
}

pub(crate) trait WriteBytecode {
    fn write_simple(&mut self, op: opcodes::OpCode) -> std::io::Result<()>;
    fn write_k(&mut self, op: opcodes::OpCode, k: i32) -> std::io::Result<()>;
}

impl WriteBytecode for Vec<u32> {
    #[inline]
    fn write_simple(&mut self, op: opcodes::OpCode) -> std::io::Result<()> {
        self.push(u32::from_le_bytes([op, 0, 0, 0]));
        Ok(())
    }

    #[inline]
    fn write_k(&mut self, op: opcodes::OpCode, k: i32) -> std::io::Result<()> {
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
pub fn get_opcode(instruction: u32) -> u8 {
    (instruction & 0xFF) as u8
}

#[inline]
pub fn read_arg_k(instruction: u32) -> u32 {
    (instruction & 0xFFFFFF00) >> 8
}

#[inline]
pub fn read_arg_a(instruction: u32) -> i32 {
    ((instruction & 0xFFFFFF00) >> 8) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    // Bytecode instruction must be 32-bits.
    // #[test]
    // fn test_instruction_size() {
    //     assert_eq!(std::mem::size_of::<ByteOp>(), 4);
    // }
    // #[test]
    // fn test_u8_conversion() {
    //     assert_eq!(
    //         Instruction::Noop,
    //         Instruction::try_from(Instruction::Noop as u8).unwrap()
    //     );

    //     let opcodes = [
    //         Instruction::Noop,
    //         // -------------------------
    //         Instruction::Add_I32,
    //         Instruction::Sub_I32,
    //         Instruction::Eq_I32,
    //         Instruction::PushConst,
    //         Instruction::PushConst_Imm,
    //         // -------------------------
    //         Instruction::Return,
    //     ];

    //     for opcode in opcodes {
    //         assert_eq!(opcode, Instruction::try_from(opcode as u8).unwrap());
    //     }
    // }
}
