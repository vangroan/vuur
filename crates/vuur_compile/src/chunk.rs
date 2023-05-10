//! Executable bytecode chunk.
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::constants::*;
use crate::error::{CompileError, ErrorKind, Result};
use crate::func::{FuncDef, FuncId};
use crate::limits::*;

// TODO: Serialise and deserialise chunk to binary file

/// Binary chunk of executable byte code, intended for the interpreter VM.
pub struct Chunk {
    /// Bytecode
    pub(crate) code: Vec<u32>,
    pub(crate) funcs: Vec<FuncDef>,
    pub(crate) data: Vec<Box<[u8]>>,
    /// Name of file where the original source was loaded.
    pub(crate) name: String,
    pub(crate) header: ChunkHeader,
    pub(crate) entrypoint: Option<FuncId>,
}

impl Chunk {
    pub fn new<S>(name: S, code: Vec<u32>) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            funcs: vec![Self::stub_func_def()],
            data: Vec::new(),
            code,
            header: ChunkHeader::empty(),
            entrypoint: None,
        }
    }

    pub fn from_code(code: Vec<u32>) -> Self {
        Self {
            name: CHUNK_DEFAULT_NAME.to_owned(),
            funcs: vec![Self::stub_func_def()],
            data: Vec::new(),
            code,
            header: ChunkHeader::empty(),
            entrypoint: None,
        }
    }

    pub fn entrypoint(&self) -> Option<FuncId> {
        self.entrypoint
    }

    #[inline]
    pub fn func_by_id(&self, func_id: u32) -> Option<&FuncDef> {
        self.funcs.get(func_id as usize)
    }

    fn stub_func_def() -> FuncDef {
        FuncDef {
            id: None,
            // Point bytecode to end of chunk to avoid conflicts with real functions.
            bytecode_span: (std::u32::MAX, std::u32::MAX),
            arity: 0,
        }
    }

    #[inline]
    pub fn code(&self) -> &[u32] {
        &self.code
    }

    #[inline]
    pub(crate) fn code_mut(&mut self) -> &mut Vec<u32> {
        &mut self.code
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub fn encode(&self, buffer: &mut Vec<u8>) -> Result<()> {
        self.header.encode_vec(buffer)?;

        let mut cursor = Cursor::new(buffer);
        cursor.seek(SeekFrom::Start(CHUNK_HEADER_RESERVED as u64))?;

        for instruction in self.code.iter().cloned() {
            cursor.write_u32::<LittleEndian>(instruction)?;
        }

        Ok(())
    }

    pub(crate) fn replace_func_stub(&mut self, func: FuncDef) {
        assert!(func.id.is_some(), "function definition must have an ID");
        let index = func.id.unwrap().to_usize();
        self.funcs[index] = func;
    }

    /// Adds a function definition to the chunk's function table.
    pub(crate) fn add_func(&mut self, mut func: FuncDef) -> FuncId {
        assert!(self.funcs.len() < MAX_FUNCS, "maximum number of functions reached");
        assert!(self.funcs.len() > 0, "function table must start at 1");
        let next_id = FuncId::new(self.funcs.len() as u32);
        func.id = next_id;
        self.funcs.push(func);
        next_id.unwrap()
    }

    /// Adds a stub function to the chunk to reserve a function ID.
    pub(crate) fn add_func_stub(&mut self) -> FuncId {
        self.add_func(Self::stub_func_def())
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::from_code(vec![])
    }
}

#[derive(Debug)]
pub struct ChunkHeader {
    pub version: u8,
    pub endianess: u8,
    pub size_t: u8,
}

impl ChunkHeader {
    pub(crate) fn empty() -> Self {
        ChunkHeader {
            version: 0,
            endianess: 0,
            size_t: CHUNK_SIZE_32,
        }
    }

