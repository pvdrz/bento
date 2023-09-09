use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl Pack for Vec<u8> {
    fn pack<P: PackBytes>(&self, mut packer: P) -> Result<(), PackError> {
        self.len().pack(&mut packer)?;

        packer.pack_bytes(self.as_slice())
    }
}

impl<T: Pack> Pack for Vec<T> {
    fn pack<P: PackBytes>(&self, mut packer: P) -> Result<(), PackError> {
        self.len().pack(&mut packer)?;

        for item in self.iter() {
            item.pack(&mut packer)?;
        }

        Ok(())
    }
}

impl Unpack for Vec<u8> {
    fn unpack<U: UnpackBytes>(mut unpacker: U) -> Result<Self, UnpackError> {
        let len = u64::unpack(&mut unpacker)?;

        let mut vec = vec![0u8; len as usize];

        unpacker.unpack_bytes(&mut vec)?;

        Ok(vec)
    }
}

impl<T: Unpack> Unpack for Vec<T> {
    fn unpack<U: UnpackBytes>(mut unpacker: U) -> Result<Self, UnpackError> {
        let len = u64::unpack(&mut unpacker)?;

        let mut vec = Vec::with_capacity(len as usize);

        for _ in 0..len {
            vec.push(T::unpack(&mut unpacker)?);
        }

        Ok(vec)
    }
}
