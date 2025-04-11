use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[jbytes(branch_take_bytes=3)]
pub enum EnumExample {
    #[jbytes(branch_value=b"get")]
    Read {
        address: u8,
    },
    #[jbytes(branch_value=b"set")]
    Write {
        address: u8,
        value: u16,
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_enum_example() {
    // get
    let data = b"get\x01";
    let bytes = Bytes::new(data);
    let value = EnumExample::Read { address: 1 };
    assert_eq!(EnumExample::decode(&bytes).unwrap(), value);
    assert_eq!(*jbytes::encode(value).unwrap(), b"get\x01");

    // set
    let data = b"set\x01\x00\x02";
    let bytes = Bytes::new(data);
    let value = EnumExample::Write { address: 1, value: 2 };
    assert_eq!(EnumExample::decode(&bytes).unwrap(), value);
    assert_eq!(*jbytes::encode(value).unwrap(), b"set\x01\x00\x02");

    // Unknown
    let data = b"put";
    let bytes = Bytes::new(data);
    let value = EnumExample::Unknown;
    assert_eq!(EnumExample::decode(&bytes).unwrap(), value);
    assert_eq!(*jbytes::encode(value).unwrap(), b"unknown");
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[jbytes(branch_starts_with)]
pub enum EnumExample2 {
    #[jbytes(branch_value=b"read")]
    Read {
        address: u8,
    },
    // #[jbytes(branch_value=b"write")]
    Write {
        address: u8,
        value: u16,
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_enum_example2() {
    // read
    let data = b"read\x01";
    let bytes = Bytes::new(data);
    let value = EnumExample2::Read { address: 1 };
    assert_eq!(EnumExample2::decode(&bytes).unwrap(), value);
    assert_eq!(*jbytes::encode(value).unwrap(), b"read\x01");

    // write
    let data = b"write\x01\x00\x02";
    let bytes = Bytes::new(data);
    let value = EnumExample2::Write { address: 1, value: 2 };
    assert_eq!(EnumExample2::decode(&bytes).unwrap(), value);
    assert_eq!(*jbytes::encode(value).unwrap(), b"write\x01\x00\x02");

    // Unknown
    let data = b"put";
    let bytes = Bytes::new(data);
    let value = EnumExample2::Unknown;
    assert_eq!(EnumExample2::decode(&bytes).unwrap(), value);
    assert_eq!(*jbytes::encode(value).unwrap(), b"unknown");
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct EnumExample3 {
    pub version: u8,
    pub value: EnumExample2,
}


#[test]
fn test_enum_example3() {
    // read
    let data = b"\x01read\x01";
    let bytes = Bytes::new(data);
    let value = EnumExample3 {
        version: 1,
        value: EnumExample2::Read { address: 1 },
    };
    assert_eq!(EnumExample3::decode(&bytes).unwrap(), value);
    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01read\x01");
}
