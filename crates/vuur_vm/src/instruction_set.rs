use std::fmt;
use std::fmt::Formatter;

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
    I32_Cmp,

    /// Push a constant int32 value onto the operand stack.
    I32_Const {
        constant_id: ConstantId,
    },
    I32_Const_Inline {
        arg: Arg24,
    },

    // ------------------------------------------------------------------------
    // Variables
    Store_Local {
        local_id: Arg24,
    },
    Load_Local {
        local_id: Arg24,
    },

    // ------------------------------------------------------------------------
    // Up-values
    /// "Close" the up-value, copying its inner value into its heap slot.
    Upvalue_Close,

    // ------------------------------------------------------------------------
    // Callables
    /// Statically call a function identified by `func_id`.
    Call_Func {
        func_id: Arg24,
    },
    Return,
    /// Create a closure instance.
    Closure_Create,

    // ------------------------------------------------------------------------
    // Control Flow
    /// Unconditionally jump.
    Jump,
    /// Conditionally jump if the top of the operand stack is value 0, type int32.
    Jump_False,
    /// Ends the current block.
    End,
    /// Unconditionally error.
    Abort,
}

pub type ConstantId = u16;

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
