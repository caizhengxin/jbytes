use crate::{
    types::MacAddress,
    JResult, BufRead,
    // ByteDecode, BorrowByteDecode,s
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
    // ErrorKind, make_error,
};


impl crate::ByteDecode for MacAddress {
    #[inline]
    fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Self::from_bits(input.take_byteorder_uint(6, get_byteorder(cattr, fattr))?))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for MacAddress {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Self::from_bits(input.take_byteorder_uint(6, get_byteorder(cattr, fattr))?))
    }
}


#[cfg(test)]
mod tests {
    use crate::{BufRead, Bytes, ByteOrder, BorrowByteDecode};
    use super::*;

    #[test]
    fn test_decode_macaddress() {
        let bytes = Bytes::new([0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(MacAddress::decode(&bytes).unwrap().to_string(), "00:01:02:03:04:05");
        assert_eq!(bytes.remaining_len(), 0);

        // test little-endian byte order.
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(MacAddress::decode_inner(&bytes, None, Some(&fattr)).unwrap().to_string(), "05:04:03:02:01:00");
        assert_eq!(bytes.remaining_len(), 0);

        // error
        let bytes = Bytes::new([0x00, 0x01, 0x02, 0x03, 0x04]);
        assert_eq!(MacAddress::decode(&bytes).is_err(), true);
        assert_eq!(bytes.remaining_len(), 5);
    }
}