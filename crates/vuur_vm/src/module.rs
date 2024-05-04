use crate::func_def::{NativeFunc, ScriptFunc};
use std::rc::Rc;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub func_defs: Vec<Rc<Func>>,
    /// Module level global variables.
    pub vars: Vec<()>,
}

#[derive(Debug)]
pub enum Func {
    Script(ScriptFunc),
    Native(NativeFunc),
}
