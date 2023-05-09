//! Bytecoded compiler frontend.
pub mod bytecode;
mod chunk;
mod codegen_bytecode;
pub mod constants;
mod disasm;
mod error;
mod limits;
mod func;

pub use self::func::FuncDef;
pub use self::chunk::Chunk;
pub use self::disasm::disassemble;
pub use self::error::*;

pub fn compile(module: &vuur_parse::module::VuurModule) -> Result<Chunk> {
    let mut gen = codegen_bytecode::BytecodeCodegen::new();
    gen.compile(module)
}
