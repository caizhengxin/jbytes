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