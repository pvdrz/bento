use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl Pack for bool {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        if *self {
            [1u8].pack(packer)
        } else {
            [0u8].pack(packer)
        }
    }
}

impl Unpack for bool {
    fn unpack<U: UnpackBytes>(mut unpacker: U) -> Result<Self, UnpackError> {
        let mut bytes = [0u8];
        unpacker.unpack_bytes(&mut bytes)?;

        match bytes[0] {
            0u8 => Ok(false),
            1u8 => Ok(true),
            _ => Err("invalid boolean byte".into()),
        }
    }
}
