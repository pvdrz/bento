mod util;

tests! {
    [u8; 16]: [*b"yellow submarine", u8_array_consistency, u8_array_packed_len],
    [u32; 2]: [[0xdeadbeef, 0xbeefdead], u32_array_consistency, u32_array_packed_len],
}
