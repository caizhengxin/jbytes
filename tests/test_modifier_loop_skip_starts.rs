use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct SkipStartsExample {
    pub command: u8,
    #[jbytes(loop_skip_starts=b"\xff")]
    pub data: Vec<u8>,
    pub flag: u8,
}


#[test]
fn test_modifier_loop_skip_starts_example() {
    let bytes = Bytes::new(b"\x01\xff\x00\xff\x01\xff\x02\x05");
    let value = SkipStartsExample {
        command: 1,
        data: vec![0, 1, 2],
        flag: 5,
    };
    assert_eq!(SkipStartsExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01\xff\x00\xff\x01\xff\x02\x05");    
}


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub enum SkipStartsExample2 {
    #[jbytes(branch_value=1)]
    Read {
        command: u8,
        #[jbytes(loop_skip_starts=b"\xff")]
        data: Vec<u8>,
        flag: u8,    
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_loop_skip_starts_example2() {
    let bytes = Bytes::new(b"\x01\x01\xff\x00\xff\x01\xff\x02\x05");
    let value = SkipStartsExample2::Read {
        command: 1,
        data: vec![0, 1, 2],
        flag: 5,
    };
    assert_eq!(SkipStartsExample2::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01\x01\xff\x00\xff\x01\xff\x02\x05");    
}


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub enum SkipStartsExample3 {
    #[jbytes(branch_value=1)]
    Read(#[jbytes(loop_skip_starts=b"\xff")] Vec<u8>),
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_loop_skip_starts_example3() {
    let bytes = Bytes::new(b"\x01\xff\x00\xff\x01\xff\x02");
    let value = SkipStartsExample3::Read(vec![0, 1, 2]);
    assert_eq!(SkipStartsExample3::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01\xff\x00\xff\x01\xff\x02");    
}
