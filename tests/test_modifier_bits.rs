use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct BitsExample {
    #[jbytes(bits_start=0xf0, untake)]
    pub version: u8,
    #[jbytes(bits=0x0f)]
    pub length: u8,
}


#[test]
fn test_modifier_bits_example() {
    // decode
    let data = b"\x12";
    let bytes = Bytes::new(data);
    let value = BitsExample {
        version: 0x01,
        length: 0x02,
    };
    assert_eq!(BitsExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    // encode
    assert_eq!(*jbytes::encode(value).unwrap(), data);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct BitsExample2 {
    #[jbytes(bits_start=0xf000, untake)]
    pub version: u16,
    #[jbytes(bits=0x0f00, untake)]
    pub length: u16,
    #[jbytes(bits=0x00ff)]
    pub value: u16,
}


#[test]
fn test_modifier_bits_example2() {
    // decode
    let data = b"\x12\x34";
    let bytes = Bytes::new(data);
    let value = BitsExample2 {
        version: 0x01,
        length: 0x02,
        value: 0x34,
    };
    assert_eq!(BitsExample2::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    // encode
    assert_eq!(*jbytes::encode(value).unwrap(), data);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum BitsEnumExample {
    #[jbytes(branch_value=1)]
    Read {
        #[jbytes(bits_start=0xf0, untake)]
        version: u8,
        #[jbytes(bits=0x0f)]
        length: u8,    
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_bits_enum_example() {
    // decode
    let data = b"\x01\x12";
    let bytes = Bytes::new(data);
    let value = BitsEnumExample::Read {
        version: 0x01,
        length: 0x02,
    };
    assert_eq!(BitsEnumExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    // encode
    assert_eq!(*jbytes::encode(value).unwrap(), data);
}
