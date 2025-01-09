
#[cfg(feature = "std")]
use std::collections::HashMap;
#[allow(unused_imports)]
use jbytes::{BorrowByteDecode, BorrowByteEncode};
#[allow(unused_imports)]
use jbytes_derive::{BorrowByteEncode, BorrowByteDecode};
#[allow(unused_imports)]
use jbytes::prelude::*;


#[cfg(feature = "std")]
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


#[cfg(feature = "std")]
fn main() -> JResult<()> {
    let data = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
    let bytes = Bytes::new(data);

    let value = Http::decode(&bytes)?;
    println!("{value:?}");

    // encode
    let mut buffer = Buffer::new();
    let _ = value.encode(&mut buffer);
    println!("{buffer:?}");
    // The headers hashmap is out of order and cannot be compared.
    // assert_eq!(*buffer, data);

    Ok(())
}

#[cfg(not(feature = "std"))]
fn main() {
}
