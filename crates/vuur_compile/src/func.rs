use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FuncId(pub(crate) NonZeroU32);

impl FuncId {
    #[inline(always)]
    pub(crate) fn new(id: u32) -> Option<Self> {
        NonZeroU32::new(id).map(Self)
    }

    pub fn to_usize(self) -> usize {
        self.0.get() as usize
    }

    pub fn to_u32(self) -> u32 {
        self.0.get() as u32
    }
}

/// Function definition.
pub struct FuncDef {
    pub id: Option<FuncId>,
    /// Start and end position of function's bytecode.
    pub bytecode_span: (u32, u32),
    /// Number of stack slots required for the function's
    /// local variables, excluding call arguments.
    pub local_count: usize,
    /// Number of operand stack slots required for this
    /// function's arguments.
    pub arity: u8,
}
