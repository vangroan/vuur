//! Virtual Machine V2.
//!
//!
//! Complete rewrite of the virtual machine.
use std::collections::HashMap;
use std::rc::Rc;

use crate::handle::Handle;
use crate::instruction_set::Op;
use crate::symbol_table::Symbol;
use crate::value::{Closure, Module, Program, Slot, Value};

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
    /// Global table of method signatures.
    ///
    /// The symbol from this table can be used to index into a class'
    /// methods. This is the method-overloading mechanism.
    methods: Vec<()>,
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
    /// Offset into operand stack where this frame's local variables start.
    stack_offset: usize,
    /// Reference to the closure instance that is being executed.
    closure: Handle<Closure>,
}

#[derive(Debug)]
pub struct Fiber {
    /// Operand stack
    pub(crate) stack: Vec<Value>,
    /// Stack of call frames (activation records).
    pub(crate) calls: Vec<CallFrame>,
}

impl Fiber {
    /// Create a fiber with a top-level function as an entrypoint.
    pub fn new(closure: Handle<Closure>) -> Self {
        Self {
            stack: vec![
                // To keep consistent with the calling convention,
                // the called closure must be on the stack when the
                // module's top level code returns.
                Value::Closure(closure.clone()),
            ],
            calls: vec![CallFrame {
                ip: 0,
                // Arguments start after the closure value.
                stack_offset: 1,
                closure,
            }],
        }
    }
}

impl VM {
    pub fn new() -> Self {
        Self {
            fiber: None,
            store: Store {
                modules: HashMap::new(),
                methods: vec![],
            },
        }
    }

    // #[inline(never)]
    pub(crate) fn run_program(&mut self, program: &Program) -> Result<Value, String> {
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
    fn pop_slots_2(&mut self) -> [Value; 2] {
        let l = self.stack.len();
        let value_a = self.stack[l - 2].clone();
        let value_b = self.stack[l - 1].clone();
        self.stack.truncate(self.stack.len() - 2);
        [value_a, value_b]
    }
}

enum FiberAction {
    /// Return a value.
    Return(Value),
    /// Pause execution of the current fiber and yield control
    /// back to host.
    Yield,
}

enum RunAction {
    /// Successfully return a value.
    Return(Value),
    /// Call a closure.
    Call {
        closure: Handle<Closure>,
        stack_offset: usize,
    },
    /// Fiber control action.
    Fiber(FiberAction),
}

/// Run the current fiber in the VM.
// TODO: Instead of Slot, return a decent value that's usable in the Rust host.
fn run_interpreter(vm: &mut VM, fiber: Handle<Fiber>) -> Result<Value, String> {
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
            RunAction::Return(value) => {
                // Drop callee stack and closure value.
                fiber.stack.truncate(frame.stack_offset - 1);

                match fiber.calls.pop() {
                    // Current frame returned but there are no callers left on the stack.
                    None => {
                        return Ok(FiberAction::Return(value));
                    }
                    Some(parent_frame) => {
                        frame = parent_frame;
                        fiber.stack.push(value);
                    }
                }
            }
            RunAction::Call { closure, stack_offset } => {
                // Setup new call frame.
                let mut new_frame = CallFrame {
                    ip: 0,
                    stack_offset,
                    closure,
                };
                std::mem::swap(&mut frame, &mut new_frame);
                // Put parent frame back onto call stack.
                fiber.calls.push(new_frame);
            }
            RunAction::Fiber(_) => {}
        }
    }
}

#[inline(always)]
fn run_op_loop(_vm: &mut VM, fiber: &mut Fiber, frame: &mut CallFrame) -> Result<RunAction, String> {
    let closure = frame.closure.clone();
    let func = closure.borrow_mut().func.clone();

    // for op in func.code.iter() {
    //     println!("    {op:?}");
    // }

    loop {
        let op = func
            .code
            .get(frame.ip)
            .cloned()
            .ok_or_else(|| "bytecode buffer out of bounds")?;

        if cfg!(feature = "trace_ops") {
            println!("{:04} {op:?}", frame.ip);
        }

        frame.ip += 1;

        match op {
            Op::NoOp => { /* Do nothing. */ }
            Op::Pop => {
                // Discard
                fiber.stack.pop();
            }
            Op::I32_Add => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Value::from_i32(a.into_i32()? + b.into_i32()?));
            }
            Op::I32_Sub => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Value::from_i32(a.into_i32()? - b.into_i32()?));
            }
            Op::I32_Mul => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Value::from_i32(a.into_i32()? * b.into_i32()?));
            }
            Op::I32_Div => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Value::from_i32(a.into_i32()? / b.into_i32()?));
            }
            Op::I32_Neg => {
                let a = fiber.stack.pop().ok_or_else(|| "operand stack is empty")?;
                fiber.stack.push(Value::from_i32(-a.into_i32()?));
            }
            Op::I32_Eq => {
                let [a, b] = fiber.pop_slots_2();
                fiber
                    .stack
                    .push(Value::from_i32(if a.into_i32()? == b.into_i32()? { 1 } else { 0 }));
            }
            Op::I32_Less => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Value::Bool(a.into_i32()? < b.into_i32()?));
            }
            Op::I32_LessEq => {
                let [a, b] = fiber.pop_slots_2();
                fiber.stack.push(Value::Bool(a.into_i32()? <= b.into_i32()?));
            }
            Op::I32_Const_Inline(arg) => {
                let a = arg.to_i32();
                fiber.stack.push(Value::from_i32(a));
            }
            Op::Store_Global(global_id) => {
                let module = func
                    .module
                    .upgrade()
                    .ok_or_else(|| "function lost reference to its lexical module")?;
                let value = fiber.stack.pop().unwrap_or(Value::Nil);
                module.borrow_mut().vars.insert(global_id, value);
            }
            Op::Load_Global(global_id) => {
                let module = func
                    .module
                    .upgrade()
                    .ok_or_else(|| "function lost reference to its lexical module")?;
                let value = module.borrow_mut().vars.get(global_id).clone();
                fiber.stack.push(value);
            }
            Op::Store_Local(local_id) => {
                let index = frame.stack_offset + local_id.to_usize();
                if index >= fiber.stack.len() {
                    return Err("operand stack overflow".to_string());
                }
                fiber.stack[index] = fiber.stack.pop().ok_or_else(|| "operand stack underflow")?;
            }
            Op::Load_Local(local_id) => {
                let value = fiber
                    .stack
                    .get(frame.stack_offset + local_id.to_usize())
                    .cloned()
                    .ok_or_else(|| "operand stack overflow")?;
                fiber.stack.push(value);
            }
            Op::Closure(constant_id) => {
                let func_value = func.constants.get(constant_id.to_usize()).cloned().unwrap_or(Value::Nil);
                let func_def = func_value.into_func()?;
                let closure = Handle::new(Closure::new(func_def));
                fiber.stack.push(Value::Closure(closure));
            }
            Op::Call_Closure { arity } => {
                let lo = fiber.stack.len() - arity as usize;
                let closure_offset = lo - 1;
                let closure_value = fiber.stack.get(closure_offset).cloned().ok_or_else(|| "stack underflow")?;
                let closure = closure_value.into_closure()?;
                return Ok(RunAction::Call {
                    closure,
                    stack_offset: lo,
                });
            }
            Op::Return => {
                return Ok(RunAction::Return(fiber.stack.pop().unwrap_or(Value::Nil)));
            }
            Op::Jump_False { addr } => {
                let value = fiber.stack.pop().unwrap_or(Value::Nil);
                if matches!(value, Value::Bool(false)) {
                    frame.ip = addr.to_u32() as usize;
                }
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
