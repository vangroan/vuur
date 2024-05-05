use std::fmt;
use std::fmt::Formatter;

use crate::value::{ConstantId, GlobalId, LocalId, UpValueId};

/// Instruction set.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Op {
    /// Does nothing. The program counter will be incremented.
    NoOp,

    /// Remove the top values from the operand stack and discard it.
    Pop,

    // ------------------------------------------------------------------------
    // Arithmetic
    I32_Add,
    I32_Sub,
    I32_Mul,
    I32_Div,
    I32_Neg,
    I32_Eq,
    I32_Less,
    I32_Greater,
    I32_LessEq,
    I32_GreaterEq,

    /// Push a constant int32 value onto the operand stack.
    I32_Const(ConstantId),
    I32_Const_Inline(Arg24),

    // ------------------------------------------------------------------------
    // Variables
    Store_Global(GlobalId),
    Load_Global(GlobalId),
    Store_Local(LocalId),
    Load_Local(LocalId),
    Store_Upvalue(UpValueId),
    Load_Upvalue(UpValueId),
    /// "Close" the up-value, copying its inner value into its heap slot.
    Upvalue_Close(UpValueId),

    // ------------------------------------------------------------------------
    // Callables
    /// Call a closure instance on the stack.
    Call_Closure {
        arity: u8,
    },
    /// Call a method defined on a class.
    Call_Method {
        arity: u8,
        func_id: u16,
    },
    Return,

    /// Create a closure instance from the function definition stored
    /// in the constant table of the current call frame.
    Closure(ConstantId),

    // ------------------------------------------------------------------------
    // Control Flow
    /// Unconditionally jump.
    Jump,
    /// Conditionally jump if the top of the operand stack is value 0, type int32.
    ///
    /// Pop 1.
    Jump_False {
        addr: Arg24,
    },
    /// Ends the current block.
    End,
    /// Unconditional error.
    Abort,
}

impl Op {
    /// The effect on the operand stack that the instruction has.
    pub fn stack_effect(&self) -> isize {
        match self {
            Op::NoOp => 0,
            Op::Pop => -1,
            Op::I32_Add => -1,
            Op::I32_Sub => -1,
            Op::I32_Mul => -1,
            Op::I32_Div => -1,
            Op::I32_Neg => 0,
            Op::I32_Eq => -1,
            Op::I32_Less => -1,
            Op::I32_Greater => -1,
            Op::I32_LessEq => -1,
            Op::I32_GreaterEq => -1,
            Op::I32_Const(_) => 1,
            Op::I32_Const_Inline(_) => 1,
            Op::Store_Global(_) => 0,
            Op::Load_Global(_) => 1,
            Op::Store_Local(_) => 0,
            Op::Load_Local(_) => 1,
            Op::Store_Upvalue(_) => 0,
            Op::Load_Upvalue(_) => 1,
            Op::Upvalue_Close(_) => 0,
            Op::Call_Closure { arity } => -(*arity as isize) + 1,
            Op::Call_Method { arity, .. } => -(*arity as isize), // remember receiver
            Op::Return => -1,
            Op::Closure(_) => 1,
            Op::Jump => 0,
            Op::Jump_False { .. } => -1,
            Op::End => 0,
            Op::Abort => 0,
        }
    }
}

/// Bytecode argument packed into 24 bits, encoded in little-endian.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Arg24([u8; 3]);

impl Arg24 {
    #[inline(always)]
    pub fn from_i32(value: i32) -> Self {
        // Shift left so sign will be preserved later when decoding.
        let [_, a, b, c] = (value << 8).to_le_bytes();
        Self([a, b, c])
    }

    #[inline(always)]
    pub fn to_i32(self) -> i32 {
        let [a, b, c] = self.0;
        // Shift right to extend to cover up the least-significant bit,
        // and preserve the sign.
        i32::from_le_bytes([0, a, b, c]) >> 8
    }

    #[inline(always)]
    pub fn from_u32(value: u32) -> Self {
        let [a, b, c, _] = value.to_le_bytes();
        Self([a, b, c])
    }

    #[inline(always)]
    pub fn to_u32(self) -> u32 {
        let [a, b, c] = self.0;
        u32::from_le_bytes([a, b, c, 0])
    }
}

impl fmt::Debug for Arg24 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:06x}", self.to_u32())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_size() {
        assert!(
            std::mem::size_of::<Op>() <= 4,
            "bytecode instruction must be at most 32-bits (4 bytes)"
        )
    }

    #[test]
    fn test_arg24() {
        assert_eq!(Arg24::from_i32(0b00000100_00000010_00000001), Arg24([1, 2, 4]));
        assert_eq!(Arg24::from_i32(-1).to_i32(), -1, "negative values must be preserved");
    }
}
