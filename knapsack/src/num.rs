use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

macro_rules! impl_pack_and_unpack_for_nums {
    ($($num:ty),*) => {
        $(
            impl Pack for $num {
                fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
                    self.to_le_bytes().pack(packer)
                }
            }

            impl Unpack for $num {
                fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
                    let bytes = <[u8; core::mem::size_of::<Self>()]>::unpack(unpacker)?;
                    Ok(Self::from_le_bytes(bytes))
                }
            }
         )*
    };
}

impl_pack_and_unpack_for_nums! {u16, i16, u32, i32, u64, i64, u128, i128, f32, f64}

impl Pack for usize {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        (*self as u64).pack(packer)
    }
}

impl Unpack for usize {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        Ok(u64::unpack(unpacker)? as usize)
    }
}

impl Pack for isize {
    fn pack<P: PackBytes>(&self, packer: P) -> Result<(), PackError> {
        (*self as i64).pack(packer)
    }
}

impl Unpack for isize {
    fn unpack<U: UnpackBytes>(unpacker: U) -> Result<Self, UnpackError> {
        Ok(i64::unpack(unpacker)? as isize)
    }
}

/// Check that `usize` values fit in 64 bits.
const _USIZE_CHECK: () = if core::mem::size_of::<usize>() > core::mem::size_of::<u64>() {
    panic!("knapsack doesn't support platforms where `usize` is larger than `u64");
};
