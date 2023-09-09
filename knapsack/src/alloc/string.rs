use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl Pack for String {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_bytes().pack(packer)
    }
}

impl Unpack for String {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        let bytes = Vec::<u8>::unpack(unpacker)?;
        let string = String::from_utf8(bytes)?;

        Ok(string)
    }
}
