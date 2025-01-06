use jbytes_derive::{BorrowByteDecode, BorrowByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct RemainingExample<'a> {
    pub command: u16,
    #[jbytes(remaining)]
    pub data: &'a [u8]
}


#[test]
fn test_modifier_remaining_example() {
    let data = b"\x00\x01\x31\x32\x33\x34\x35";
    let bytes = Bytes::new(data);
    let value = RemainingExample { command: 1, data: b"12345" };
    assert_eq!(RemainingExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode_borrow(value).unwrap(), b"\x00\x01\x31\x32\x33\x34\x35");
}


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub enum RemainingEnumExample<'a> {
    #[jbytes(branch_value=1)]
    Read {
        command: u16,
        #[jbytes(remaining)]
        data: &'a [u8]    
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_remaining_enum_example() {
    let data = b"\x01\x00\x01\x31\x32\x33\x34\x35";
    let bytes = Bytes::new(data);
    let value = RemainingEnumExample::Read { command: 1, data: b"12345" };
    assert_eq!(RemainingEnumExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode_borrow(value).unwrap(), b"\x01\x00\x01\x31\x32\x33\x34\x35");
}
