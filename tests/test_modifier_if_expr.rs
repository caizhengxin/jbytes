use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct OptionExample {
    pub flags: bool,
    #[jbytes(if_expr="flags")]
    pub value: Option<u16>,
}


#[test]
fn test_modifier_if_expr() {
    let bytes = Bytes::new(b"\x00");
    let value = OptionExample::decode(&bytes).unwrap();
    assert_eq!(value, OptionExample { flags: false, value: None });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x00");

    let bytes = Bytes::new(b"\x01\x00\x31");
    let value = OptionExample::decode(&bytes).unwrap();
    assert_eq!(value, OptionExample { flags: true, value: Some(0x31) });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01\x00\x31");
}


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub enum OptionEnumExample {
    #[jbytes(branch_value=1)]
    Read {
        flags: bool,
        #[jbytes(if_expr="flags")]
        value: Option<u16>,    
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_if_expr_enum_example() {
    let bytes = Bytes::new(b"\x01\x00");
    let value = OptionEnumExample::decode(&bytes).unwrap();
    assert_eq!(value, OptionEnumExample::Read { flags: false, value: None });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01\x00");

    let bytes = Bytes::new(b"\x01\x01\x00\x31");
    let value = OptionEnumExample::decode(&bytes).unwrap();
    assert_eq!(value, OptionEnumExample::Read { flags: true, value: Some(0x31) });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01\x01\x00\x31");
}