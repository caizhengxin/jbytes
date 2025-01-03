use crate::{
    JResult, BufWrite,
    ContainerAttrModifiers, FieldAttrModifiers,
    types::{HexString, HexBytes},
};
use super::impls_bytes::encode_inner;


impl crate::ByteEncode for HexString {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        encode_inner(buffer, cattr, fattr, self)
    }
}


impl crate::BorrowByteEncode for HexString {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        encode_inner(buffer, cattr, fattr, self)
    }
}


impl<'de> crate::BorrowByteEncode for HexBytes<'de> {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        encode_inner(buffer, cattr, fattr, self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Buffer, BorrowByteEncode,
    };

    #[test]
    fn test_encode_hex_string() {
        let mut buffer = Buffer::new();
        let value = HexString::from(vec![0x00, 0x01, 0x02]);
        assert_eq!(value.encode(&mut buffer).unwrap(), 4);
        assert_eq!(*buffer, vec![0x03, 0x00, 0x01, 0x02]);
    }

    #[test]
    fn test_encode_hex_bytes() {
        let mut buffer = Buffer::new();
        let value = HexBytes::new(b"\x00\x01\x02");
        assert_eq!(value.encode(&mut buffer).unwrap(), 4);
        assert_eq!(*buffer, vec![0x03, 0x00, 0x01, 0x02]);
    }
}