use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct ByteCountExample {
    #[jbytes(byte_count=2)]  // Fetches 2 byte mapping length in advance.
    pub value: String,
}


#[test]
fn test_modifier_byte_count_example() {
    let bytes = Bytes::new(b"\x00\x03\x31\x32\x33");
    let value = ByteCountExample::decode(&bytes).unwrap();
    assert_eq!(value, ByteCountExample { value: "123".to_string() });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x00\x03\x31\x32\x33");
}