use jbytes::{
    Bytes, BufRead,
    ByteDecode
};
use jbytes_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct UntakeExample {
    #[jbytes(untake)]
    pub a: u8,
    pub b: u16,
    #[jbytes(branch="a")]
    pub c: UntakeEnumExample,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum UntakeEnumExample {
    #[jbytes(branch_value = 0)]
    Read {
        #[jbytes(untake)]
        a: u8,
        b: u16,    
    },
    #[jbytes(branch_default)]
    Unknown, 
}


#[test]
fn test_untake_example1() {
    let bytes = Bytes::new([0x00, 0x01, 0x00, 0x01]);
    let value = UntakeExample { a: 0, b: 1, c: UntakeEnumExample::Read { a: 0, b: 1 } };
    assert_eq!(UntakeExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), [0x00, 0x01, 0x00, 0x01]);
}
