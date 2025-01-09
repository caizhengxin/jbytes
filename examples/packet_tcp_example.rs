use jbytes::{BorrowByteDecode, BorrowByteEncode};
use jbytes_derive::{BorrowByteDecode, BorrowByteEncode};
use jbytes::prelude::*;


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Tcp<'a> {
    pub sport: u16,
    pub dport: u16,
    pub seq: u32,
    pub ack: u32,
    #[jbytes(bits_start=0xf000, value_decode="header_length * 4", value_encode="header_length / 4", untake)]
    pub header_length: u16,
    #[jbytes(bits=0x0fff)]
    pub flags: u16,
    pub window: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    #[jbytes(length="header_length - 20")]
    pub options: &'a [u8],
}


fn main() {
    let data = b"\xc8\xd3\x01\xf6\xe0\x76\x90\x16\xc4\x44\x9b\x5a\x80\x18\xff\xff\
    \x6c\x1c\x00\x00\x01\x01\x08\x0a\x37\xc4\x50\xe2\x00\xba\x7c\x1c";    
    let bytes = Bytes::new(data);

    // decode
    if let Ok(value) = Tcp::decode(&bytes) {
        assert_eq!(value, Tcp {
            sport: 51411,
            dport: 502,
            seq: 3765866518,
            ack: 3292830554,
            header_length: 32,
            flags: 24,
            window: 65535,
            checksum: 27676,
            urgent_pointer: 0,
            options: b"\x01\x01\x08\x0a\x37\xc4\x50\xe2\x00\xba\x7c\x1c",
        });    
        assert_eq!(bytes.remaining_len(), 0);

        // encode
        let mut buffer = Buffer::new();
        let _ = value.encode(&mut buffer);
        assert_eq!(*buffer, data);
    }
}