use std::cell::RefCell;
use std::fmt::{self, Formatter};
use std::rc::{Rc, Weak};

use crate::handle::Handle;
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
    /// Up-value variable Id.
    #[derive(Debug, Clone, Copy)] pub struct UpValueId(u16)
);

symbol_impl!(
    /// Constant Id.
    #[derive(Debug, Clone, Copy)] pub struct ConstantId(u16)
);

symbol_impl!(
    /// Class method Id.
    #[derive(Debug, Clone, Copy)] pub struct MethodId(u32)
);

/// Dynamically typed value.
///
/// This is to simplify the internals of the VM for the short term.
/// In the future the VM will be statically typed.
///
/// See [`Slot`]
#[derive(Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f32),
    Str(Handle<String>),

    // ------------------------------------------------------------------------
    // Reference type objects.
    Func(Rc<ScriptFunc>),
    Closure(Handle<Closure>),
    Native(Handle<NativeFunc>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Value::*;

        // There are plenty of opportunities for circular
        // references, so we don't recurse into complex objects.
        match self {
            Nil => write!(f, "Nil"),
            Bool(v) => f.debug_tuple("Bool").field(&v).finish(),
            Int(v) => f.debug_tuple("Int").field(&v).finish(),
            Float(v) => f.debug_tuple("Float").field(&v).finish(),
            Str(v) => f.debug_tuple("Str").field(&v).finish(),
            Func(_) => write!(f, "Func(...)"),
            Closure(_) => write!(f, "Closure(...)"),
            Native(_) => write!(f, "Native(...)"),
        }
    }
}

impl Value {
    #[inline(always)]
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    #[inline(always)]
    pub fn into_func(self) -> Result<Rc<ScriptFunc>, String> {
        match self {
            Value::Func(func) => Ok(func),
            _ => Err(self.type_error()),
        }
    }

    #[inline(always)]
    pub fn into_closure(self) -> Result<Handle<Closure>, String> {
        match self {
            Value::Closure(closure) => Ok(closure),
            _ => Err(self.type_error()),
        }
    }

    #[inline(always)]
    pub fn into_native(self) -> Result<Handle<NativeFunc>, String> {
        match self {
            Value::Native(native) => Ok(native),
            _ => Err(self.type_error()),
        }
    }

    #[inline(always)]
    pub fn into_i32(self) -> Result<i32, String> {
        match self {
            Value::Int(int) => Ok(int),
            _ => Err(self.type_error()),
        }
    }

    #[inline(always)]
    pub fn from_i32(value: i32) -> Self {
        Self::Int(value)
    }

    fn type_error(&self) -> String {
        format!("unexpected value type: {self:?}")
    }

    pub fn repr(&self) -> ValueRepr {
        ValueRepr(self)
    }
}

pub struct ValueRepr<'a>(&'a Value);

impl<'a> fmt::Display for ValueRepr<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self.0 {
            Value::Nil => write!(f, "nil"),
            Value::Bool(v) => write!(f, "{v}"),
            Value::Int(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v}"),
            Value::Str(ref v) => write!(f, "\"{}\"", v.borrow()),
            Value::Func(_) => write!(f, "function"),
            Value::Closure(_) => write!(f, "closure"),
            Value::Native(_) => write!(f, "native function"),
        }
    }
}

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
///
/// FIXME: Storing reference type object pointers in a slot.
///
/// To keep the VM simple, the standard library `Rc` is used
/// for reference types. It doesn't expose its internal pointer,
/// making it hard to build unsafe internals around it.
///
/// When we have a proper garbage collector with our own
/// handle types we can revisit `Slot`.
#[derive(Clone, Copy)]
#[repr(transparent)]
#[allow(dead_code)]
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

    #[inline(always)]
    pub(crate) fn from_ptr<T>(ptr: *const T) -> Self {
        Self(ptr as usize as u64)
    }

    #[inline(always)]
    pub(crate) unsafe fn to_ptr<T>(self) -> *mut T {
        self.0 as usize as *mut T
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
    pub vars: SymbolTable<GlobalId, Value>,
}

impl Module {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            vars: SymbolTable::new(),
        }
    }

    pub fn dump_vars(&self) {
        println!("{} variables:", self.name);
        for (symbol, var) in self.vars.iter() {
            println!("     {symbol:?} : {var:?}");
        }
    }
}

#[derive(Debug)]
pub enum Method {
    Script(Rc<ScriptFunc>),
    Native(NativeFunc),
}

pub struct Class {
    /// Table of methods belonging to this class.
    ///
    /// This corresponds with the global method signature
    /// table in [`Store`].
    methods: SymbolTable<MethodId, Option<Method>>,
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
    /// Values defined in the function body that do not change.
    pub constants: Vec<Value>,

    /// Interpreter bytecode instructions to be executed.
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

/// Environment passed to native functions to grant
/// the host access to some virtual machine functions.
pub struct Env {}

// TODO: Decent error type for VM API.
pub type NativeFuncPtr = fn(env: Env, args: &[Value]) -> Result<Value, String>;

/// Host function defined in Rust.
#[derive(Debug)]
pub struct NativeFunc {
    pub ptr: NativeFuncPtr,
    pub arity: u8,
    // TODO: Debug info
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::handle::Handle;
    use std::rc::Rc;

    /// Ensure that a slot can hold a pointer on the current architecture.
    #[test]
    fn test_slot_size() {
        println!("{}", std::mem::size_of::<Value>());
        assert!(std::mem::size_of::<*const [u8; 1024]>() <= std::mem::size_of::<Slot>());
        assert!(std::mem::size_of::<Handle<[u8; 1024]>>() <= std::mem::size_of::<Slot>());
        assert!(std::mem::size_of::<Rc<[u8; 1024]>>() <= std::mem::size_of::<Slot>());
    }

    #[test]
    fn test_unsized_boxed() {
        struct Str<T: ?Sized> {
            size: usize,
            data: T,
        }

        let s: Box<Str<[u8]>> = Box::new(Str {
            size: 4,
            data: [1, 2, 3, 4],
        });
    }
}
