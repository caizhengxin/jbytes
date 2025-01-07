
use jbytes_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct SimpleExample {
    pub length: u16,
    #[jbytes(length="length")]
    pub value: String,
    pub cmd: u8,
    #[jbytes(branch="cmd")]
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
    let input = b"\x00\x03\x31\x32\x33\x01\x05";
    let value: SimpleExample = jbytes::decode(input).unwrap();
    assert_eq!(value, SimpleExample { length: 3, value: "123".to_string(), cmd: 1, body: SimpleExampleBody::Read { address: 5 } });
    assert_eq!(*jbytes::encode(value).unwrap(), input);
}