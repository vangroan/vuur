pub type FnSymbol = u16;
pub type Address = u16;
pub type RelativeAddr = i16;
// TODO: Should Register be renamed to RegisterAddr?
pub type Register = u8;
pub type ConstOffset = u8;
pub type LiteralInt = i16;
pub const RegisterCount: usize = std::u8::MAX as usize;

/// Bytecode operation.
#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
pub enum ByteOp {
    /// Only advances program counter.
    Noop,

    /// Unconditional jump
    Jmp         { addr: RelativeAddr },

    // ------------------------------------------------------------------------
    // Arithmetic
    Add_I32     { dest: Register, a: Register, b: Register },
    Sub_I32     { dest: Register, a: Register, b: Register },
    Mul_I32     { dest: Register, a: Register, b: Register },
    Eq_I32      { dest: Register, a: Register, b: Register },
    LoadConst   { dest: Register, konst: ConstOffset },
    
    // ------------------------------------------------------------------------
    // Functions
    Call        { func: FnSymbol },
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
pub enum Instruction {
    Noop = 0,

    // ------------------------------------------------------------------------
    // Arithmetic
    Add_I32 = 0x0A,
    Sub_I32,
    Mul_I32,
    Eq_I32,
    PushConst = 0x0E,
    PushConst_Imm = 0x0F,

    // ------------------------------------------------------------------------
    // Callables
    Return = 0x20,
}

#[rustfmt::skip]
pub mod instruction {
    pub const NOOP: u8 = 0;

    pub const ADD_I32: u8 = 0x0A;
    pub const SUB_I32: u8 = 0x0B;
    pub const MUL_I32: u8 = 0x0C;
    pub const EQ_I32:  u8 = 0x0D;

    pub const PUSH_CONST:     u8 = 0x0E;
    pub const PUSH_CONST_IMM: u8 = 0x0F;

    pub const RETURN: u8 = 0x20;
    pub const ABORT:  u8 = 0xFF;
}

impl TryFrom<u8> for Instruction {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use instruction::*;
        match value {
            NOOP => Ok(Self::Noop),
            ADD_I32 => Ok(Self::Add_I32),
            SUB_I32 => Ok(Self::Sub_I32),
            EQ_I32 => Ok(Self::Eq_I32),
            PUSH_CONST => Ok(Self::PushConst),
            PUSH_CONST_IMM => Ok(Self::PushConst_Imm),
            RETURN => Ok(Self::Return),
            _ => Err(format!("unknown bytecode 0x{:02X}", value)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Bytecode instruction must be 32-bits.
    #[test]
    fn test_instruction_size() {
        assert_eq!(std::mem::size_of::<ByteOp>(), 4);
    }

    #[test]
    fn test_u8_conversion() {
        assert_eq!(
            Instruction::Noop,
            Instruction::try_from(Instruction::Noop as u8).unwrap()
        );

        let opcodes = [
            Instruction::Noop,
            // -------------------------
            Instruction::Add_I32,
            Instruction::Sub_I32,
            Instruction::Eq_I32,
            Instruction::PushConst,
            Instruction::PushConst_Imm,
            // -------------------------
            Instruction::Return,
        ];

        for opcode in opcodes {
            assert_eq!(opcode, Instruction::try_from(opcode as u8).unwrap());
        }
    }
}
