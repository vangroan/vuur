//! Executable bytecode chunk.
use std::io::{Cursor, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::constants::*;
use crate::error::{CompileError, ErrorKind, Result};
use crate::util::ReadExt;

/// Binary chunk of executable byte code, intended for the interpreter VM.
pub struct Chunk {
    /// Bytecode
    pub(crate) code: Vec<u32>,
    /// Name of file where the original source was loaded.
    pub(crate) name: String,
    pub(crate) header: ChunkHeader,
}

impl Chunk {
    pub fn new<S>(name: S, code: Vec<u32>) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            code,
            header: ChunkHeader::empty(),
        }
    }

    pub fn from_code(code: Vec<u32>) -> Self {
        Self {
            name: CHUNK_DEFAULT_NAME.to_owned(),
            code,
            header: ChunkHeader::empty(),
        }
    }

    #[inline]
    pub fn code(&self) -> &[u32] {
        &self.code
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
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
        cursor
            .read_exact(&mut buf)
            .map_err(|err| CompileError::new(ErrorKind::Decode, "failed to read chunk header into buffer"))?;
        if buf != CHUNK_HEADER {
            return Err(CompileError::new(ErrorKind::Decode, "invalid chunk header"));
        }

        // Language Version
        let version = cursor.read_u8()?;
        // if version != CHUNK_VERSION {
        //     panic!("unexpected chunk version: {}", version);
        // }

        let endianess = cursor.read_u8()?;
        // TODO: integer size marker
        let size_t: u8 = cursor.read_u8()?;

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

    pub fn encode(&self, buf: &mut [u32]) -> std::io::Result<()> {
        let bytes = vec![];

        {
            let mut cursor = Cursor::new(bytes);

            cursor.write_u8(CHUNK_START_BYTE)?;
            cursor.write_all(CHUNK_HEADER)?;
            cursor.write_u8(self.version)?;
            cursor.write_u8(self.endianess)?;
            cursor.write_u8(self.size_t)?;

            // File format reserves bytes for future use.
            while cursor.position() < CHUNK_HEADER_RESERVED as u64 {
                cursor.write_u8(0)?;
            }
        }

        // FIXME
        // bytes.read_slice_u32(&mut buf)?;

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

pub(crate) type ByteCursor<'a> = Cursor<&'a [u8]>;
