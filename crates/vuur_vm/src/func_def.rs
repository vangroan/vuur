use crate::instruction_set::Op;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FuncId(pub(crate) u32);

impl FuncId {
    #[inline(always)]
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    #[inline(always)]
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    pub fn to_u32(self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct Closure {
    pub func_id: FuncId,
    pub func: Rc<ScriptFunc>,
    pub up_values: Vec<()>,
}

/// Function definition in the guest script.
///
/// It contains interpreter instructions which can be executed
/// in the virtual machine.
#[derive(Debug)]
pub struct ScriptFunc {
    pub id: FuncId,
    pub constants: Vec<u32>,
    pub code: Box<[Op]>,
}

pub type NativeFuncPtr = fn() -> ();

#[derive(Debug)]
pub struct NativeFunc {
    pub id: FuncId,
    pub ptr: NativeFuncPtr,
}
