//! Bytecoded compiler frontend.
pub mod bytecode;
mod chunk;
mod codegen;
mod codegen_bytecode;
pub mod constants;
mod disasm;
mod limits;

pub use self::chunk::Chunk;
use self::codegen::Codegen;
pub use self::codegen_bytecode::write_header;
pub use self::disasm::disassemble;

pub mod prelude {
    pub use super::chunk::Chunk;
    pub use super::codegen::Codegen;
}

pub fn compile(module: &vuur_parse::module::VuurModule) -> Result<Chunk, String> {
    let mut gen = codegen_bytecode::BytecodeCodegen::new();
    gen.compile(module)
}
