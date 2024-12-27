use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct ValueExample {
    #[jbytes(value_decode="length * 2", value_encode="length / 2")]
    pub length: u8,
}


#[test]
fn test_monidifer_value() {
    // decode
    let bytes = Bytes::new(b"\x01");
    let value = ValueExample::decode(&bytes).unwrap();
    assert_eq!(value, ValueExample { length: 2 });

    // encode
    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01");
}
