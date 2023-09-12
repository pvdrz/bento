use core::mem::MaybeUninit;

use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

impl<const N: usize> Pack for [u8; N] {
    fn pack<P: PackBytes>(&self, mut packer: P) -> Result<(), PackError> {
        packer.pack_bytes(self)
    }
}

impl<const N: usize> Unpack for [u8; N] {
    fn unpack<U: UnpackBytes>(mut unpacker: U) -> Result<Self, UnpackError> {
        let mut arr = [0u8; N];

        unpacker.unpack_bytes(&mut arr)?;

        Ok(arr)
    }
}

impl<T: Pack, const N: usize> Pack for [T; N] {
    fn pack<P: PackBytes>(&self, mut packer: P) -> Result<(), PackError> {
        for item in self {
            item.pack(&mut packer)?;
        }

        Ok(())
    }
}

impl<T: Unpack, const N: usize> Unpack for [T; N] {
    fn unpack<U: UnpackBytes>(mut unpacker: U) -> Result<Self, UnpackError> {
        // FIXME: Change this by `MaybeUninit::array_uninit` when stabilized.
        // SAFETY: An uninitialized `[MaybeUninit<_>; N]` is valid.
        let mut arr = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };

        for item in &mut arr {
            let value = T::unpack(&mut unpacker)?;
            *item = MaybeUninit::new(value);
        }

        // FIXME: Change this by `MaybeUninit::array_assume_init` when stabilized.
        // SAFETY: `[MaybeUninit<T>; N]` and `[T; N]` have the same layout and the whole array has
        // been initialized already.
        Ok(unsafe { core::mem::transmute_copy(&arr) })
    }
}
