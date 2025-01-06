use crate::{
    JResult, BufWrite,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    ByteOrder, get_byteorder,
};


impl ByteEncode for f32 {
    #[inline]
    fn encode_inner<T: BufWrite>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>,
                                                              fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
    {
        if get_byteorder(cattr, fattr) == ByteOrder::Be {
            return buffer.push_be_f32(*self);
        }

        buffer.push_le_f32(*self)
    }
}


impl BorrowByteEncode for f32 {
    #[inline]
    fn encode_inner<T: BufWrite>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        ByteEncode::encode_inner(self, buffer, cattr, fattr)
    }
}


impl ByteEncode for f64 {
    #[inline]
    fn encode_inner<T: BufWrite>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>,
                                                              fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
    {
        if get_byteorder(cattr, fattr) == ByteOrder::Be {
            return buffer.push_be_f64(*self);
        }

        buffer.push_le_f64(*self)
    }
}


impl BorrowByteEncode for f64 {
    #[inline]
    fn encode_inner<T: BufWrite>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        ByteEncode::encode_inner(self, buffer, cattr, fattr)
    }
}


#[cfg(test)]
mod tests {
    use crate::std::*;
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

    #[test]
    fn test_encode_f64() {
        let mut buffer = Buffer::new();
        0.1_f64.encode(&mut buffer).unwrap();
        assert_eq!(*buffer, vec![63, 185, 153, 153, 153, 153, 153, 154]);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        0.1_f64.encode_inner(&mut buffer, Some(&cattr), None).unwrap();
        assert_eq!(*buffer, vec![154, 153, 153, 153, 153, 153, 185, 63]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        0.1_f64.encode_inner(&mut buffer, Some(&cattr), Some(&fattr)).unwrap();
        assert_eq!(*buffer, vec![63, 185, 153, 153, 153, 153, 153, 154]);
    }
}