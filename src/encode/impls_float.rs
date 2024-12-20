use crate::{
    JResult, BufWriteMut,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    ByteOrder, get_byteorder,
};


impl ByteEncode for f32 {
    #[inline]
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>,
                                                              fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
    {
        if get_byteorder(cattr, fattr) == ByteOrder::Be {
            return input.push_be_f32(*self);
        }

        input.push_le_f32(*self)
    }
}


impl BorrowByteEncode for f32 {
    #[inline]
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        ByteEncode::encode_inner(self, input, cattr, fattr)
    }
}


#[cfg(test)]
mod tests {
    // use super::*;
    use crate::{
        Buffer, BorrowByteEncode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_encode_f32() {
        let mut buffer = Buffer::new();
        0.1_f32.encode_inner(&mut buffer, None, None).unwrap();
        assert_eq!(*buffer, vec![61, 204, 204, 205]);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        0.1_f32.encode_inner(&mut buffer, Some(&cattr), None).unwrap();
        assert_eq!(*buffer, vec![205, 204, 204, 61]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        0.1_f32.encode_inner(&mut buffer, Some(&cattr), Some(&fattr)).unwrap();
        assert_eq!(*buffer, vec![61, 204, 204, 205]);
    }
}