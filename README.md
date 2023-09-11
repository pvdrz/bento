A simple binary serialization and deserialization framework.

This crate provides the [`Pack`] and [`Unpack`] traits for types that can be serialized and
deserialized respectively. These two traits are implemented for some of the standard library
types.

Bentō also provides the [`PackBytes`] and [`UnpackBytes`] traits for types that can can be
used to read and write bytes respectively.

# Example
```rust
use bento::{PackExt, Unpack};

// Some data we want to serialize.
let data = vec!["hello", "world"];

// Serialize the data and write it into a buffer.
let buffer: Vec<u8> = data.pack_to_vec();

// Deserialize the data by reading it from the buffer.
let unpacked_data = Vec::<String>::unpack(buffer.as_slice()).unwrap();

// Check that we deserialized the same data that we had before.
assert_eq!(data.len(), unpacked_data.len());
assert_eq!(unpacked_data[0], "hello");
assert_eq!(unpacked_data[1], "world");
```
