
use std::str::FromStr;
use jbytes::{ByteDecode, ByteEncode};
use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct Ethernet {
    pub smac: MacAddress,
    pub dmac: MacAddress,
    pub r#type: u16,
}


fn main() -> JResult<()> {
    let data = b"\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x08\x00\x45\x00";
    let bytes = Bytes::new(data);

    // decode
    let value = Ethernet::decode(&bytes)?;
    assert_eq!(value, Ethernet {
        smac: MacAddress::from_str("ff:ff:ff:ff:ff:ff").unwrap(),
        dmac: MacAddress::from_str("00:00:00:00:00:00").unwrap(),
        r#type: 0x0800,
    });
    assert_eq!(bytes.remaining_len(), 2);

    // encode
    let mut buffer = Buffer::new();

    let _ = value.encode(&mut buffer);
    assert_eq!(*buffer, b"\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x08\x00");

    Ok(())
}
