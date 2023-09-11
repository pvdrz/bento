mod util;

tests! {
    (u32, u16): [(0xdeadbeef, 0xf00d), duple_consistency, duple_packed_len],
    (u16, u16, u32): [(0xcafe, 0xf00d, 0xdeadbeef), truple_consistency, truple_packed_len],
}
