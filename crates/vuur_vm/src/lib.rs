use std::cell::RefCell;
use std::rc::Rc;

use vuur_compile::bytecode::{get_opcode, opcodes as ops, read_arg_a, read_arg_k};
use vuur_compile::constants::CHUNK_HEADER_RESERVED;
use vuur_compile::Chunk;

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
}

#[derive(Debug)]
struct FrameInfo {
    /// Byte offset in chunk to return to when
    /// this stack frame is popped.
    return_addr: usize,
}

impl Default for FrameInfo {
    #[inline]
    fn default() -> Self {
        Self { return_addr: 0 }
    }
}

impl VM {
    pub fn new() -> Self {
        Self {
            fiber: Rc::new(RefCell::new(Fiber::new())),
        }
    }

    pub fn run(&mut self, chunk: &Chunk) {
        match (*self.fiber).try_borrow_mut() {
            Ok(mut fiber) => {
                fiber.ip = 0;
                fiber.run(chunk);
                if fiber.done {
                    // Fiber is done executing, and cannot be resumed.
                    // TODO: Return top stack value
                    println!("fiber is done: {:?}", fiber.stack);
                }
            }
            Err(err) => {
                eprintln!("fiber already borrowed: {}", err);
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
                return_addr: END_OF_CHUNK,
            }],
            done: false,
        }
    }

    pub fn run(&mut self, chunk: &Chunk) {
        'eval: loop {
            if self.ip >= chunk.code().len() {
                println!("end-of-chunk");
                self.done = true;
                break 'eval;
            }
            let instruction = chunk.code()[self.ip];

            {
                let [o, a, b, c] = instruction.to_le_bytes();
                self.print_ip();
                print!("{:02X} {:02X} {:02X} {:02X}  ", o, a, b, c);
            }

            let op = get_opcode(instruction);
            // print!("opcode: {}", op);

            match op {
                ops::NOOP => {
                    println!("");
                    self.ip += 1
                }
                ops::ADD_I32 => {
                    println!(".add");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    let c = a.wrapping_add(b);
                    self.stack.push(c as u32);
                    self.ip += 1
                }
                ops::SUB_I32 => {
                    println!(".sub");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    let c = a.wrapping_sub(b);
                    self.stack.push(c as u32);
                    self.ip += 1
                }
                ops::MUL_I32 => {
                    println!(".mul");
                    let b = self.stack.pop().unwrap_or_default() as i32;
                    let a = self.stack.pop().unwrap_or_default() as i32;
                    let c = a.wrapping_mul(b);
                    self.stack.push(c as u32);
                    self.ip += 1
                }
                ops::PUSH_CONST => {
                    // TODO: constant table
                    let konst_idx = read_arg_k(instruction);
                    println!(".pushk {}", konst_idx);
                    self.ip += 1
                }
                ops::PUSH_CONST_IMM => {
                    let konst = read_arg_a(instruction);
                    println!(".pushi {}", konst);
                    self.stack.push(konst as u32);
                    self.ip += 1;
                }
                ops::FUNC => {
                    println!(".function");
                    self.ip += 2; // skip constant table
                }
                ops::RETURN => {
                    match self.calls.pop() {
                        Some(frame) => {
                            println!(".return to 0x{:06X}", frame.return_addr);
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
                _ => {
                    println!(".abort");
                    self.done = true;
                    break 'eval;
                }
            }
        }
    }

    fn print_ip(&self) {
        print!("0x{:08X}  ", CHUNK_HEADER_RESERVED + self.ip);
    }
}
