use crate::{
    types::NetAddress,
    JResult, BufWriteMut,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl crate::ByteEncode for NetAddress {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        match self {
            Self::V4(addr) => addr.encode_inner(buffer, cattr, fattr),
            Self::V6(addr) => addr.encode_inner(buffer, cattr, fattr),
            Self::Mac(addr) => addr.encode_inner(buffer, cattr, fattr),
            Self::Usize(addr) => addr.encode_inner(buffer, cattr, fattr),
        }
    }
}


impl crate::BorrowByteEncode for NetAddress {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        match self {
            Self::V4(addr) => addr.encode_inner(buffer, cattr, fattr),
            Self::V6(addr) => addr.encode_inner(buffer, cattr, fattr),
            Self::Mac(addr) => addr.encode_inner(buffer, cattr, fattr),
            Self::Usize(addr) => addr.encode_inner(buffer, cattr, fattr),
        }
    }
}


#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use core::net::{Ipv4Addr, Ipv6Addr};
    use crate::{
        types::MacAddress,
        Buffer, BorrowByteEncode
    };
    use super::*;

    #[test]
    fn test_encode_netaddress() {
        // test ipv4 address
        let fattr = FieldAttrModifiers {
            length: Some(4),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(NetAddress::V4(Ipv4Addr::from_bits(0x01020304)).encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 4);
        assert_eq!(*buffer, vec![0x01, 0x02, 0x03, 0x04]);

        // test ipv6 address
        let fattr = FieldAttrModifiers {
            length: Some(16),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(NetAddress::V6(Ipv6Addr::from_str("fa80::e74:abff:fe93:56f0").unwrap()).encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 16);
        assert_eq!(*buffer, vec![250, 128, 0, 0, 0, 0, 0, 0, 14, 116, 171, 255, 254, 147, 86, 240]);

        // test mac address
        let fattr = FieldAttrModifiers {
            length: Some(6),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(NetAddress::Mac(MacAddress::from_bits(0x010203040506)).encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 6);
        assert_eq!(*buffer, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);

        // test integer address
        let fattr = FieldAttrModifiers {
            length: Some(1),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(NetAddress::Usize(1).encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 1);
        assert_eq!(*buffer, vec![0x01]);

        // test integer address
        let fattr = FieldAttrModifiers {
            length: Some(2),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(NetAddress::Usize(1).encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 2);
        assert_eq!(*buffer, vec![0x00, 0x01]);
    }
}