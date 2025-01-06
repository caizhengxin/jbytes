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


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum ByteCountEnumExample {
    #[jbytes(branch_value=1)]
    Read {
        #[jbytes(byte_count=2)]  // Fetches 2 byte mapping length in advance.
        value: String,    
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_byte_count_enum_example() {
    let bytes = Bytes::new(b"\x01\x00\x03\x31\x32\x33");
    let value = ByteCountEnumExample::decode(&bytes).unwrap();
    assert_eq!(value, ByteCountEnumExample::Read { value: "123".to_string() });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01\x00\x03\x31\x32\x33");
}