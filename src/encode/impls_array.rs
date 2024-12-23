use crate::{
    JResult, BufWriteMut,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<T: ByteEncode, const N: usize> ByteEncode for [T; N] {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        let mut nbytes = 0;

        for value in self {
            nbytes += T::encode_inner(&value, buffer, cattr, fattr)?;
        }

        Ok(nbytes)
    }
}


impl<T: BorrowByteEncode + ByteEncode, const N: usize> BorrowByteEncode for [T; N] {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        ByteEncode::encode_inner(self, buffer, cattr, fattr)
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        Buffer, BorrowByteEncode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_encode_array() {
        let mut buffer = Buffer::new();
        assert_eq!([0x0001_u16, 0x0002].encode(&mut buffer).unwrap(), 4);
        assert_eq!(*buffer, vec![0x00, 0x01, 0x00, 0x02]);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };

        let mut buffer = Buffer::new();
        assert_eq!([0x0001_u16, 0x0002].encode_inner(&mut buffer, Some(&cattr), None).unwrap(), 4);
        assert_eq!(*buffer, vec![0x01, 0x00, 0x02, 0x00]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!([0x0001_u16, 0x0002].encode_inner(&mut buffer, Some(&cattr), Some(&fattr)).unwrap(), 4);
        assert_eq!(*buffer, vec![0x00, 0x01, 0x00, 0x02]);
    }
}