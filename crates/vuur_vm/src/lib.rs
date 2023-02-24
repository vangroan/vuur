use std::cell::RefCell;
use std::rc::Rc;

use vuur_compile::bytecode::opcodes as ops;
use vuur_compile::constants::CHUNK_HEADER_RESERVED;
use vuur_compile::Chunk;

pub const STRIDE: usize = 4;

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
                fiber.ip = CHUNK_HEADER_RESERVED;
                fiber.run(chunk);
                if fiber.done {
                    // Fiber is done executing, and cannot be resumed.
                    // TODO: Return top stack value
                    println!("fiber is done");
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
            calls: vec![FrameInfo { return_addr: 0 }],
            done: false,
        }
    }

    pub fn run(&mut self, chunk: &Chunk) {
        'eval: loop {
            let op = chunk.code()[self.ip];

            self.print_ip();
            print!(
                "{:02X} {:02X} {:02X} {:02X} ",
                instruction[0], instruction[1], instruction[2], instruction[3]
            );

            let op = instruction.get(0).cloned().unwrap_or(ops::NOOP);

            match op {
                ops::NOOP => self.incr(),
                ops::ADD_I32 => {
                    println!("add");
                    self.incr()
                }
                ops::SUB_I32 => {
                    println!("sub");
                    self.incr()
                }
                ops::MUL_I32 => {
                    println!("mul");
                    self.incr()
                }
                ops::PUSH_CONST => {
                    let konst_idx = instruction.get(1).cloned().unwrap_or_default();
                    println!("pushk {}", konst_idx);
                    self.incr();
                }
                ops::RETURN => {
                    let frame = self.calls.pop().unwrap_or_else(FrameInfo::default);
                    println!("return to 0x{:04X}", frame.return_addr);
                    self.ip = frame.return_addr;
                }
                _ => {
                    println!("abort");
                    self.done = true;
                    break 'eval;
                }
            }
        }
    }

    fn print_ip(&self) {
        print!("0x{:08X} ", self.ip);
    }

    #[inline(always)]
    fn incr(&mut self) {
        self.ip += STRIDE
    }
}
