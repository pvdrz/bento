#[macro_export]
macro_rules! tests {
    ($($ty:ty: [ $value:expr, $consistency:ident, $packed_len:ident],)*) => {
        use knapsack::{PackExt, Unpack};

        $(
            #[test]
            fn $consistency() {
                let data: $ty = $value;

                let bytes = data.pack_to_vec();

                let unpacked = <$ty>::unpack(&mut bytes.as_slice()).unwrap();

                assert_eq!(data, unpacked);
            }

            #[test]
            fn $packed_len() {
                let data: $ty = $value;

                let bytes = data.pack_to_vec();

                assert_eq!(bytes.len(), data.packed_len());
            }
        )*
    }
}
