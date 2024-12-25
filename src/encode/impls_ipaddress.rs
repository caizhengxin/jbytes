use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use crate::{
    JResult, BufWriteMut,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
};


impl crate::ByteEncode for Ipv4Addr {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        Ok(buffer.push_byteorder_u32(self.to_bits(), get_byteorder(cattr, fattr))?)
    }
}


impl crate::BorrowByteEncode for Ipv4Addr {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        Ok(buffer.push_byteorder_u32(self.to_bits(), get_byteorder(cattr, fattr))?)
    }
}


impl crate::ByteEncode for Ipv6Addr {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        Ok(buffer.push_byteorder_u128(self.to_bits(), get_byteorder(cattr, fattr))?)
    }
}


impl crate::BorrowByteEncode for Ipv6Addr {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        Ok(buffer.push_byteorder_u128(self.to_bits(), get_byteorder(cattr, fattr))?)
    }
}


impl crate::ByteEncode for IpAddr {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        match self {
            Self::V4(addr) => addr.encode_inner(buffer, cattr, fattr),
            Self::V6(addr) => addr.encode_inner(buffer, cattr, fattr),
        }
    }
}


impl crate::BorrowByteEncode for IpAddr {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        match self {
            Self::V4(addr) => addr.encode_inner(buffer, cattr, fattr),
            Self::V6(addr) => addr.encode_inner(buffer, cattr, fattr),
        }
    }
}


#[cfg(test)]
mod tests {
    use core::{
        net::{IpAddr, Ipv4Addr, Ipv6Addr},
        str::FromStr
    };
    #[allow(unused_imports)]
    use crate::{
        Buffer, BorrowByteEncode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_encode_ipv4() {
        let mut buffer = Buffer::new();
        let addr = Ipv4Addr::from_str("192.168.1.100").unwrap();
        assert_eq!(addr.encode(&mut buffer).unwrap(), 4);
        assert_eq!(*buffer, vec![192, 168, 1, 100]);

        // test little-endian
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        let addr = Ipv4Addr::from_str("192.168.1.100").unwrap();
        assert_eq!(addr.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 4);
        assert_eq!(*buffer, vec![100, 1, 168, 192]);
    }

    #[test]
    fn test_encode_ipv6() {
        let mut buffer = Buffer::new();
        let addr = Ipv6Addr::from_str("fa80::e74:abff:fe93:56f0").unwrap();
        assert_eq!(addr.encode(&mut buffer).unwrap(), 16);
        assert_eq!(*buffer, vec![250, 128, 0, 0, 0, 0, 0, 0, 14, 116, 171, 255, 254, 147, 86, 240]);

        // test little-endian
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        let addr = Ipv6Addr::from_str("fa80::e74:abff:fe93:56f0").unwrap();
        assert_eq!(addr.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 16);
        assert_eq!(*buffer, vec![240, 86, 147, 254, 255, 171, 116, 14, 0, 0, 0, 0, 0, 0, 128, 250]);
    }

    #[test]
    fn test_encode_ipv46() {
        let mut buffer = Buffer::new();
        let addr = IpAddr::V4(Ipv4Addr::from_str("192.168.1.100").unwrap());
        assert_eq!(addr.encode(&mut buffer).unwrap(), 4);
        assert_eq!(*buffer, vec![192, 168, 1, 100]);

        // test little-endian
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(addr.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 4);
        assert_eq!(*buffer, vec![100, 1, 168, 192]);

        // test ipv6
        let mut buffer = Buffer::new();
        let addr = IpAddr::V6(Ipv6Addr::from_str("fa80::e74:abff:fe93:56f0").unwrap());
        assert_eq!(addr.encode(&mut buffer).unwrap(), 16);
        assert_eq!(*buffer, vec![250, 128, 0, 0, 0, 0, 0, 0, 14, 116, 171, 255, 254, 147, 86, 240]);

        // test little-endian
        let mut buffer = Buffer::new();
        assert_eq!(addr.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 16);
        assert_eq!(*buffer, vec![240, 86, 147, 254, 255, 171, 116, 14, 0, 0, 0, 0, 0, 0, 128, 250]);
    }
}