use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl<T: Pack> Pack for Box<T> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl<T: Unpack> Unpack for Box<T> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        T::unpack(unpacker).map(Box::new)
    }
}
