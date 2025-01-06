use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct CountExample {
    // count - Specifies the number of Vec elements.
    // byte_count - Fetches 2 byte mapping string length in advance.
    #[jbytes(count=3, byte_count=2)]
    pub value: Vec<String>,
}


#[test]
fn test_modifier_count_example() {
    let bytes = Bytes::new([
        0x00, 0x02,             // byte_count, string_len=2,
        0x31, 0x32,             // "12"
        0x00, 0x03,             // byte_count, string_len=3,
        0x33, 0x34, 0x35,       // "345"
        0x00, 0x04,             // byte_count, string_len=4,
        0x36, 0x37, 0x38, 0x39, // "6789"
    ]);
    let value = CountExample::decode(&bytes).unwrap();
    assert_eq!(value, CountExample { value: vec!["12".to_string(), "345".to_string(), "6789".to_string()] });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), *bytes);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum CountEnumExample {
    #[jbytes(branch_value=1)]
    Read {
        // count - Specifies the number of Vec elements.
        // byte_count - Fetches 2 byte mapping string length in advance.
        #[jbytes(count=3, byte_count=2)]
        value: Vec<String>,
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_count_enum_example() {
    let bytes = Bytes::new([
        0x01,                   // branch_value
        0x00, 0x02,             // byte_count, string_len=2,
        0x31, 0x32,             // "12"
        0x00, 0x03,             // byte_count, string_len=3,
        0x33, 0x34, 0x35,       // "345"
        0x00, 0x04,             // byte_count, string_len=4,
        0x36, 0x37, 0x38, 0x39, // "6789"
    ]);
    let value = CountEnumExample::decode(&bytes).unwrap();
    assert_eq!(value, CountEnumExample::Read { value: vec!["12".to_string(), "345".to_string(), "6789".to_string()] });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), *bytes);
}