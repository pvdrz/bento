//! A simple binary serialization and deserialization framework
//!
//! This crate provides the [`Pack`] and [`Unpack`] traits for types that can be serialized and
//! deserialized respectively. These two traits are implemented for some of the standard library
//! types.
//!
//! Knapsack also provides the [`PackBytes`] and [`UnpackBytes`] traits for types that can can be
//! used to read and write bytes respectively.
//!
//! # Example
//! ```rust
//! use bento::{Pack, Unpack};
//!
//! // Some data we want to serialize.
//! let data = vec!["hello", "world"];
//!
//! // A buffer where we will write the serialized data.
//! let mut buffer = vec![];
//!
//! // Serialize the data and write it into the buffer.
//! data.pack(&mut buffer).unwrap();
//!
//! // Deserialize the data by reading it from the buffer.
//! let unpacked_data = Vec::<String>::unpack(buffer.as_slice()).unwrap();
//!
//! // Check that we deserialized the same data that we had before.
//! assert_eq!(data.len(), unpacked_data.len());
//! assert_eq!(unpacked_data[0], "hello");
//! assert_eq!(unpacked_data[1], "world");
//! ```
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

#[derive(Debug)]
pub enum PackError {
    UnexpectedEOF,
}

#[derive(Debug)]
pub enum UnpackError {
    NotEnoughBytes,
    Validation(Box<dyn Error + 'static>),
}

impl fmt::Display for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnpackError::NotEnoughBytes => "not enough bytes while unpacking".fmt(f),
            UnpackError::Validation(err) => write!(f, "validation error: {err}"),
        }
    }
}

impl<E: Into<Box<dyn Error + 'static>>> From<E> for UnpackError {
    fn from(err: E) -> Self {
        Self::Validation(err.into())
    }
}

pub trait Pack {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError>;
}

pub trait Unpack: Sized {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError>;
}

mod sealed {
    use crate::Pack;

    pub trait Sealed {}
    impl<T: Pack + ?Sized> Sealed for T {}
}

pub trait PackExt: sealed::Sealed {
    fn packed_len(&self) -> usize;

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
