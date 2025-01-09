use jbytes::prelude::*;


fn main() {
    let bytes = Bytes::new(b"\x01\x02\x03");
    assert_eq!(bytes.take_be_u16(), Ok(0x0102));
    assert_eq!(bytes.take_be_u16().is_err(), true);
}