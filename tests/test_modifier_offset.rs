use jbytes::{
    Bytes, BufRead,
    ByteDecode
};
use jbytes_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct OffsetExample1 {
    #[jbytes(offset=2)]
    pub a: u8,
    pub b: u16,
}


#[test]
fn test_offset_example1() {
    let bytes = Bytes::new([0x00, 0x00, 0x01, 0x00, 0x02]);
    let value = OffsetExample1 { a: 1, b: 2};
    assert_eq!(OffsetExample1::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), [0x00, 0x00, 0x01, 0x00, 0x02]);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct OffsetExample2 {
    pub a: u8,
    #[jbytes(offset="a")]
    pub b: u16,
}


#[test]
fn test_offset_example2() {
    let bytes = Bytes::new([0x02, 0x00, 0x00, 0x00, 0x01]);
    let value = OffsetExample2 { a: 2, b: 1};
    assert_eq!(OffsetExample2::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), [0x02, 0x00, 0x00, 0x00, 0x01]);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct OffsetExample3 {
    pub a: u8,
    #[jbytes(offset="a - 1")]
    pub b: u16,
}


#[test]
fn test_offset_example3() {
    let bytes = Bytes::new([0x02, 0x00, 0x00, 0x01]);
    let value = OffsetExample3 { a: 2, b: 1};
    assert_eq!(OffsetExample3::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), [0x02, 0x00, 0x00, 0x01]);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum OffsetExample4 {
    #[jbytes(branch_value = 1)]
    Jkc {
        #[jbytes(offset=2)]
        a: u8,
        b: u16,    
    },
    #[jbytes(branch_default)]
    Unknown
}


#[test]
fn test_offset_example4() {
    let bytes = Bytes::new([0x01, 0x00, 0x00, 0x02, 0x00, 0x01]);
    let value = OffsetExample4::Jkc { a: 2, b: 1};
    assert_eq!(OffsetExample4::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), [0x01, 0x00, 0x00, 0x02, 0x00, 0x01]);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum OffsetExample5 {
    #[jbytes(branch_value = 1)]
    Jkc {
        a: u8,
        #[jbytes(offset = "a")]
        b: u16,    
    },
    #[jbytes(branch_default)]
    Unknown
}


#[test]
fn test_offset_example5() {
    let bytes = Bytes::new([0x01, 0x02, 0x00, 0x00, 0x00, 0x01]);
    let value = OffsetExample5::Jkc { a: 2, b: 1};
    assert_eq!(OffsetExample5::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), [0x01, 0x02, 0x00, 0x00, 0x00, 0x01]);
}
