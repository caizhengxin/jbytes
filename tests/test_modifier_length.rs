use jbytes::{
    Bytes, BufRead,
    BorrowByteDecode
};
use jbytes_derive::{BorrowByteDecode, BorrowByteEncode};


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct LengthExample1<'a> {
    #[jbytes(length=3)]
    pub a: u32,
    #[jbytes(length=1)]
    pub b: &'a [u8],
    #[jbytes(count=1, length=3)]  // count indicates the Vec size and length indicates bytes of the int type.
    pub c: Vec<u32>,
    #[jbytes(length=3)]
    pub d: &'a str,
    #[jbytes(length=4)]
    pub e: String,
}


#[test]
fn test_length_example1() {
    let data = [
        0x00, 0x00, 0x01,
        0xff,
        0x00, 0x00, 0x02,
        0x61, 0x62, 0x63,
        0x61, 0x62, 0x63, 0x64
    ];
    let bytes = Bytes::new(data);
    let value = LengthExample1 { a: 0x000001, b: &[0xff], c: vec![0x000002], d: "abc", e: "abcd".to_string()};
    assert_eq!(LengthExample1::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode_borrow(value).unwrap(), data);
}


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct LengthExample2<'a> {
    pub a: u8,
    #[jbytes(length="a")]
    pub b: &'a str,
    #[jbytes(length="a - 1")]
    pub c: &'a str,
}


#[test]
fn test_length_example2() {
    let data = [
        0x03,
        0x61, 0x62, 0x63,
        0x61, 0x62,
    ];
    let bytes = Bytes::new(data);
    let value = LengthExample2 { a: 0x03, b: "abc", c: "ab"};
    assert_eq!(LengthExample2::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode_borrow(value).unwrap(), data);
}


#[derive(Debug, Default, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub enum LengthExample3<'a> {
    #[jbytes(branch_value = 1)]
    Jkc {
        a: u8,
        #[jbytes(length="a")]
        b: &'a str,
        #[jbytes(length="a - 1")]
        c: &'a str,    
    },
    #[jbytes(branch_default)]
    #[default]
    Unknown
}


#[test]
fn test_length_example3() {
    let data = [
        0x01, // enum branch
        0x03,
        0x61, 0x62, 0x63,
        0x61, 0x62,
    ];
    let bytes = Bytes::new(data);
    let value = LengthExample3::Jkc { a: 0x03, b: "abc", c: "ab"};
    assert_eq!(LengthExample3::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode_borrow(value).unwrap(), data);
}