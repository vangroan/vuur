//! Disassembler
use std::fmt;

use crate::bytecode::{get_opcode, opcodes, read_arg_a, read_arg_k};
use crate::chunk::Chunk;
use crate::constants::*;
use crate::error::Result;

pub fn disassemble<W>(f: &mut W, chunk: &Chunk) -> Result<()>
where
    W: fmt::Write,
{
    // let mut cursor = Cursor::new(chunk.code.as_slice());

    // Check Header for chunk compatibility
    writeln!(f, "{}", chunk.header)?;

    // TODO: Read function definitions

    let mut ip = 0;
    while ip < chunk.code.len() {
        let instruction = chunk.code[ip];

        let byte_offset = CHUNK_HEADER_RESERVED + ip * std::mem::size_of::<u32>();
        write!(f, "  ")?;
        write_instruction_hex(f, byte_offset, instruction)?;
        write!(f, "  ")?;

        let opcode = get_opcode(instruction);
        match opcode {
            opcodes::NOOP => {
                /* skip */
                write!(f, ".noop: {}", opcode)?
            }
            opcodes::ADD_I32 => write!(f, ".add")?,
            opcodes::SUB_I32 => write!(f, ".sub")?,
            opcodes::MUL_I32 => write!(f, ".mul")?,
            opcodes::PUSH_CONST => write!(f, ".pushk\t{}", read_arg_k(instruction))?,
            opcodes::PUSH_CONST_IMM => write!(f, ".pushi\t{}", read_arg_a(instruction))?,
            opcodes::FUNC => write!(f, ".function")?,
            opcodes::RETURN => write!(f, ".return")?,
            opcodes::ABORT => write!(f, ".abort")?,
            _ => write!(f, "UNKNOWN")?,
        }

        write!(f, "\n")?;
        ip += 1;
    }

    Ok(())
}

#[rustfmt::skip]
fn write_instruction_hex<W>(f: &mut W, offset: usize, instruction: u32) -> fmt::Result
where
    W: fmt::Write,
{
    let [o, a, b, c] = instruction.to_le_bytes();
    write!(f,
        "0x{:08X}  {:02X} {:02X} {:02X} {:02X}",
        offset, o, a, b, c
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bytecode::opcodes::*;
    use crate::chunk::ChunkHeader;
    use crate::codegen_bytecode::BytecodeChunkExt;

    #[test]
    fn test_basic_disassemble() {
        let mut buf = String::new();
        let mut chunk = Chunk::default();
        chunk.header = ChunkHeader {
            version: CHUNK_VERSION,
            endianess: CHUNK_ENDIAN_LIT,
            size_t: CHUNK_SIZE_32,
        };
        chunk.emit_a(PUSH_CONST, 0);
        chunk.emit_a(PUSH_CONST, 1);
        chunk.emit_simple(ADD_I32);
        chunk.emit_simple(RETURN);

        disassemble(&mut buf, &chunk).expect("failed to disassemble binary chunk");

        println!("{}", buf);
    }
}
