pub use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

use vuur_compile::bytecode::{decode_arg_a, decode_arg_k, decode_opcode, opcodes as ops};
use vuur_compile::constants::CHUNK_HEADER_RESERVED;
use vuur_compile::Chunk;

pub mod error;
pub mod obj;

use self::error::{ErrorKind, Result, RuntimeError};

pub const STRIDE: usize = 4;
pub const END_OF_CHUNK: usize = std::usize::MAX;

#[derive(Debug)]
pub struct VM {
    /// Current running fiber
    pub(crate) fiber: Rc<RefCell<Fiber>>,
}

#[derive(Debug)]
pub struct Fiber {
    /// Instruction pointer
    pub(crate) ip: usize,
    /// Operand stack
    pub(crate) stack: Vec<u32>,
    /// Call stack of function return information.
    pub(crate) calls: Vec<FrameInfo>,
    /// Indicates if the fiber intends to resume execution in the future
    pub(crate) done: bool,
    /// ---------------------------
    pub(crate) error: Option<String>,
}

#[derive(Debug)]
struct FrameInfo {
    /// Offset in the stack where this call frame's
    /// local stack starts.
    base: usize,
    /// Byte offset in chunk to return to when
    /// this stack frame is popped.
    return_addr: usize,
}

impl Default for FrameInfo {
    #[inline]
    fn default() -> Self {
        Self {
            base: 0,
            return_addr: 0,
        }
    }
}

impl VM {
    pub fn new() -> Self {
        Self {
            fiber: Rc::new(RefCell::new(Fiber::new())),
        }
    }

    /// The current fiber that the VM will execute when resumed.
    #[inline]
    pub fn fiber(&self) -> Ref<'_, Fiber> {
        self.fiber.borrow()
    }

    // TODO: Return value from finished fiber
    pub fn run(&mut self, chunk: &Chunk) -> Option<u32> {
        let entrypoint_id = chunk.entrypoint().unwrap();
        let entrypoint = chunk.func_by_id(entrypoint_id.to_u32());
        let entrypoint_addr = entrypoint.map(|f| f.bytecode_span.0).unwrap_or(0) as usize;

        match (*self.fiber).try_borrow_mut() {
            Ok(mut fiber) => {
                fiber.ip = entrypoint_addr;
                fiber.run(chunk);
                if let Some(error) = &fiber.error {
                    println!("runtime error: {}", error);
                    None
                } else if fiber.done {
                    // Fiber is done executing, and cannot be resumed.
                    match fiber.take_return() {
                        Ok(return_value) => Some(return_value),
                        Err(RuntimeError {
                            kind: ErrorKind::Nil, ..
                        }) => None,
                        Err(err) => {
                            eprintln!("failed getting return value from fiber: {}", err);
                            None
                        }
                    }
                } else {
                    // TODO: Communicate to caller that fiber is paused
                    None
                }
            }
            Err(err) => {
                eprintln!("fiber already borrowed: {}", err);
                None
            }
        }
    }

    pub fn resume(&mut self) {
        todo!("resume paused fiber")
    }
}

