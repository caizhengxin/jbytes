use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;

// support String/&str/&[u8] type.


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct KeyExample {
    #[jbytes(key=b"Version: ", linend=b"\r\n")]
    pub version: String,
    #[jbytes(key=b"Host: ", linend=b"\r\n")]
    pub host: String,
}


#[test]
fn test_modifier_key_example() {
    // decode
    let data = b"Cookie: sssss\r\nVersion: 1.0.0\r\nHeader: jkc\r\nHost: 192.168.1.1\r\nOther: jkc\r\n";
    let bytes = Bytes::new(data);
    let value = KeyExample { version: "1.0.0".to_string(), host: "192.168.1.1".to_string() };
    assert_eq!(KeyExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining(), b"Other: jkc\r\n");

    // encode
    assert_eq!(*jbytes::encode(value).unwrap(), b"Version: 1.0.0\r\nHost: 192.168.1.1\r\n");
}
