use crate::{
    types::MacAddress,
    JResult, BufWriteMut,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
};


impl crate::ByteEncode for MacAddress {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        buffer.push_byteorder_uint(self.to_bits(), 6, get_byteorder(cattr, fattr))
    }
}


impl crate::BorrowByteEncode for MacAddress {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        buffer.push_byteorder_uint(self.to_bits(), 6, get_byteorder(cattr, fattr))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Buffer, ByteOrder, BorrowByteEncode};
    use super::*;

    #[test]
    fn test_encode_macaddress() {
        let mut buffer = Buffer::new();
        assert_eq!(MacAddress::from_bits(0x000102030405).encode(&mut buffer).unwrap(), 6);
        assert_eq!(*buffer, vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);

        // test little-endian byte order.
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(MacAddress::from_bits(0x000102030405).encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 6);
        assert_eq!(*buffer, vec![0x05, 0x04, 0x03, 0x02, 0x01, 0x00]);
    }
}