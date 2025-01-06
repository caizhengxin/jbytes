use crate::{
    types::NetAddress,
    JResult, BufRead,
    // ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    // get_byteorder,
    ErrorKind, make_error,
};


impl crate::ByteDecode for NetAddress {
    #[inline]
    fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        if let Some(fattr_var) = fattr {
            if let Some(length) = fattr_var.length {
                match length {
                    0 => { },
                    4 => {
                        return Ok(Self::V4(crate::ByteDecode::decode_inner(input, cattr, fattr)?));
                    },
                    6 => {
                        return Ok(Self::Mac(crate::ByteDecode::decode_inner(input, cattr, fattr)?));
                    },
                    16 => {
                        return Ok(Self::V6(crate::ByteDecode::decode_inner(input, cattr, fattr)?));
                    },
                    _ => {
                        return Ok(Self::Usize(crate::ByteDecode::decode_inner(input, cattr, fattr)?));
                    },
                }    
            }
        }

        Err(make_error(input.get_position(), ErrorKind::Fail))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for NetAddress {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        crate::ByteDecode::decode_inner(input, cattr, fattr)
    }
}


#[cfg(test)]
mod tests {
    use crate::std::*;
    use crate::{BufRead, Bytes, BorrowByteDecode};
    use super::*;

    #[test]
    fn test_decode_netaddress() {
        // test ipv4
        let bytes = Bytes::new([192, 168, 1, 100]);
        let fattr = FieldAttrModifiers {
            length: Some(4),
            ..Default::default()
        };
        assert_eq!(NetAddress::decode_inner(&bytes, None, Some(&fattr)).unwrap().to_string(), "192.168.1.100");
        assert_eq!(bytes.remaining_len(), 0);

        // test ipv6
        let bytes = Bytes::new([250, 128, 0, 0, 0, 0, 0, 0, 14, 116, 171, 255, 254, 147, 86, 240]);
        let fattr = FieldAttrModifiers {
            length: Some(16),
            ..Default::default()
        };
        assert_eq!(NetAddress::decode_inner(&bytes, None, Some(&fattr)).unwrap().to_string(), "fa80::e74:abff:fe93:56f0");
        assert_eq!(bytes.remaining_len(), 0);

        // test mac
        let bytes = Bytes::new([0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
        let fattr = FieldAttrModifiers {
            length: Some(6),
            ..Default::default()
        };
        assert_eq!(NetAddress::decode_inner(&bytes, None, Some(&fattr)).unwrap().to_string(), "00:01:02:03:04:05");
        assert_eq!(bytes.remaining_len(), 0);

        // test usize
        let bytes = Bytes::new([0x01]);
        let fattr = FieldAttrModifiers {
            length: Some(1),
            ..Default::default()
        };
        assert_eq!(NetAddress::decode_inner(&bytes, None, Some(&fattr)).unwrap().to_string(), "1");
        assert_eq!(bytes.remaining_len(), 0);

        // test usize
        let bytes = Bytes::new([0x00, 0x01]);
        let fattr = FieldAttrModifiers {
            length: Some(2),
            ..Default::default()
        };
        assert_eq!(NetAddress::decode_inner(&bytes, None, Some(&fattr)).unwrap().to_string(), "1");
        assert_eq!(bytes.remaining_len(), 0);

        // error
        let bytes = Bytes::new([0x01]);
        let fattr = FieldAttrModifiers {
            length: Some(0),
            ..Default::default()
        };
        assert_eq!(NetAddress::decode_inner(&bytes, None, Some(&fattr)).is_err(), true);
        assert_eq!(bytes.remaining_len(), 1);
    }
}