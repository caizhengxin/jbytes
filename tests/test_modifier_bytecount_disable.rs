use jbytes_derive::{ByteDecode, ByteEncode};
// use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
#[jbytes(byte_count_disable)]
pub enum Example {
    Read {
        address: u8,
    },
    Write {
        address: u8,
        value: u16,
    }
}


#[test]
fn test_modifier_byte_count_disable() {
    let value = Example::Read { address: 1 };
    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01");
}