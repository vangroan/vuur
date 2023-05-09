//! Bytecoded compiler frontend.
pub mod bytecode;
mod chunk;
mod codegen;
pub mod constants;
mod disasm;
mod error;
mod func;
mod limits;

pub use self::chunk::Chunk;
pub use self::disasm::disassemble;
pub use self::error::*;
pub use self::func::FuncDef;

pub fn compile(module: &vuur_parse::module::VuurModule) -> Result<Chunk> {
    let mut gen = codegen::BytecodeCodegen::new();
    gen.compile(module)
}
