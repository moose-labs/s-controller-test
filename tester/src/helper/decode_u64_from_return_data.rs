use base64::{self, engine::general_purpose, Engine};
use std::convert::TryInto;

pub fn decode_u64_from_return_data(encoded: &str) -> u64 {
    // Decode the Base64 string into bytes.
    let decoded = general_purpose::STANDARD
        .decode(encoded)
        .expect("failed to decode base64");

    // If the return data is exactly 8 bytes, we can take those 8 bytes.
    // (If there is extra data, you may need to adjust the offset.)
    let bytes: [u8; 8] = decoded[..8]
        .try_into()
        .expect("slice with incorrect length");

    // Convert the little-endian bytes into a u64.
    u64::from_le_bytes(bytes)
}

#[test]
fn test_decode_u64_from_return_data() {
    // This is the string printed in the transaction log.
    let encoded = "hW/KLgAAAACJb8ouAAAAAA==";
    let value = decode_u64_from_return_data(encoded);
    println!("Decoded u64: {}", value);
    assert_eq!(value, 785018757)
}
