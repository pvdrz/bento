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

impl Pack for Box<[u8]> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl Unpack for Box<[u8]> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        <Vec<u8>>::unpack(unpacker).map(|vec| vec.into_boxed_slice())
    }
}

impl<T: Unpack> Unpack for Box<[T]> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        <Vec<T>>::unpack(unpacker).map(|vec| vec.into_boxed_slice())
    }
}

impl<T: Pack> Pack for Box<[T]> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl Unpack for Box<str> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        String::unpack(unpacker).map(|string| string.into_boxed_str())
    }
}

impl Pack for Box<str> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}
