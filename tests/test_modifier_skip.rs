use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct SkipExample {
    #[jbytes(skip)]
    pub version: u16,
    pub command: u16,
}


#[test]
fn test_modifier_skip() {
    let bytes = Bytes::new(b"\x00\x01");
    assert_eq!(SkipExample::decode(&bytes).unwrap(), SkipExample { version: 0, command: 1 });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(SkipExample { version: 0, command: 1 }).unwrap(), b"\x00\x01");
}


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct SkipExample2 {
    #[jbytes(skip_encode, skip_decode)]
    pub version: u16,
    pub command: u16,
}


#[test]
fn test_modifier_skip_example2() {
    let bytes = Bytes::new(b"\x00\x01");
    assert_eq!(SkipExample2::decode(&bytes).unwrap(), SkipExample2 { version: 0, command: 1 });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(SkipExample2 { version: 0, command: 1 }).unwrap(), b"\x00\x01");
}