use crate::{Pack, PackBytes, PackError};

impl Pack for &str {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_bytes().pack(packer)
    }
}
