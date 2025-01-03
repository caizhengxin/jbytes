use std::marker::PhantomData;
use std::net::Ipv4Addr;
use jbytes_derive::{BorrowByteDecode, BorrowByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Layer<'a> {
    pub layer12: EthernetHeader<'a>,
    #[jbytes(branch="layer12.r#type")]
    pub layer3: Layer3<'a>,
    #[jbytes(branch="layer3.get_protocol().unwrap_or_default()")]
    pub layer4: Layer4<'a>,
    #[jbytes(remaining)]
    pub remain: &'a [u8],
}


#[derive(Debug, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub enum Layer3<'a> {
    #[jbytes(branch_value=0x0800)]
    Ipv4(Ipv4Header<'a>),
    #[jbytes(branch_default)]
    Unknown,
}


impl<'a> Layer3<'a> {
    pub fn get_protocol(&self) -> Option<u8> {
        match self {
            Self::Ipv4(v) => Some(v.protocol),
            _ => None,
        }
    }
}


#[derive(Debug, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub enum Layer4<'a> {
    #[jbytes(branch_value=6)]
    Tcp(TcpHeader<'a>),
    #[jbytes(branch_value=17)]
    Udp(UdpHeader<'a>),
    #[jbytes(branch_default)]
    Unknown,
}


#[derive(Debug, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct EthernetHeader<'a> {
    pub smac: MacAddress,
    pub dmac: MacAddress,
    pub r#type: u16,
    _mark: PhantomData<&'a ()>,
}


#[derive(Debug, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct Ipv4Header<'a> {
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


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct TcpHeader<'a> {
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


#[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
pub struct UdpHeader<'a> {
    pub sport: u16,
    pub dport: u16,
    pub length: u16,
    pub checksum: u16,
    _mark: PhantomData<&'a ()>,
}


fn main() {
    let data = b"\x00\xc0\x9f\x32\x41\x8c\x00\xe0\x18\xb1\x0c\xad\x08\x00\x45\x00\
    \x00\x38\x00\x00\x40\x00\x40\x11\x65\x47\xc0\xa8\xaa\x08\xc0\xa8\
    \xaa\x14\x80\x1b\x00\x35\x00\x24\x85\xed\x10\x32\x01\x00\x00\x01\
    \x00\x00\x00\x00\x00\x00\x06\x67\x6f\x6f\x67\x6c\x65\x03\x63\x6f\
    \x6d\x00\x00\x10\x00\x01";
    let bytes = Bytes::new(data);

    let value: Layer<'_> = jbytes::decode_borrow(&bytes).unwrap();
    println!(">>> {value:?}");
    assert_eq!(*jbytes::encode_borrow(value).unwrap(), data);
}