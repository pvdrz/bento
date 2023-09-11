#![allow(overflowing_literals)]

mod util;

macro_rules! num_tests {
    ($($num:ty : [$consistency:ident, $packed_len:ident],)*) => {
        tests! {
            $($num: [0xdeadbeef as $num, $consistency, $packed_len],)*
        }
    }
}

num_tests! {
    u16: [ u16_consistency, u16_packed_len ],
    i16: [ i16_consistency, i16_packed_len ],
    u32: [ u32_consistency, u32_packed_len ],
    i32: [ i32_consistency, i32_packed_len ],
    u64: [ u64_consistency, u64_packed_len ],
    i64: [ i64_consistency, i64_packed_len ],
    u128: [ u128_consistency, u128_packed_len ],
    i128: [ i128_consistency, i128_packed_len ],
    usize: [ usize_consistency, usize_packed_len ],
    isize: [ isize_consistency, isize_packed_len ],
    f32: [ f32_consistency, f32_packed_len ],
    f64: [ f64_consistency, f64_packed_len ],
}
