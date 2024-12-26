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
