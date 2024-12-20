use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl ByteDecode for bool {
    #[inline]
    fn decode_inner<T: BufRead>(input: &T, _cattr: Option<&ContainerAttrModifiers>, 
                                _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_bool()
    }
}


impl<'de> BorrowByteDecode<'de> for bool {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, _cattr: Option<&ContainerAttrModifiers>,
                                _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_bool()
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        Bytes, BufRead, ByteDecode,
    };

    #[test]
    fn test_decode_bool() {
        let bytes = Bytes::new([0x00, 0x01]);
        assert_eq!(bool::decode_inner(&bytes, None, None).unwrap(), false);
        assert_eq!(bytes.remaining_len(), 1);

        let bytes = Bytes::new([0x01, 0x01]);
        assert_eq!(bool::decode_inner(&bytes, None, None).unwrap(), true);
        assert_eq!(bytes.remaining_len(), 1);

        let bytes = Bytes::new([0x10, 0x01]);
        assert_eq!(bool::decode_inner(&bytes, None, None).unwrap(), true);
        assert_eq!(bytes.remaining_len(), 1);

        let bytes = Bytes::new([0xff, 0x01]);
        assert_eq!(bool::decode_inner(&bytes, None, None).unwrap(), true);
        assert_eq!(bytes.remaining_len(), 1);
    }
}