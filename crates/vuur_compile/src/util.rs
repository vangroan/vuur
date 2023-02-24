use std::io::{Error, ErrorKind, Result};

pub trait ReadExt {
    fn read_slice_u32(&self, target: &mut [u32]) -> Result<()>;
}

const U32_STRIDE: usize = 4;

impl ReadExt for &[u8] {
    fn read_slice_u32(&self, target: &mut [u32]) -> Result<()> {
        let end = usize::min(self.len(), target.len() >> 2); // div by 4

        // TODO: padding
        for i in (0..end).step_by(U32_STRIDE) {
            let parts = [self[i], self[i + 1], self[i + 2], self[i + 3]];
            let el = u32::from_le_bytes(parts);
            target[i >> 2] = el; // div by 4
        }

        Ok(())
    }
}

impl ReadExt for Vec<u8> {
    fn read_slice_u32(&self, target: &mut [u32]) -> Result<()> {
        (&self).read_slice_u32(target)
    }
}
