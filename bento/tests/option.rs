mod util;

tests! {
    Option<u32>: [Some(0xdeadbeef), some_consistency, some_packed_len],
    Option<u32>: [None, none_consistency, none_packed_len],
}
