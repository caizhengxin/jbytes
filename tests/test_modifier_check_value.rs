use jbytes::{
    Bytes, BufRead,
    ByteDecode,
    ErrorKind, make_error,
};
use jbytes_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct CheckValueExample {
    #[jbytes(check_value=2)]
    pub a: u16,
    #[jbytes(byteorder = "LE")] // set little-endian byte order.
    pub b: u16,
}


#[test]
fn test_modifier_check_value_example() {
    let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
    assert_eq!(CheckValueExample::decode(&bytes), Err(make_error(2, ErrorKind::InvalidValue("1".to_string()))));
    assert_eq!(bytes.remaining_len(), 2);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum CheckValueEnumExample {
    #[jbytes(branch_value=1)]
    Read {
        #[jbytes(check_value=2)]
        a: u16,
        #[jbytes(byteorder = "LE")] // set little-endian byte order.
        b: u16,    
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_check_value_enum_example() {
    let bytes = Bytes::new([0x01, 0x00, 0x01, 0x00, 0x02]);
    assert_eq!(CheckValueEnumExample::decode(&bytes), Err(make_error(3, ErrorKind::InvalidValue("1".to_string()))));
    assert_eq!(bytes.remaining_len(), 2);
}