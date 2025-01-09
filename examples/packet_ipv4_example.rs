
use std::net::Ipv4Addr;
use std::str::FromStr;
use jbytes::{BorrowByteDecode, BorrowByteEncode};
use jbytes_derive::{BorrowByteDecode, BorrowByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Ipv4<'a> {
    #[jbytes(bits_start=0xf0, untake)]
    pub version: u8,
    #[jbytes(bits=0x0f, value_decode="header_length << 2", value_encode="header_length >> 2")]
    pub header_length: u8,
    pub tos: u8,
    pub total_length: u16,
    pub identification: u16,
    #[jbytes(bits_start=0xe000, untake)]
    pub flags: u16,
    #[jbytes(bits=0x1fff)]
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src: Ipv4Addr,
    pub dst: Ipv4Addr,
    #[jbytes(length="header_length - 20")]
    pub options: &'a [u8],
}


fn main() -> JResult<()> {
    let data = b"\x45\x00\x00\x40\xb5\xf2\x00\x00\x40\x06\xa9\x7c\x0a\x01\x01\xea\x0a\x0a\x05\x55";    
    let bytes = Bytes::new(data);

    // decode
    let value = Ipv4::decode(&bytes)?;
    assert_eq!(value, Ipv4 {
        version: 4,
        header_length: 20,
        tos: 0,
        total_length: 64,
        identification: 46578,
        flags: 0,
        fragment_offset: 0,
        ttl: 64,
        protocol: 6,
        checksum: 43388,
        src: Ipv4Addr::from_str("10.1.1.234").unwrap(),
        dst: Ipv4Addr::from_str("10.10.5.85").unwrap(),
        options: &[],
    });
    assert_eq!(bytes.remaining_len(), 0);

    // encode
    let mut buffer = Buffer::new();
    let _ = value.encode(&mut buffer);
    assert_eq!(*buffer, data);

    Ok(())
}