    pub fn decode(buf: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(buf);

        // Starting Byte
        if cursor.read_u8()? != CHUNK_START_BYTE {
            return Err(CompileError::new(
                ErrorKind::Decode,
                "invalid starting byte for chunk header",
            ));
        }

        // Header Marker
        let mut buf = [0_u8; CHUNK_HEADER.len()];
        cursor.read_exact(&mut buf).map_err(|err| {
            CompileError::new(
                ErrorKind::Decode,
                format!("failed to read chunk header into buffer: {}", err),
            )
        })?;
        if buf != CHUNK_HEADER {
            return Err(CompileError::new(ErrorKind::Decode, "invalid chunk header"));
        }

        // Language Version
        let version = cursor.read_u8()?;

        let endianess = cursor.read_u8()?;
        let size_t: u8 = cursor.read_u8()?; // TODO: integer size marker

        debug_assert!(
            (cursor.position() as usize) <= CHUNK_HEADER_RESERVED,
            "header decoding read beyond the reserved space"
        );

        let chunk_header = ChunkHeader {
            version,
            endianess,
            size_t,
        };
        log::debug!("decoded chunk header: {}", chunk_header);
        Ok(chunk_header)
    }

    pub fn encode(&self, buf: &mut [u8]) -> Result<()> {
        debug_assert!(
            buf.len() >= CHUNK_HEADER_RESERVED,
            "buffer is too small for chunk header"
        );

        let mut cursor = Cursor::new(buf);

        cursor.write_u8(CHUNK_START_BYTE)?;
        cursor.write_all(CHUNK_HEADER)?;
        cursor.write_u8(self.version)?;
        cursor.write_u8(self.endianess)?;
        cursor.write_u8(self.size_t)?;

        // File format reserves bytes for future use.
        while cursor.position() < CHUNK_HEADER_RESERVED as u64 {
            cursor.write_u8(0)?;
        }

        Ok(())
    }

    pub fn encode_vec(&self, vec: &mut Vec<u8>) -> Result<()> {
        // let start = if vec.is_empty() { 0 } else { vec.len() };
        // vec.extend((0..CHUNK_HEADER_RESERVED).map(|_| 0));
        // println!("vec len {}", vec.len());
        // self.encode(&mut vec[start..start + CHUNK_HEADER_RESERVED])
        let mut cursor = Cursor::new(vec);

        cursor.write_u8(CHUNK_START_BYTE)?;
        cursor.write_all(CHUNK_HEADER)?;
        cursor.write_u8(self.version)?;
        cursor.write_u8(self.endianess)?;
        cursor.write_u8(self.size_t)?;

        // File format reserves bytes for future use.
        while cursor.position() < CHUNK_HEADER_RESERVED as u64 {
            cursor.write_u8(0)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ChunkHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let endianess = match self.endianess {
            CHUNK_ENDIAN_LIT => "LE",
            CHUNK_ENDIAN_BIG => "BE",
            _ => "??",
        };

        write!(f, "vuur v{:x} endian:{} size:{}", self.version, endianess, self.size_t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_VERSION: u8 = 42;

    #[test]
    #[rustfmt::skip]
    fn test_header_decode() {
        let bytes: &[u8; CHUNK_HEADER_RESERVED] = &[
            CHUNK_START_BYTE,
            0x76, 0x75, 0x75, 0x72, 0x0, // "vuur\0"
            0x2A,  // version
            0x1,   // little endian
            0x4,   // 32-bit usize
            0, 0, 0, 0, 0, 0, 0,
        ];

        let header = ChunkHeader::decode(bytes).expect("failed to decode chunk header");

        assert_eq!(header.version, TEST_VERSION);
        assert_eq!(header.endianess, CHUNK_ENDIAN_LIT);
        assert_eq!(header.size_t, CHUNK_SIZE_32);
    }

    #[test]
    #[rustfmt::skip]
    fn test_header_encode() {
        let header = ChunkHeader {
            version: TEST_VERSION, endianess: CHUNK_ENDIAN_LIT, size_t: CHUNK_SIZE_32
        };

        let mut buf = [0u8; CHUNK_HEADER_RESERVED];
        header.encode(&mut buf).expect("failed to encode chunk header");

        let expected = &[
            CHUNK_START_BYTE,
            0x76, 0x75, 0x75, 0x72, 0x0, // "vuur\0"
            0x2A,  // version
            0x1,   // little endian
            0x4,   // 32-bit usize
            0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(&buf, expected);
    }
}