impl Fiber {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::with_capacity(1024),
            // FIXME: For now the top level module function directs the interpreter to the starting byte to abort.
            calls: vec![FrameInfo {
                base: 0,
                return_addr: END_OF_CHUNK,
            }],
            done: false,
            error: None,
        }
    }

    // TODO: Support other value types for Fiber return
    pub fn take_return(&mut self) -> Result<u32> {
        if self.done {
            self.stack.last().cloned().ok_or_else(|| RuntimeError::new(ErrorKind::Nil, ""))
        } else {
            Err(RuntimeError::new(
                ErrorKind::FiberState,
                "cannot return value from running fiber",
            ))
        }
    }

    pub fn run(&mut self, chunk: &Chunk) {
        println!("running...");
        'eval: loop {
            if self.ip >= chunk.code().len() {
                println!("end-of-chunk");
                self.complete();
                break 'eval;
            }

            if self.error.is_some() {
                self.complete();
                break 'eval;
            }

            let instruction = chunk.code()[self.ip];

            {
                let [o, a, b, c] = instruction.to_le_bytes();
                self.print_ip();
                print!("{:02X} {:02X} {:02X} {:02X}  ", o, a, b, c);
            }

            let op = decode_opcode(instruction);

            match op {
                ops::NOOP => {
                    println!("");
                    self.ip += 1
                }
                ops::POP => {
                    println!("pop");
                    self.stack.pop();
                    self.ip += 1;
                }
                ops::ADD_I32 => {
                    println!("add.i32");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    let c = a.wrapping_add(b);
                    self.stack.push(c as u32);
                    self.ip += 1;
                    println!("  stack: {:?}", self.stack);
                }
                ops::SUB_I32 => {
                    println!("sub.i32");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    let c = a.wrapping_sub(b);
                    self.stack.push(c as u32);
                    self.ip += 1
                }
                ops::MUL_I32 => {
                    println!("mul.i32");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    let c = a.wrapping_mul(b);
                    self.stack.push(c as u32);
                    self.ip += 1
                }
                ops::DIV_I32 => {
                    println!("div.i32");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    match a.checked_div(b) {
                        Some(c) => {
                            self.stack.push(c as u32);
                            self.ip += 1;
                        }
                        None => self.set_error("divide by zero"),
                    }
                }
                ops::NEG_I32 => {
                    println!("neg.i32");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    self.stack.push(-b as u32);
                    self.ip += 1;
                }
                ops::EQ_I32 => {
                    println!("eq.i32");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    self.stack.push(if a == b { 1 } else { 0 });
                    self.ip += 1;
                }
                ops::PUSH_CONST => {
                    // TODO: constant table
                    let konst_idx = decode_arg_k(instruction);
                    println!(".pushk {}", konst_idx);
                    self.ip += 1
                }
                ops::PUSH_CONST_IMM => {
                    let konst = decode_arg_a(instruction);
                    println!("push.i32.im {}", konst);
                    self.stack.push(konst as u32);
                    self.ip += 1;
                }
                ops::PUSH_LOCAL_I32 => {
                    let local_id = decode_arg_k(instruction);
                    println!("local.i32 {}", local_id);
                    match self.calls.last() {
                        // Because the VM is stack based, the function's local variables
                        // are already on the operand stack.
                        Some(frame) => {
                            let stack_offset = frame.base + local_id as usize;
                            self.stack.push(self.stack[stack_offset]);
                            self.ip += 1;
                        }
                        None => {
                            self.set_error("variable lookup but no frame on call stack");
                        }
                    }
                }
                ops::FUNC => {
                    println!(".function");
                    self.ip += 2; // skip constant table
                }
                ops::SKIP_1 => {
                    println!("skip.i32.1");
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    if a == 1 {
                        self.ip += 2
                    } else {
                        self.ip += 1
                    }
                }
                ops::SKIP_EQ_I32 => {
                    println!("skip.eq.i32");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    if a == b {
                        self.ip += 2
                    } else {
                        self.ip += 1
                    }
                }
                ops::CALL => {
                    let func_id = decode_arg_k(instruction);
                    println!("call {func_id}");
                    self.call_func(chunk, func_id);
                }
                ops::RETURN => {
                    let n = decode_arg_k(instruction);
                    println!("return {n}");
                    match self.calls.pop() {
                        Some(frame) => {
                            // TODO: Multiple return values
                            let result = self.stack.pop().unwrap_or_default();

                            // Truncate the stack that belongs to the current function.
                            self.stack.truncate(frame.base);

                            // Put the result back onto the stack for the caller function.
                            self.stack.push(result);

                            println!("return to 0x{:06X}", frame.return_addr);
                            println!("  base: {}", frame.base);
                            println!("  result: {result}");
                            println!("  stack: {:?}", self.stack);
                            self.ip = frame.return_addr;
                        }
                        None => {
                            // abort
                            println!(".abort");
                            self.done = true;
                            break 'eval;
                        }
                    }
                }
                ops::JUMP => {
                    let addr = decode_arg_k(instruction);
                    println!("jump 0x{:X}", addr * 4);
                    self.ip = addr as usize;
                }
                ops::ABORT => {
                    println!("abort");
                    self.done = true;
                    break 'eval;
                }
                _ => {
                    println!("abort");
                    self.done = true;
                    break 'eval;
                }
            }
        }
    }

    #[inline(always)]
    fn call_func(&mut self, chunk: &Chunk, func_id: u32) {
        match chunk.func_by_id(func_id) {
            Some(func) => {
                let mut arg_start = 0;
                match self.stack.len().checked_sub(func.arity as usize) {
                    Some(stack_base) => {
                        arg_start = stack_base;
                        self.calls.push(FrameInfo {
                            base: stack_base,
                            // after this insrtuction
                            return_addr: self.ip + 1,
                        });
                    }
                    None => self.set_error("stack underflow when attempting to set function call base"),
                }

                // jump to function bytecode
                self.ip = func.bytecode_span.0 as usize;
                println!("call {} 0x{:X}", func_id, func.bytecode_span.0);
                println!("  base:  {arg_start}");
                println!("  args:  {:?}", &self.stack[arg_start..]);
                println!("  stack: {:?}", self.stack);
            }
            None => self.set_error("failed to find function for id {func_id}"),
        }
    }

    /// Sets the fiber to an error state, storing the error message
    /// for later retrieval. See [`Self::error()`]
    #[cold]
    fn set_error<S: ToString>(&mut self, message: S) {
        self.error = Some(message.to_string())
    }

    /// Retrieve the fiber's current error, if any.
    pub fn error(&self) -> Option<&str> {
        self.error.as_ref().map(|s| s.as_str())
    }

    /// Checks whether the fiber is in an error state.
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    /// Mark the fiber as completed.
    ///
    /// A cold function call in a branch will mark that branch as unlikely.
    #[cold]
    fn complete(&mut self) {
        self.done = true
    }

    fn print_ip(&self) {
        print!("0x{:08X}  ", self.ip * 4);
    }
}
