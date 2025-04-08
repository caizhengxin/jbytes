use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u16)]
pub enum EnumReprExample {
    Read {
        address: u8,
    },                    // default index=0.
    Write {
        address: u8,
        value: u16,
    },                    // default index=1.
}


#[test]
fn test_enum_repr_example() {
    let data = b"\x00\x00\x01";
    let bytes = Bytes::new(data);
    let value = EnumReprExample::Read { address: 1 };
    assert_eq!(EnumReprExample::decode(&bytes).unwrap(), value);
    assert_eq!(*jbytes::encode(value).unwrap(), b"\x00\x00\x01");
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum EnumReprExample2 {
    Read {
        address: u8,
    } = 1,                    // set index = 1.
    Write {
        address: u8,
        value: u16,
    },                       // increment index = 2.
}


#[test]
fn test_enum_repr_example2() {
    let data = b"\x01\x01";
    let bytes = Bytes::new(data);
    let value = EnumReprExample2::Read { address: 1 };
    assert_eq!(EnumReprExample2::decode(&bytes).unwrap(), value);
    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01\x01");
}


#[derive(Debug, Default, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum EnumReprExample3 {
    Read {
        address: u8,
    } = 1,                    // set index = 1.
    Write {
        address: u8,
        value: u16,
    },                        // increment index = 2.
    #[default]
    Unknown,                  // index >= 3.
}


#[test]
fn test_enum_repr_example3() {
    let data = b"\x05";
    let bytes = Bytes::new(data);
    let value = EnumReprExample3::Unknown;
    assert_eq!(EnumReprExample3::decode(&bytes).unwrap(), value);
}