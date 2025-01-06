#[cfg(not(feature = "std"))]
extern crate alloc;
use jkcenum::JkcEnum;
use jbytes_derive::{BorrowByteEncode, BorrowByteDecode};
use jbytes::prelude::*;
use jbytes::std::*;


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Http<'a> {
    #[jbytes(linend=b"\x20", from_str)]
    pub method: HttpMethodEnum,
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


#[derive(Debug, Default, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode, JkcEnum)]
#[jbytes(byte_count_disable)]
pub enum HttpMethodEnum {
    #[default]
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}


fn main() {
    // decode
    let data = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    let bytes = Bytes::new(data);
    let value: Http = jbytes::decode_borrow(&bytes).unwrap();
    println!("{value:?}");

    // encode
    assert_eq!(*jbytes::encode_borrow(value).unwrap(), data);

    // error
    let data: &[u8; 97] = b"SET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    let bytes = Bytes::new(data);
    let value: JResult<Http> = jbytes::decode_borrow(&bytes);
    assert_eq!(value.is_err(), true);
}