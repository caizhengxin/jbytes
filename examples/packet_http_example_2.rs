
use jbytes_derive::{BorrowByteEncode, BorrowByteDecode};
use jbytes::prelude::*;


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Http<'a> {
    #[jbytes(linend=b"\x20")]
    pub method: &'a str,
    #[jbytes(linend=b"\x20")]
    pub uri: &'a str,
    #[jbytes(linend=b"\r\n")]
    pub version: &'a str,
    #[jbytes(try_count=20)]
    pub headers: Vec<HttpHeader<'a>>,
}


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct HttpHeader<'a> {
    #[jbytes(linend=b": ")]
    pub key: &'a str,
    #[jbytes(linend=b"\r\n")]
    pub value: &'a str,
}


fn main() {
    // decode
    let data = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    let bytes = Bytes::new(data);
    let value: Http = jbytes::decode_borrow(&bytes).unwrap();
    println!("{value:?}");

    // encode
    assert_eq!(*jbytes::encode_borrow(value).unwrap(), data);
}