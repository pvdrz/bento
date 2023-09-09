use crate::{Pack, PackBytes, PackError};

impl Pack for [u8] {
    fn pack<P: PackBytes>(&self, mut packer: P) -> Result<(), PackError> {
        self.len().pack(&mut packer)?;

        packer.pack_bytes(self)
    }
}

impl<T: Pack> Pack for &[T] {
    fn pack<P: PackBytes>(&self, mut packer: P) -> Result<(), PackError> {
        self.len().pack(&mut packer)?;

        for item in *self {
            item.pack(&mut packer)?;
        }

        Ok(())
    }
}
