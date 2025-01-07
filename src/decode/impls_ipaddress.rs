use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
    ErrorKind, make_error,
};


impl ByteDecode for Ipv4Addr {
    #[inline]
    fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Self::from(input.take_byteorder_u32(get_byteorder(cattr, fattr))?))
    }
}


impl<'de> BorrowByteDecode<'de> for Ipv4Addr {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Self::from(input.take_byteorder_u32(get_byteorder(cattr, fattr))?))
    }
}


impl ByteDecode for Ipv6Addr {
    #[inline]
    fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Self::from(input.take_byteorder_u128(get_byteorder(cattr, fattr))?))
    }
}


impl<'de> BorrowByteDecode<'de> for Ipv6Addr {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Self::from(input.take_byteorder_u128(get_byteorder(cattr, fattr))?))
    }
}


impl ByteDecode for IpAddr {
    #[inline]
    fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        if let Some(fattr_var) = fattr {
            if let Some(length) = fattr_var.length {
                if length == 16 {    
                    return Ok(Self::V6(ByteDecode::decode_inner(input, cattr, fattr)?));
                }
                else if length == 4 {
                    return Ok(Self::V4(ByteDecode::decode_inner(input, cattr, fattr)?));
                }    
            }
        }

        Err(make_error(input.get_position(), ErrorKind::Fail))
    }
}


impl<'de> BorrowByteDecode<'de> for IpAddr {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        ByteDecode::decode_inner(input, cattr, fattr)
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
        Bytes, BufRead, ByteDecode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_decode_ipv4() {
        let bytes = Bytes::new([192, 168, 1, 100]);
        assert_eq!(Ipv4Addr::decode(&bytes).unwrap(), Ipv4Addr::from_str("192.168.1.100").unwrap());
        assert_eq!(bytes.remaining_len(), 0);

        // test little-endian
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([100, 1, 168, 192]);
        assert_eq!(Ipv4Addr::decode_inner(&bytes, None, Some(&fattr)).unwrap(), Ipv4Addr::from_str("192.168.1.100").unwrap());
        assert_eq!(bytes.remaining_len(), 0);
    }

    #[test]
    fn test_decode_ipv6() {
        let bytes = Bytes::new([250, 128, 0, 0, 0, 0, 0, 0, 14, 116, 171, 255, 254, 147, 86, 240]);
        assert_eq!(Ipv6Addr::decode(&bytes).unwrap(), Ipv6Addr::from_str("fa80::e74:abff:fe93:56f0").unwrap());
        assert_eq!(bytes.remaining_len(), 0);

        // test little-endian
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([240, 86, 147, 254, 255, 171, 116, 14, 0, 0, 0, 0, 0, 0, 128, 250]);
        assert_eq!(Ipv6Addr::decode_inner(&bytes, None, Some(&fattr)).unwrap(), Ipv6Addr::from_str("fa80::e74:abff:fe93:56f0").unwrap());
        assert_eq!(bytes.remaining_len(), 0);
    }

    #[test]
    fn test_decode_ipv46() {
        // test ipv4
        let bytes = Bytes::new([192, 168, 1, 100]);
        let fattr = FieldAttrModifiers {
            length: Some(4),
            ..Default::default()
        };
        let addr = IpAddr::V4(Ipv4Addr::from_str("192.168.1.100").unwrap());
        assert_eq!(IpAddr::decode_inner(&bytes, None, Some(&fattr)).unwrap(), addr);
        assert_eq!(bytes.remaining_len(), 0);

        // test ipv4 little-endian
        let fattr = FieldAttrModifiers {
            length: Some(4),
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([100, 1, 168, 192]);
        assert_eq!(IpAddr::decode_inner(&bytes, None, Some(&fattr)).unwrap(), addr);
        assert_eq!(bytes.remaining_len(), 0);

        // test ipv4 error
        let bytes = Bytes::new([192, 168, 1]);
        let fattr = FieldAttrModifiers {
            length: Some(4),
            ..Default::default()
        };
        assert_eq!(IpAddr::decode_inner(&bytes, None, Some(&fattr)).is_err(), true);
        assert_eq!(bytes.remaining_len(), 3);

        // test ipv6
        let fattr = FieldAttrModifiers {
            length: Some(16),
            ..Default::default()
        };
        let bytes = Bytes::new([250, 128, 0, 0, 0, 0, 0, 0, 14, 116, 171, 255, 254, 147, 86, 240]);
        let addr = IpAddr::V6(Ipv6Addr::from_str("fa80::e74:abff:fe93:56f0").unwrap());
        assert_eq!(Ipv6Addr::decode_inner(&bytes, None, Some(&fattr)).unwrap(), addr);
        assert_eq!(bytes.remaining_len(), 0);

        // test ipv6 little-endian
        let fattr = FieldAttrModifiers {
            length: Some(16),
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([240, 86, 147, 254, 255, 171, 116, 14, 0, 0, 0, 0, 0, 0, 128, 250]);
        assert_eq!(IpAddr::decode_inner(&bytes, None, Some(&fattr)).unwrap(), addr);
        assert_eq!(bytes.remaining_len(), 0);

        // error, no length modifiers.
        let bytes = Bytes::new([192, 168, 1, 100]);
        assert_eq!(IpAddr::decode_inner(&bytes, None, None).is_err(), true);
        assert_eq!(bytes.remaining_len(), 4);
    }
}