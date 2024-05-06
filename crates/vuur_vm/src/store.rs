//! Global store.
//!
//! The [`Store`] keeps global state that span across modules,
//! during runtime and the compiler.
use std::collections::HashMap;
use std::fmt::{self, Formatter};
use std::ops;
use std::rc::Rc;

use crate::symbol_table::SymbolTable;
use crate::value::{MethodId, Module};

#[derive(Debug)]
pub struct Store {
    /// Registry of cached compiled modules.
    ///
    /// Key is the canonical name of the module.
    pub(crate) modules: HashMap<String, Rc<Module>>,

    /// Global table of method signatures.
    ///
    /// The symbol from this table can be used to index into a class' methods.
    /// This is the method-overloading mechanism.
    // TODO: MethodSig instead of String
    pub(crate) methods: SymbolTable<MethodId, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            methods: SymbolTable::new(),
        }
    }

    pub fn insert_func(&mut self) {
        todo!("Insert function signature")
    }
}

/// Method signature, which can be used to match calls to methods.
#[derive(Debug)]
pub struct MethodSig {
    pub flags: MethodFlags,
    pub args: Vec<()>,
    pub return_: (),
}

// `fib(Int32) -> Int32`
// `replace(Str,Str) -> Str`
// `static validate(Int32) -> Bool`
// `native static sqrt(Float) -> Float`
#[derive(Clone, Copy)]
pub struct MethodFlags(u32);

impl MethodFlags {
    pub const STATIC: MethodFlags = MethodFlags(0b0001);
    pub const NATIVE: MethodFlags = MethodFlags(0b0010);

    #[inline(always)]
    pub fn is_static(self) -> bool {
        (self.0 & Self::STATIC.0) != 0
    }

    #[inline(always)]
    pub fn is_native(self) -> bool {
        (self.0 & Self::NATIVE.0) != 0
    }
}

impl ops::BitAnd for MethodFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl fmt::Debug for MethodFlags {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "MethodFlags({:04b})", self.0)
    }
}
