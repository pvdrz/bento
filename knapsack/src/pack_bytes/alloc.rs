use crate::{PackBytes, PackError};

impl PackBytes for Vec<u8> {
    fn pack_bytes(&mut self, bytes: &[u8]) -> Result<(), PackError> {
        self.extend_from_slice(bytes);

        Ok(())
    }
}
