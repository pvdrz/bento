use std::borrow::Cow;

use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl<'a, B: Pack + ToOwned + ?Sized + 'a> Pack for Cow<'a, B> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl<'a, B: ToOwned + ?Sized + 'a> Unpack for Cow<'a, B>
where
    B::Owned: Unpack,
{
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        B::Owned::unpack(unpacker).map(Self::Owned)
    }
}
