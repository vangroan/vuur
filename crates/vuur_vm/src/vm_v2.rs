//! Virtual Machine V2.
//!
//!
//! Complete rewrite of the virtual machine.
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

use crate::handle::Handle;
use crate::instruction_set::Op;
use crate::value::Module;
use crate::value::Slot;
use crate::value::{Closure, Program};

const ENTRY_POINT: &str = "Main";

#[derive(Debug)]
pub struct VM {
    /// Current running fiber
    pub(crate) fiber: Option<Handle<Fiber>>,
    store: Store,
}

#[derive(Debug)]
pub struct Store {
    modules: HashMap<String, Rc<Module>>,
    /// Global table of function signatures.
    funcs: Vec<()>,
}

impl Store {
    pub fn insert_func(&mut self) {
        todo!("Insert function signature")
    }
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
    /// Stack of call frames (activation records).
    pub(crate) calls: Vec<CallFrame>,
}

impl Fiber {
    pub fn new(closure: Handle<Closure>) -> Self {
        Self {
            stack: vec![],
            calls: vec![CallFrame { ip: 0, closure }],
        }
    }
}

impl VM {
    pub fn new() -> Self {
        Self {
            fiber: None,
            store: Store {
                modules: HashMap::new(),
                funcs: vec![],
            },
        }
    }

    pub(crate) fn run_program(&mut self, program: &Program) -> Result<Slot, String> {
        let module = program.module.clone();
        let closure = program.closure.clone();

        // Setup a fiber
        let fiber = Handle::new(Fiber::new(closure));

        let result = run_interpreter(self, fiber)?;

        Ok(result)
    }

    /// Runs the entry point of the named module.
    pub fn run_entrypoint(&mut self, module_name: &str) -> Result<(), String> {
        if self.store.modules.get(module_name).is_none() {
            self.store
                .modules
                .insert(module_name.to_string(), Rc::new(Module::new(module_name)));
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
    /// Return a value.
    Return(Slot),
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
// TODO: Instead of Slot, return a decent value that's usable in the Rust host.
fn run_interpreter(vm: &mut VM, fiber: Handle<Fiber>) -> Result<Slot, String> {
    vm.fiber = Some(fiber.clone());

    loop {
        let fiber = &mut *fiber.borrow_mut();

        match run_fiber(vm, fiber)? {
            FiberAction::Return(slot) => {
                return Ok(slot);
            }
            FiberAction::Yield => {
                todo!()
            }
        }
    }
}

fn run_fiber(vm: &mut VM, fiber: &mut Fiber) -> Result<FiberAction, String> {
    let mut frame = fiber.calls.pop().ok_or_else(|| "fiber has no frames on its callstack")?;

    loop {
        match run_op_loop(vm, fiber, &mut frame)? {
            RunAction::Return(slot) => {
                // Current frame returned but there are no callers left on the stack.
                if fiber.calls.is_empty() {
                    return Ok(FiberAction::Return(slot));
                }
            }
            RunAction::Call => {}
            RunAction::Fiber(_) => {}
        }
    }
}

#[inline(always)]
fn run_op_loop(vm: &mut VM, fiber: &mut Fiber, frame: &mut CallFrame) -> Result<RunAction, String> {
    let closure = frame.closure.clone();
    let func = closure.borrow_mut().func.clone();

    loop {
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
            Op::Return => {
                return Ok(RunAction::Return(fiber.stack.pop().unwrap_or(Slot::ZERO)));
            }
            Op::Abort => {
                return Err("abort".to_string());
            }
            _ => {
                return Err(format!("instruction not implemented yet: {op:?}"));
            }
        }
    }
}
