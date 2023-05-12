//! Disassembler
use std::fmt;

use crate::bytecode::{decode_arg_a, decode_arg_k, decode_opcode, opcodes};
use crate::chunk::Chunk;
use crate::constants::*;
use crate::error::Result;

pub fn disassemble<W>(f: &mut W, chunk: &Chunk) -> Result<()>
where
    W: fmt::Write,
{
    writeln!(f, "{}", chunk.header)?;
    writeln!(f, "")?;

    // TODO: Read function definitions

    // column headings
    writeln!(f, "      offset  00 08 16 24")?;
    writeln!(f, "------------  -----------")?;

    let mut ip = 0;
    while ip < chunk.code.len() {
        if let Some(func) = chunk.funcs.iter().find(|f| f.bytecode_span.0 == ip as u32) {
            let func_id = func.id.map(|id| id.to_usize()).unwrap_or(0);
            writeln!(f, "\nfunc \"...\" {func_id}:")?;
        }

        let instruction = chunk.code[ip];

        let byte_offset = ip * std::mem::size_of::<u32>();
        write!(f, "  ")?;
        write_instruction_hex(f, byte_offset, instruction)?;
        write!(f, "  ")?;

        let opcode = decode_opcode(instruction);
        match opcode {
            opcodes::NOOP => { /* skip */ }
            opcodes::POP => write!(f, "pop")?,
            opcodes::ADD_I32 => write!(f, "add.i32")?,
            opcodes::SUB_I32 => write!(f, "sub.i32")?,
            opcodes::MUL_I32 => write!(f, "mul.i32")?,
            opcodes::DIV_I32 => write!(f, "div.i32")?,
            opcodes::NEG_I32 => write!(f, "neg.i32")?,
            opcodes::EQ_I32 => write!(f, "eq.i32")?,
            opcodes::PUSH_CONST => write!(f, "pushk\t{}", decode_arg_k(instruction))?,
            opcodes::PUSH_CONST_IMM => write!(f, "push.i32.im\t{}", decode_arg_a(instruction))?,
            opcodes::PUSH_LOCAL_I32 => write!(f, "local.i32\t{}", decode_arg_k(instruction))?,
            opcodes::FUNC => write!(f, "function")?,
            opcodes::SKIP_1 => write!(f, "skip.i32.1")?,
            opcodes::SKIP_EQ_I32 => write!(f, "skip.eq.i32")?,
            opcodes::CALL => write!(f, "call \t{}", decode_arg_k(instruction))?,
            opcodes::RETURN => write!(f, "return\t{}", decode_arg_k(instruction))?,
            opcodes::JUMP => write!(
                f,
                "jump\t0x{:X}",
                decode_arg_k(instruction) * std::mem::size_of::<u32>() as u32
            )?,
            opcodes::ABORT => write!(f, "abort")?,
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
    let [i, a, b, c] = instruction.to_le_bytes();
    write!(f,
        "0x{:08X}  {:02X} {:02X} {:02X} {:02X}",
        offset, i, a, b, c,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bytecode::{opcodes::*, WriteBytecode};
    use crate::chunk::ChunkHeader;

    #[test]
    fn test_basic_disassemble() {
        let mut buf = String::new();
        let mut chunk = Chunk::default();
        chunk.header = ChunkHeader {
            version: CHUNK_VERSION,
            endianess: CHUNK_ENDIAN_LIT,
            size_t: CHUNK_SIZE_32,
        };
        let code = chunk.code_mut();
        code.write_a(PUSH_CONST, 0).unwrap();
        code.write_a(PUSH_CONST, 1).unwrap();
        code.write_simple(ADD_I32).unwrap();
        code.write_simple(RETURN).unwrap();

        disassemble(&mut buf, &chunk).expect("failed to disassemble binary chunk");

        println!("{}", buf);
    }
}
