//! Executable bytecode chunk.
use crate::constants::CHUNK_DEFAULT_NAME;

/// Binary chunk of executable code.
pub struct Chunk {
    /// Bytecode
    pub(crate) code: Vec<u8>,
    /// Name of file where the original source was loaded.
    pub(crate) name: String,
    konst_i32: Vec<i32>,
}

impl Chunk {
    pub fn new<S>(name: S, code: Vec<u8>) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            code,
            konst_i32: vec![],
        }
    }

    pub fn from_code(code: Vec<u8>) -> Self {
        Self {
            name: CHUNK_DEFAULT_NAME.to_owned(),
            code,
            konst_i32: vec![],
        }
    }

    #[inline]
    pub fn code(&self) -> &[u8] {
        &self.code
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn constant_i32(&self, index: usize) -> Option<i32> {
        self.konst_i32.get(index).cloned()
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
