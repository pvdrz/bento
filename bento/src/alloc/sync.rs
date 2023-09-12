use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl<T: Pack> Pack for Arc<T> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl<T: Unpack> Unpack for Arc<T> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        T::unpack(unpacker).map(Self::new)
    }
}

impl Pack for Arc<[u8]> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl Unpack for Arc<[u8]> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        <Vec<u8>>::unpack(unpacker).map(Self::from)
    }
}

impl<T: Unpack> Unpack for Arc<[T]> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        <Vec<T>>::unpack(unpacker).map(Self::from)
    }
}

impl<T: Pack> Pack for Arc<[T]> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl Unpack for Arc<str> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        String::unpack(unpacker).map(Self::from)
    }
}

impl Pack for Arc<str> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl Unpack for Arc<CStr> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        CString::unpack(unpacker).map(Self::from)
    }
}

impl Pack for Arc<CStr> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}
