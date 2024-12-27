use jbytes::{
    Bytes, BufRead,
    ByteDecode
};
use jbytes_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
// default: big-endian byte order.
pub struct ByteOrderExample1 {
    pub a: u16,
    #[jbytes(byteorder = "LE")] // set little-endian byte order.
    pub b: u16,
}


#[test]
fn test_byteorder_example1() {
    let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
    let value = ByteOrderExample1 { a: 0x0001, b: 0x0200};
    assert_eq!(ByteOrderExample1::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), [0x00, 0x01, 0x00, 0x02]);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[jbytes(byteorder = "LE")]
pub struct ByteOrderExample2 {
    pub a: u16,
    pub b: u16,
}


#[test]
fn test_byteorder_example2() {
    let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
    let value = ByteOrderExample2 { a: 0x0100, b: 0x0200};
    assert_eq!(ByteOrderExample2::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), [0x00, 0x01, 0x00, 0x02]);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
// default: big-endian byte order.
pub struct ByteOrderExample3 {
    pub a: u8,
    #[jbytes(byteorder = "a")] // BE=0, LE=1
    pub b: u16,
}


#[test]
fn test_byteorder_example3() {
    let bytes = Bytes::new([0x01, 0x00, 0x02]);
    let value = ByteOrderExample3 { a: 0x01, b: 0x0200};
    assert_eq!(ByteOrderExample3::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), [0x01, 0x00, 0x02]);
}