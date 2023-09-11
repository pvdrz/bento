use crate::{Pack, PackBytes, PackError, Unpack, UnpackBytes, UnpackError};

macro_rules! impl_for_tuple {
    ($($ty:ident: $field:ident,)*) => {
         impl<$($ty: Pack),*> Pack for ($($ty,)*) {
            fn pack<P: PackBytes>(&self, mut packer: P) -> Result<(), PackError> {
                let ($($field,)*) = self;
                $($field.pack(&mut packer)?;)*

                Ok(())
            }
         }

         impl<$($ty: Unpack),*> Unpack for ($($ty,)*) {
            fn unpack<U: UnpackBytes>(mut unpacker: U) -> Result<Self, UnpackError> {
                Ok(($($ty::unpack(&mut unpacker)?,)*))
            }
         }
    };
}

macro_rules! impl_for_tuples {
    ($ty:ident: $field:tt, $($tokens:tt)*) => {
        impl_for_tuple!($ty: $field, $($tokens)*);
        impl_for_tuples!($($tokens)*);
    };
    () => {};
}

impl_for_tuples! {
    A: a,
    B: b,
    C: c,
    D: d,
    E: e,
    F: f,
    G: g,
    H: h,
    I: i,
    J: j,
    K: k,
    L: l,
}
