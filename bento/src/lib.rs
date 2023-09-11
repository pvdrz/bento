#![doc = include_str!("../../README.md")]

mod pack_bytes;
mod unpack_bytes;

mod alloc;
mod array;
mod num;
mod option;
mod slice;
mod str;
mod bool;
mod tuple;

use core::fmt;
use std::error::Error;

#[cfg(feature = "derive")]
pub use bento_derive::{Unpack, Pack};

pub use pack_bytes::{PackBytes, SlicePacker};
pub use unpack_bytes::UnpackBytes;

/// An error that can happen while packing a value.
#[derive(Debug)]
pub enum PackError {
    /// The [packer](`PackBytes`) did not have enough capacity to pack the value. 
    NotEnoughBytes,
}


/// An error that can happen while unpacking a value.
#[derive(Debug)]
pub enum UnpackError {
    /// The [unpacker](`UnpackBytes`) unexpectly ran out of bytes while unpacking a value. 
    UnexpectedEOF,
    /// A validation error specific to the [`Unpack`] implementation.
    Validation(Box<dyn Error + 'static>),
}

impl fmt::Display for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnpackError::UnexpectedEOF => "not enough bytes while unpacking".fmt(f),
            UnpackError::Validation(err) => write!(f, "validation error: {err}"),
        }
    }
}

impl<E: Into<Box<dyn Error + 'static>>> From<E> for UnpackError {
    fn from(err: E) -> Self {
        Self::Validation(err.into())
    }
}

/// A type that can be binarily serialized.
pub trait Pack {
    /// Pack this value into the given packer.
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError>;
}

/// A type that can be binarily deserialized.
pub trait Unpack: Sized {
    /// Unpack this value from the given packer.
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError>;
}

mod sealed {
    use crate::Pack;

    pub trait Sealed {}
    impl<T: Pack + ?Sized> Sealed for T {}
}

/// An extension trait for types that implement [`Pack`].
///
/// This trait is sealed.
pub trait PackExt: sealed::Sealed {
    /// Compute the number of bytes that are required to pack this value without actually packing
    /// it.
    fn packed_len(&self) -> usize;

    /// Pack this value into a `Vec<u8>`.
    ///
    /// The buffer is preemptively created with a capacity equal to the value returned by
    /// [`PackExt::packed_len`] to avoid unnecessary allocations.
    fn pack_to_vec(&self) -> Vec<u8>;
}

impl<T: Pack + ?Sized> PackExt for T {
    fn packed_len(&self) -> usize {
        let mut packer = pack_bytes::LenPacker::new();
        // This cannot panic as `LenPacker::pack_bytes` only increases an integer counter.
        self.pack(&mut packer).unwrap();

        packer.len()
    }

    fn pack_to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.packed_len());
        // This cannot panic as packing to a `Vec` is infallible. 
        self.pack(&mut buf).unwrap();

        buf
    }
}
