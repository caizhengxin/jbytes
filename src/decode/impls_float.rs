use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    ByteOrder, get_byteorder,
};


impl ByteDecode for f32 {
    #[inline]
    fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        if get_byteorder(cattr, fattr) == ByteOrder::Be {
            return input.take_be_f32();
        } 

        input.take_le_f32()
    }
}


impl<'de> BorrowByteDecode<'de> for f32 {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>,
                                    fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        ByteDecode::decode_inner(input, cattr, fattr)
    }
}


impl ByteDecode for f64 {
    #[inline]
    fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        if get_byteorder(cattr, fattr) == ByteOrder::Be {
            return input.take_be_f64();
        } 

        input.take_le_f64()
    }
}


impl<'de> BorrowByteDecode<'de> for f64 {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>,
                                    fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        ByteDecode::decode_inner(input, cattr, fattr)
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        Bytes, BufRead, ByteDecode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };


    #[test]
    fn test_decode_f32() {
        let bytes = Bytes::new([61, 204, 204, 205]);
        assert_eq!(f32::decode_inner(&bytes, None, None).unwrap(), 0.1);
        assert_eq!(bytes.remaining_len(), 0);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([205, 204, 204, 61]);
        assert_eq!(f32::decode_inner(&bytes, Some(&cattr), None).unwrap(), 0.1);
        assert_eq!(bytes.remaining_len(), 0);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };

        let bytes = Bytes::new([61, 204, 204, 205]);
        assert_eq!(f32::decode_inner(&bytes, Some(&cattr), Some(&fattr)).unwrap(), 0.1);
        assert_eq!(bytes.remaining_len(), 0);
    }

    #[test]
    fn test_decode_f64() {
        let bytes = Bytes::new([63, 185, 153, 153, 153, 153, 153, 154]);
        assert_eq!(f64::decode_inner(&bytes, None, None).unwrap(), 0.1);
        assert_eq!(bytes.remaining_len(), 0);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([154, 153, 153, 153, 153, 153, 185, 63]);
        assert_eq!(f64::decode_inner(&bytes, Some(&cattr), None).unwrap(), 0.1);
        assert_eq!(bytes.remaining_len(), 0);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };

        let bytes = Bytes::new([63, 185, 153, 153, 153, 153, 153, 154]);
        assert_eq!(f64::decode_inner(&bytes, Some(&cattr), Some(&fattr)).unwrap(), 0.1);
        assert_eq!(bytes.remaining_len(), 0);
    }
}
