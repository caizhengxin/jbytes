use jdefault_derive::Jdefault;
use jbytes::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode, Jdefault)]
pub struct jbytesExample {
    #[jbytes(default=18)]
    pub value: u16,
}


#[test]
fn test_jbytes_default() {
    let value = jbytesExample::default();

    assert_eq!(value.value, 18);
}