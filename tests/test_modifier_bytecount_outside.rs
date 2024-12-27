use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct ByteCountExample {
    #[jbytes(byte_count_outside=2, byte_count=2)]  // Fetches 2 byte mapping length in advance.
    pub value: Vec<String>,
}


#[test]
fn test_modifier_byte_count_example() {
    let bytes = Bytes::new([
        0x00, 0x03,             // byte_count_outside, vec_len=3
        0x00, 0x02,             // byte_count, string_len=2,
        0x31, 0x32,             // "12"
        0x00, 0x03,             // byte_count, string_len=3,
        0x33, 0x34, 0x35,       // "345"
        0x00, 0x04,             // byte_count, string_len=4,
        0x36, 0x37, 0x38, 0x39, // "6789"
    ]);
    let value = ByteCountExample::decode(&bytes).unwrap();
    assert_eq!(value, ByteCountExample { value: vec!["12".to_string(), "345".to_string(), "6789".to_string()] });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), *bytes);
}