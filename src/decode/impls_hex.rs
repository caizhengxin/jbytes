use crate::{
    JResult, BufRead,
    ContainerAttrModifiers, FieldAttrModifiers,
    types::{HexString, HexBytes},
};
use super::impls_bytes::find_subsequence;


impl crate::ByteDecode for HexString {
    #[inline]
    fn decode_inner<I: BufRead>(input: &I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Self::from(find_subsequence(input, cattr, fattr)?.to_vec()))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for HexString {
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Self::from(find_subsequence(input, cattr, fattr)?.to_vec()))
    }
}


impl<'de> crate::BorrowByteDecode<'de> for HexBytes<'de> {
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(HexBytes::new(find_subsequence(input, cattr, fattr)?))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Bytes, BufRead, BorrowByteDecode,
    };

    #[test]
    fn test_decode_hex_string() {
        let data = b"\x03\x00\x01\x02";
        let bytes = Bytes::new(data);
        assert_eq!(HexString::decode(&bytes).unwrap().to_hex_lowercase(), "000102");
        assert_eq!(bytes.remaining_len(), 0);
    }

    #[test]
    fn test_decode_hex_bytes() {
        let data = b"\x03\x00\x01\x02";
        let bytes = Bytes::new(data);
        assert_eq!(HexBytes::decode(&bytes).unwrap().to_hex_lowercase(), "000102");
        assert_eq!(bytes.remaining_len(), 0);
    }
}