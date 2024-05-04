use crate::handle::Handle;
use std::cell::RefCell;
use std::fmt::{self, Formatter};
use std::rc::{Rc, Weak};

use crate::instruction_set::Op;
use crate::symbol_impl;
use crate::symbol_table::{Symbol, SymbolTable};

symbol_impl!(
    /// Global variable Id.
    #[derive(Debug, Clone, Copy)] pub struct GlobalId(u16)
);

symbol_impl!(
    /// Local variable Id.
    #[derive(Debug, Clone, Copy)] pub struct LocalId(u16)
);

symbol_impl!(
    /// Local variable Id.
    #[derive(Debug, Clone, Copy)] pub struct UpValueId(u16)
);

/// An executable Vuur program.
pub struct Program {
    /// An executable closure object holding the top-level code of the main module.
    pub(crate) closure: Handle<Closure>,

    /// Handle to the module that acted as the function's environment.
    ///
    /// This keeps helps keep the module alive since the closure only
    /// has a weak reference to its lexical module.
    pub(crate) module: Handle<Module>,
}

impl Program {
    pub fn new(module: Handle<Module>, closure: Handle<Closure>) -> Self {
        Self { closure, module }
    }
}

/// Slot is an untyped operand stack value.
///
/// It holds the raw bits of a value. The encoding is
/// specific to the current platform.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct Slot(u64);

impl Slot {
    pub(crate) const ZERO: Self = Slot(0);

    #[inline(always)]
    pub(crate) fn raw(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub(crate) fn from_i32(val: i32) -> Self {
        Self(val as u64)
    }

    #[inline(always)]
    pub(crate) fn to_i32(self) -> i32 {
        self.0 as i32
    }

    #[inline(always)]
    pub(crate) fn from_f32(val: f32) -> Self {
        Self(val.to_bits() as u64)
    }

    #[inline(always)]
    pub(crate) fn to_f32(self) -> f32 {
        f32::from_bits(self.0 as u32)
    }
}

impl fmt::Debug for Slot {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self(value) = *self;
        write!(f, "Slot(0x{value:04x})")
    }
}

#[derive(Debug)]
pub struct Module {
    /// Name of the module.
    pub name: String,
    /// Module level global variables.
    pub vars: SymbolTable<GlobalId, Slot>,
}

impl Module {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            vars: SymbolTable::new(),
        }
    }
}

#[derive(Debug)]
pub enum Func {
    Script(Rc<ScriptFunc>),
    Native(NativeFunc),
}

#[derive(Debug)]
pub struct Closure {
    pub func: Rc<ScriptFunc>,

    /// Up-values are variables that are referenced in this closure's scope,
    /// but are not local to this scope, or global to the module.
    ///
    /// They are boxed into handles because they can be shared between
    /// multiple closures, as well as the call frames that have to close
    /// them on return.
    pub up_values: SymbolTable<UpValueId, Handle<UpValue>>,
}

impl Closure {
    pub fn new(func: Rc<ScriptFunc>) -> Self {
        Self {
            func,
            up_values: SymbolTable::new(),
        }
    }
}

/// An Up-value is a variable that is referenced within a scope, but is not
/// local to that scope.
#[derive(Debug, Clone)]
pub enum UpValue {
    /// A local variable is an **open** up-value when it is still within scope
    /// and on the operand stack.
    ///
    /// In this case the up-value holds an absolute *stack offset* pointing to the
    /// local variable.
    ///
    /// This implies that the stack offset will be invalid when the call frame
    /// is popped from the stack. The up-value must be closed before (the value
    /// copied from the stack to the heap) before the frame returns.
    Open(usize),

    /// A local variable is a **closed** up-value when the closure escapes its
    /// parent scope. The lifetime of those locals extend beyond their scope,
    /// so must be replaced with heap allocated values.
    ///
    /// In this case the up-value holds a *handle* to a heap value.
    Closed(Slot),
}

/// Function defined in the guest script.
///
/// It contains interpreter instructions which can be executed
/// in the virtual machine.
///
/// After compilation a script function is immutable,
/// so it can be stored without `RefCell`.
#[derive(Debug)]
pub struct ScriptFunc {
    pub constants: Vec<u32>,
    pub code: Box<[Op]>,

    /// The function keeps a reference to the module it lexically belongs to.
    ///
    /// This allows instructions in the function to interact with module level
    /// global variables.
    ///
    /// This unfortunately creates circular ownership between modules and functions.
    /// A weak reference is needed to avoid leaking memory, which means when a
    /// function definition leaves the module (like when the host keeps a closure)
    /// the module must be kept alive.
    pub module: Weak<RefCell<Module>>,
}

pub type NativeFuncPtr = fn() -> ();

#[derive(Debug)]
pub struct NativeFunc {
    pub ptr: NativeFuncPtr,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::handle::Handle;
    use std::rc::Rc;

    /// Ensure that a slot can hold a pointer on the current architecture.
    #[test]
    fn test_slot_size() {
        assert!(std::mem::size_of::<*const [u8; 1024]>() <= std::mem::size_of::<Slot>());
        assert!(std::mem::size_of::<Handle<[u8; 1024]>>() <= std::mem::size_of::<Slot>());
        assert!(std::mem::size_of::<Rc<[u8; 1024]>>() <= std::mem::size_of::<Slot>());
    }
}
