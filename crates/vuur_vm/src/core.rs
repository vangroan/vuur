//! Core module.
use crate::handle::Handle;
use crate::store::Store;
use crate::value::{Env, Module, NativeFunc, NativeFuncPtr, Value, Value::Int};

/// Initialize the built-in types and function of the core module.
pub fn init_core(store: &mut Store, module: &mut Module) {
    bind_method(store, module, "static max(_,_)", int32_max);
    bind_method(store, module, "static print(_)", system_print);
}

fn bind_method(store: &mut Store, module: &mut Module, sig: &str, ptr: NativeFuncPtr) {
    // TODO: Get or insert function signature into store
    // Arity from signature
    let arity = sig.chars().filter(|c| *c == '_').count() as u8;
    // Save native func as global variable
    module.vars.push(Value::Native(Handle::new(NativeFunc { ptr, arity })));
    store.methods.push(sig.to_string());
}

/// Returns the maximum integer.
fn int32_max(_env: Env, args: &[Value]) -> Result<Value, String> {
    match args {
        &[Int(a), Int(b)] => Ok(Int(a.max(b))),
        _ => Err("unexpected types".to_string()),
    }
}

// ----------------------------------------------------------------------------
// System

fn system_print(_env: Env, args: &[Value]) -> Result<Value, String> {
    println!("{}", args[0].repr());
    Ok(Value::Nil)
}
