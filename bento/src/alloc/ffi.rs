use std::ffi::CString;

use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl Pack for CString {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_bytes().pack(packer)
    }
}

impl Unpack for CString {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        let bytes = Vec::<u8>::unpack(unpacker)?;
        let cstring = CString::from_vec_with_nul(bytes)?;

        Ok(cstring)
    }
}
