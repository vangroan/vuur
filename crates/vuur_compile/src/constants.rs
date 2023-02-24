//! Compiler constants.

/// First byte of a binary chunk.
pub const CHUNK_START_BYTE: u8 = 0x1B; // ASCII ESCAPE

/// Header marker of a binary chunk following the first byte,
/// which marks it as executable Vuur bytecode.
pub const CHUNK_HEADER: &[u8] = b"vuur\0";

/// Version of the chunk binary format.
pub const CHUNK_VERSION: u8 = 0x01;

pub const CHUNK_ENDIAN_LIT: u8 = 1;
pub const CHUNK_ENDIAN_BIG: u8 = 2;

pub const CHUNK_SIZE_32: u8 = std::mem::size_of::<u32>() as u8; // 4
pub const CHUNK_SIZE_64: u8 = std::mem::size_of::<u64>() as u8; // 8

/// Number of total bytes reserved by the chunk header format.
pub const CHUNK_HEADER_RESERVED: usize = 16;

/// Default name of chunk when compiled from a source without
/// a file name, like from a REPL or a Rust string literal.
pub const CHUNK_DEFAULT_NAME: &str = "<script>";

/// Size in bytes of a single instruction.
pub const INSTRUCTION_SIZE: usize = 4; // 32-bit

/// Maximum signed integer that can be stored in the 24-bit argument A.
pub const INSTRUCTION_A_MAX: i32 = 0x7FFFFF;
