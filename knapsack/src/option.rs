use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl<T: Pack> Pack for Option<T> {
    fn pack<P: PackBytes>(&self, mut packer: P) -> Result<(), PackError> {
        match self.as_ref() {
            Some(item) => {
                [1u8].pack(&mut packer)?;
                item.pack(packer)
            }
            None => [0u8].pack(packer),
        }
    }
}

impl<T: Unpack> Unpack for Option<T> {
    fn unpack<U: UnpackBytes>(mut unpacker: U) -> Result<Self, UnpackError> {
        match <[u8; 1]>::unpack(&mut unpacker)?[0] {
            0 => Ok(None),
            1 => T::unpack(unpacker).map(Some),
            prefix => Err(format!("Invalid prefix for option: {prefix}").into()),
        }
    }
}
