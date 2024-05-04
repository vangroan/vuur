//! Symbol table.
//!
//! ```
//! # use vuur_vm::symbol_table::{SymbolTable, Symbol};
//! # struct Func{}
//!
//! struct FuncId(u16);
//!
//! impl Symbol for FuncId {
//!     const MAX: usize = u16::MAX as usize;
//!
//!     fn from_usize(index: usize) -> Self {
//!         Self(index as u16)
//!     }
//!
//!     fn to_usize(&self) -> usize {
//!         self.0 as usize
//!     }
//!
//! }
//!
//! let mut table = SymbolTable::<FuncId, Func>::new();
//!
//! let func_id = table.push(Func{});
//! ```
use std::marker::PhantomData;

/// Symbol table.
pub struct SymbolTable<K, V> {
    symbols: Vec<V>,
    _key: PhantomData<K>,
}

pub trait Symbol {
    /// The maximum symbol index allowed.
    ///
    /// This is to limit the number of symbols to
    /// what the key can store, given its own restraints.
    ///
    /// For example if the symbol is backed by an `u16`, then
    /// the maximum 16-bit unsigned integer is the
    /// largest the table can grow.
    ///
    /// See module documents [`crate::symbol_table`]
    const MAX: usize;

    /// Create a symbol from an index.
    fn from_usize(index: usize) -> Self;

    /// Determine a table index.
    fn to_usize(&self) -> usize;
}

/// Convenience macro for implementing a symbol key,
/// if the key storage is a simple integer type that
/// can be cast to and from `usize`.
#[macro_export]
macro_rules! symbol_impl {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident($ty:tt)
    ) => {
        $(#[$outer])*
        #[repr(transparent)]
        $vis struct $name($ty);

        impl $name {
            #[inline]
            $vis fn new(index: usize) -> Self {
                assert!(index <= Self::MAX, "index overflows maximum value of {}", stringify!($ty));
                Self::from_usize(index)
            }

            #[inline]
            $vis fn inner(&self) -> &$ty {
                &self.0
            }
        }

        impl $crate::symbol_table::Symbol for $name {
            const MAX: usize = $ty::MAX as usize;

            #[inline(always)]
            fn from_usize(index: usize) -> Self {
                Self(index as $ty)
            }

            #[inline(always)]
            fn to_usize(&self) -> usize {
                self.0 as usize
            }
        }
    };
}

impl<K, V> SymbolTable<K, V> {
    /// Create a new empty symbol table.
    pub fn new() -> Self {
        Self {
            symbols: vec![],
            _key: PhantomData,
        }
    }
}

impl<K: Symbol, V> SymbolTable<K, V> {
    /// Push the given value to the end of the table.
    ///
    /// Returns the symbol identifying the new location.
    pub fn push(&mut self, value: V) -> K {
        if self.symbols.len() + 1 > K::MAX {
            panic!("symbol table overflowed maximum key space: {}", K::MAX);
        }
        let symbol = K::from_usize(self.symbols.len());
        self.symbols.push(value);
        symbol
    }

    /// Insert the given value at the location identified
    /// by the given symbol.
    ///
    /// If a value already exists at the given location,
    /// it is returned.
    ///
    /// # Panic
    ///
    /// Panics if the symbol overflows the table space.
    pub fn insert(&mut self, symbol: K, value: V) -> Option<V> {
        let index = symbol.to_usize();
        if index >= self.symbols.len() {
            panic!("symbol is out of range of table");
        }
        let existing = std::mem::replace(&mut self.symbols[index], value);
        Some(existing)
    }

    /// Retrieve the value identified by the given symbol.
    ///
    /// # Panic
    ///
    /// Panics if the symbol overflows the table space.
    pub fn get(&self, symbol: K) -> &V {
        let index = symbol.to_usize();
        if index >= self.symbols.len() {
            panic!("symbol is out of range of table");
        }
        &self.symbols[index]
    }

    /// Mutably retrieve the value identified by the given symbol.
    ///
    /// # Panic
    ///
    /// Panics if the symbol overflows the table space.
    pub fn get_mut(&mut self, symbol: K) -> &mut V {
        let index = symbol.to_usize();
        if index >= self.symbols.len() {
            panic!("symbol is out of range of table");
        }
        &mut self.symbols[index]
    }
}

impl<K, V> Default for SymbolTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_symbol_macro() {
        {
            symbol_impl!(
                #[derive(Debug, PartialEq, Eq)]
                struct FooId(u8)
            );
            let mut table = SymbolTable::<FooId, ()>::new();
            let symbol1 = table.push(());
            let symbol2 = table.push(());
            let symbol3 = table.push(());
            assert_eq!(symbol1, FooId::new(0));
            assert_eq!(symbol1.to_usize(), 0);
            assert_eq!(symbol2.to_usize(), 1);
            assert_eq!(symbol3.to_usize(), 2);
        }

        {
            symbol_impl!(struct FooId(u16));
            let mut table = SymbolTable::<FooId, ()>::new();
            let symbol1 = table.push(());
            let symbol2 = table.push(());
            let symbol3 = table.push(());
            assert_eq!(symbol1.to_usize(), 0);
            assert_eq!(symbol2.to_usize(), 1);
            assert_eq!(symbol3.to_usize(), 2);
        }

        {
            symbol_impl!(struct FooId(u32));
            let mut table = SymbolTable::<FooId, ()>::new();
            let symbol1 = table.push(());
            let symbol2 = table.push(());
            let symbol3 = table.push(());
            assert_eq!(symbol1.to_usize(), 0);
            assert_eq!(symbol2.to_usize(), 1);
            assert_eq!(symbol3.to_usize(), 2);
        }

        {
            symbol_impl!(struct FooId(i32));
            let mut table = SymbolTable::<FooId, ()>::new();
            let symbol1 = table.push(());
            let symbol2 = table.push(());
            let symbol3 = table.push(());
            assert_eq!(symbol1.to_usize(), 0);
            assert_eq!(symbol2.to_usize(), 1);
            assert_eq!(symbol3.to_usize(), 2);
        }
    }
}
