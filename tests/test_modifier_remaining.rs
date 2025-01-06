use jbytes_derive::{BorrowByteDecode, BorrowByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct RemainingExample<'a> {
    pub command: u16,
    #[jbytes(remaining)]
    pub data: &'a [u8]
}


#[test]
fn test_modifier_remaining() {
    let data = b"\x00\x01\x31\x32\x33\x34\x35";
    let bytes = Bytes::new(data);
    let value = RemainingExample { command: 1, data: b"12345" };
    assert_eq!(RemainingExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode_borrow(value).unwrap(), b"\x00\x01\x31\x32\x33\x34\x35");
}
