#[allow(unused_imports)]
use jbytes::{ByteDecode, ByteEncode};
use jbytes_derive::{ByteDecode, ByteEncode};
// use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct Example {
    pub v1: u8,
    pub v2: u16,
    pub v3: u32,
    pub v4: u64,
    pub v5: u128,
    #[jbytes(length = 3)]
    pub v6: u32,
    #[jbytes(length = "v2")]
    pub v7: usize,
    #[jbytes(byteorder = "LE")]
    pub v8: u16,
}


#[test]
fn test_type_int() {
    let input = [
        0x01,
        0x00, 0x02,
        0x00, 0x00, 0x00, 0x03,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x06,
        0x00, 0x07,
        0x08, 0x00,
    ];
    let mt_value = Example {
        v1: 0x01,
        v2: 0x0002,
        v3: 0x00000003,
        v4: 0x0000000000000004,
        v5: 0x00000000000000000000000000000005,
        v6: 0x000006,
        v7: 0x0007,
        v8: 0x0008,
    };

    let value: Example = jbytes::decode(&input).unwrap();
    assert_eq!(value, mt_value);
    assert_eq!(*jbytes::encode(value).unwrap(), input);
}


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
#[jbytes(byteorder = "LE")]
pub struct LittleEndianExample {
    pub v1: u8,
    pub v2: u16,
    pub v3: u32,
    pub v4: u64,
    pub v5: u128,
    #[jbytes(length = 3)]
    pub v6: u32,
    #[jbytes(length = "v2")]
    pub v7: usize,
    #[jbytes(byteorder = "BE")]
    pub v8: u16,
}


#[test]
fn test_type_int_with_little_endian() {
    let input = [
        0x01,
        0x02, 0x00,
        0x03, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x00,
        0x07, 0x00,
        0x00, 0x08,
    ];
    let mt_value = LittleEndianExample {
        v1: 0x01,
        v2: 0x0002,
        v3: 0x00000003,
        v4: 0x0000000000000004,
        v5: 0x00000000000000000000000000000005,
        v6: 0x000006,
        v7: 0x0007,
        v8: 0x0008,
    };

    let value: LittleEndianExample = jbytes::decode(&input).unwrap();
    assert_eq!(value, mt_value);
    assert_eq!(*jbytes::encode(value).unwrap(), input);
}
