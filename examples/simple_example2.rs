
use jbytes_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct SimpleExample {
    pub version: u8,
    pub value: String,
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum SimpleExampleBody {
    #[jbytes(branch_value=1)]
    Read {
        address: u8,
    },
    Write {
        address: u8,
        value: [u8; 3],
    },
    #[jbytes(branch_default)]
    Unknown, 
}


fn main() {
    let input = b"\x01\x03\x31\x32\x33\x01\x05";
    let value: SimpleExample = jbytes::decode(input).unwrap();
    assert_eq!(value, SimpleExample { version: 1, value: "123".to_string(), body: SimpleExampleBody::Read { address: 5 } });
    assert_eq!(*jbytes::encode(value).unwrap(), input);
}