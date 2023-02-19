//! Disassembler
use std::fmt::{self};
use std::io::{Cursor, Read};

use byteorder::ReadBytesExt;

use crate::bytecode::Instruction;
use crate::chunk::Chunk;
use crate::constants::*;

pub fn disassemble<W>(f: &mut W, chunk: &Chunk) -> Result
where
    W: fmt::Write,
{
    // Initial byte read would fail.
    if chunk.is_empty() {
        return Err(DissasmError::ChunkEmpty);
    }

    let mut cursor: ByteCursor = Cursor::new(chunk.code.as_slice());

    // Check Header for chunk compatibility
    decode_header(f, &mut cursor)?;

    // TODO: Read function definitions

    println!(".module");

    // Each instruction is 32-bit
    let mut buf = [0_u8; INSTRUCTION_SIZE];
    let mut offset = cursor.position();

    while let Ok(_) = cursor.read_exact(&mut buf) {
        match Instruction::try_from(buf[0]) {
            Ok(opcode) => {
                write!(
                    f,
                    "  0x{:08X} {:02X} {:02X} {:02X} {:02X} ",
                    offset, buf[0], buf[1], buf[2], buf[3]
                )?;

                match opcode {
                    Instruction::Add_I32 => {
                        write!(f, ".add")?;
                    }
                    Instruction::Sub_I32 => {
                        write!(f, ".sub")?;
                    }
                    Instruction::Mul_I32 => {
                        write!(f, ".mul")?;
                    }
                    Instruction::PushConst => {
                        write!(f, ".pushk\t{}", buf[1])?;
                    }
                    Instruction::PushConst_Imm => {
                        write!(f, ".pushi\t{}", buf[1])?;
                    }
                    Instruction::Return => {
                        write!(f, ".return")?;
                    }
                    _ => write!(f, "UNKNOWN")?,
                }

                write!(f, "\n")?;
            }
            Err(_) => { /* skip */ }
        }

        offset = cursor.position();
    }

    Ok(())
}

fn decode_header<W>(f: &mut W, cursor: &mut ByteCursor) -> Result
where
    W: fmt::Write,
{
    // Starting Byte
    if cursor.read_u8()? != CHUNK_START_BYTE {
        return Err(DissasmError::Message("invalid start byte"));
    }

    // Header Marker
    let mut buf = [0_u8; CHUNK_HEADER.len()];
    cursor.read_exact(&mut buf)?;
    if buf != CHUNK_HEADER {
        panic!("invalid chunk header marker");
    }

    // Language Version
    let version = cursor.read_u8()?;
    if version != CHUNK_VERSION {
        panic!("unexpected chunk version: {}", version);
    }

    // TODO: Endianess
    let endianess = cursor.read_u8()?;

    // TODO: integer size marker
    let size_t: u8 = cursor.read_u8()?;

    debug_assert!(
        (cursor.position() as usize) <= CHUNK_HEADER_RESERVED,
        "header decoding read beyond the reserved space"
    );

    while (cursor.position() as usize) < CHUNK_HEADER_RESERVED {
        cursor.read_u8()?;
    }

    writeln!(f, "vuur v{:x} endian:{:?} size:{:?}", version, endianess, size_t)?;

    Ok(())
}

type ByteCursor<'a> = Cursor<&'a [u8]>;

pub type Result = std::result::Result<(), DissasmError>;

#[derive(Debug)]
pub enum DissasmError {
    ChunkEmpty,
    Message(&'static str),
    Fmt(std::fmt::Error),
    Io(std::io::Error),
}

impl fmt::Display for DissasmError {
    #[cold]
    #[inline(never)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to disassemble chunk: ")?;
        match self {
            Self::ChunkEmpty => write!(f, "binary chunk is empty"),
            Self::Message(msg) => fmt::Display::fmt(msg, f),
            Self::Fmt(err) => fmt::Display::fmt(err, f),
            Self::Io(err) => match err.kind() {
                std::io::ErrorKind::UnexpectedEof => {
                    write!(f, "unexpected end-of-file: {}", err)
                }
                _ => fmt::Display::fmt(err, f),
            },
        }
    }
}

impl From<std::io::Error> for DissasmError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<std::fmt::Error> for DissasmError {
    fn from(err: std::fmt::Error) -> Self {
        Self::Fmt(err)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::codegen_bytecode::write_header;

    #[test]
    fn test_basic_disassemble() {
        let mut buf = String::new();
        let mut chunk = Chunk::default();
        write_header(&mut chunk);

        if let Err(err) = disassemble(&mut buf, &chunk) {
            eprintln!("{}", err);
        }

        disassemble(&mut buf, &chunk).expect("failed to disassemble binary chunk");

        println!("{:?}", buf);

        todo!();
    }
}
