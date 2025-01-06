use jbytes::prelude::*;


fn main() {
    let mut buffer = Buffer::new();
    assert_eq!(buffer.push_be_u16(1).unwrap(), 2);
    assert_eq!(buffer.push(b"\x01\x02\x03").unwrap(), 3);
    assert_eq!(*buffer, b"\x00\x01\x01\x02\x03");
}