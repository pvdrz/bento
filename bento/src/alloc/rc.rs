use std::{
    ffi::{CStr, CString},
    rc::Rc,
};

use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl<T: Pack> Pack for Rc<T> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl<T: Unpack> Unpack for Rc<T> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        T::unpack(unpacker).map(Self::new)
    }
}

impl Pack for Rc<[u8]> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl Unpack for Rc<[u8]> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        <Vec<u8>>::unpack(unpacker).map(Self::from)
    }
}

impl<T: Unpack> Unpack for Rc<[T]> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        <Vec<T>>::unpack(unpacker).map(Self::from)
    }
}

impl<T: Pack> Pack for Rc<[T]> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl Unpack for Rc<str> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        String::unpack(unpacker).map(Self::from)
    }
}

impl Pack for Rc<str> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}

impl Unpack for Rc<CStr> {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        CString::unpack(unpacker).map(Self::from)
    }
}

impl Pack for Rc<CStr> {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        self.as_ref().pack(packer)
    }
}
