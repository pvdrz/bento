use std::ffi::CStr;

use crate::{Pack, PackBytes, PackError};

impl Pack for &CStr {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.to_bytes().pack(packer)
    }
}
