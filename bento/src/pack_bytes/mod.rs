mod alloc;

use crate::PackError;

pub trait PackBytes {
    fn pack_bytes(&mut self, bytes: &[u8]) -> Result<(), PackError>;
}

impl<P: PackBytes> PackBytes for &mut P {
    fn pack_bytes(&mut self, bytes: &[u8]) -> Result<(), PackError> {
        (*self).pack_bytes(bytes)
    }
}

pub struct SlicePacker<'p> {
    offset: usize,
    slice: &'p mut [u8],
}

impl<'p> From<&'p mut [u8]> for SlicePacker<'p> {
    fn from(slice: &'p mut [u8]) -> Self {
        Self { offset: 0, slice }
    }
}

impl<'p> PackBytes for SlicePacker<'p> {
    fn pack_bytes(&mut self, bytes: &[u8]) -> Result<(), PackError> {
        let len = bytes.len();
        let slice = &mut self.slice[self.offset..];

        if slice.len() >= len {
            slice.copy_from_slice(bytes);
            self.offset += len;

            Ok(())
        } else {
            Err(PackError::UnexpectedEOF)
        }
    }
}

pub(crate) struct LenPacker {
    len: usize,
}

impl LenPacker {
    pub(crate) const fn new() -> Self {
        Self { len: 0 }
    }

    pub(crate) const fn len(&self) -> usize {
        self.len
    }
}

impl PackBytes for LenPacker {
    fn pack_bytes(&mut self, bytes: &[u8]) -> Result<(), PackError> {
        self.len += bytes.len();

        Ok(())
    }
}

