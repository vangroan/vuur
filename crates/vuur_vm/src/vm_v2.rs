//! Virtual Machine V2.
//!
//!
//! Complete rewrite of the virtual machine.
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

use crate::func_def::{Closure, ScriptFunc};
use crate::handle::Handle;
use crate::instruction_set::Op;
use crate::module::Module;

const ENTRY_POINT: &str = "Main";

#[derive(Debug)]
pub struct VM {
    /// Current running fiber
    pub(crate) fiber: Rc<RefCell<Fiber>>,
    store: Store,
}

#[derive(Debug)]
struct Store {
    modules: HashMap<String, Rc<Module>>,
    /// Global table of function signatures.
    funcs: Vec<()>,
}

#[derive(Debug)]
struct CallFrame {
    /// Instruction pointer
    ip: usize,
    /// Reference to the closure instance that is being executed.
    closure: Handle<Closure>,
}

#[derive(Debug)]
pub struct Fiber {
    /// Operand stack
    pub(crate) stack: Vec<Slot>,
    /// Top frame of the call stack.
    ///
    /// Kept outside the stack buffer to make access infallible.
    pub(crate) frame: CallFrame,
    /// Stack of call frames (activation records).
    pub(crate) calls: Vec<CallFrame>,
}

impl Fiber {
    pub fn new(closure: Handle<Closure>) -> Self {
        Self {
            stack: vec![],
            frame: CallFrame { ip: 0, closure },
            calls: vec![],
        }
    }
}

/// Slot is an untyped operand stack value.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub(crate) struct Slot(u64);

impl Slot {
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

impl VM {
    pub fn new() -> Self {
        todo!()
    }

    /// Runs the entry point of the named module.
    pub fn run_entrypoint(&mut self, module_name: &str) -> Result<(), String> {
        if self.store.modules.get(module_name).is_none() {
            self.store.modules.insert(
                module_name.to_string(),
                Rc::new(Module {
                    name: module_name.to_string(),
                    func_defs: vec![],
                    vars: vec![],
                }),
            );
        }

        let module_rc = self.store.modules[module_name].clone();
        self.run_module(module_rc, ENTRY_POINT, &[])
    }

    /// Execute a top-level function inside the given module.
    pub fn run_module(&mut self, module: Rc<Module>, func_name: &str, args: &[u8]) -> Result<(), String> {
        todo!()
    }

    pub fn resume_fiber(&mut self, fiber: &mut Fiber) -> Result<(), String> {
        todo!()
    }
}

impl Fiber {
    #[inline(always)]
    fn pop_slots_2(&mut self) -> [Slot; 2] {
        let l = self.stack.len();
        let slot_b = self.stack[l - 2];
        let slot_a = self.stack[l - 1];
        self.stack.truncate(self.stack.len() - 2);
        [slot_a, slot_b]
    }
}

enum FiberAction {
    /// Pause execution of the current fiber and yield control
    /// back to host.
    Yield,
}

enum RunAction {
    /// Successfully return a value.
    Return(Slot),
    /// Call a script function.
    Call,
    /// Fiber control action.
    Fiber(FiberAction),
}

/// Run the current fiber in the VM.
fn run_fiber(vm: &mut VM, fiber: &mut Fiber) -> Result<FiberAction, String> {
    todo!()
}

#[inline(always)]
fn run_interpreter(fiber: &mut Fiber, frame: &mut CallFrame) -> Result<RunAction, String> {
    let closure = frame.closure.clone();
    let func = closure.borrow_mut().func.clone();

    'eval: loop {
        let op = func
            .code
            .get(frame.ip)
            .cloned()
            .ok_or_else(|| "bytecode buffer out of bounds")?;
        frame.ip += 1;

        match op {
            Op::NoOp => { /* Do nothing. */ }
            Op::Pop => {
                // Discard
                fiber.stack.pop();
            }
            Op::I32_Add => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Slot::from_i32(a.to_i32() + b.to_i32()));
            }
            Op::I32_Sub => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Slot::from_i32(a.to_i32() - b.to_i32()));
            }
            Op::I32_Mul => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Slot::from_i32(a.to_i32() * b.to_i32()));
            }
            Op::I32_Div => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Slot::from_i32(a.to_i32() / b.to_i32()));
            }
            Op::I32_Neg => {
                let a = fiber.stack.pop().ok_or_else(|| "operand stack is empty")?;
                fiber.stack.push(Slot::from_i32(-a.to_i32()));
            }
            Op::I32_Eq => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Slot::from_i32(if a.to_i32() == b.to_i32() { 1 } else { 0 }));
            }
            Op::I32_Cmp => {
                let [a, b] = fiber.pop_slots_2();
                let ordering = match Ord::cmp(&a.to_i32(), &b.to_i32()) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                fiber.stack.push(Slot::from_i32(ordering));
            }
            Op::I32_Const_Inline { arg } => {
                let a = arg.to_i32();
                fiber.stack.push(Slot::from_i32(a));
            }
            _ => {
                return Err("abort".to_string());
            }
        }
    }
}
