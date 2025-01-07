
use jbytes_derive::{ByteEncode, ByteDecode};
use jdefault_derive::Jdefault;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode, Jdefault)]
pub struct SimpleExample {
    #[jbytes(byte_count=1, default="\"123\".to_string()")]
    pub value: String,
    #[jbytes(byte_count=1)]
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode, Jdefault)]
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
    Unknown {
        #[jbytes(default=10)]
        value: u8,
    },
}


#[test]
fn test_default_example() {
    let value = SimpleExample::default();
    assert_eq!(value, SimpleExample {
        value: "123".to_string(),
        body: SimpleExampleBody::Unknown { value: 10 },
    });

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x03\x31\x32\x33\x03\x0a");

    let value: SimpleExample = jbytes::decode(b"\x03\x31\x32\x33\x03\x0a").unwrap();
    assert_eq!(value, SimpleExample {
        value: "123".to_string(),
        body: SimpleExampleBody::Unknown { value: 10 },
    });
}