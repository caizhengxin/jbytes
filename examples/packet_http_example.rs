
use std::collections::HashMap;
use jbytes::{BorrowByteDecode, BorrowByteEncode};
use jbytes_derive::{BorrowByteEncode, BorrowByteDecode};
use jbytes::prelude::*;


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Http<'a> {
    #[jbytes(linend=b"\x20")]
    pub method: &'a str,
    #[jbytes(linend=b"\x20")]
    pub uri: &'a str,
    #[jbytes(linend=b"\r\n")]
    pub http: &'a str,
    #[jbytes(split=b": ", linend=b"\r\n", try_count=10)]
    pub headers: HashMap<&'a str, &'a str>,
}


fn main() {
    let data = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    let bytes = Bytes::new(data);

    let value = Http::decode(&bytes).unwrap();
    println!("{value:?}");

    // encode
    let mut buffer = Buffer::new();
    let _ = value.encode(&mut buffer);
    println!("{buffer:?}");
    // The headers hashmap is out of order and cannot be compared.
    // assert_eq!(*buffer, data);
}