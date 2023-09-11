use crate::UnpackError;

/// A type that can binarily deserialize any value whose type implements [`Unpack`](`crate::Unpack`).
///
/// Types that implement this trait are usually called unpackers.
pub trait UnpackBytes {
    /// Attempts to read the exactly number of bytes required to fill `bytes`.
    fn unpack_bytes(&mut self, bytes: &mut [u8]) -> Result<(), UnpackError>;
}

impl<U: UnpackBytes> UnpackBytes for &mut U {
    fn unpack_bytes(&mut self, bytes: &mut [u8]) -> Result<(), UnpackError> {
        (*self).unpack_bytes(bytes)
    }
}

impl UnpackBytes for &[u8] {
    fn unpack_bytes(&mut self, bytes: &mut [u8]) -> Result<(), UnpackError> {
        if self.len() < bytes.len() {
            return Err(UnpackError::NotEnoughBytes);
        }

        let (head, tail) = self.split_at(bytes.len());
        bytes.copy_from_slice(head);
        *self = tail;

        Ok(())
    }
}
