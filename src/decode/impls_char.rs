use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl ByteDecode for char {
    #[inline]
    fn decode_inner<T: BufRead>(input: &T, _cattr: Option<&ContainerAttrModifiers>, 
                                _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_char()
    }
}


impl<'de> BorrowByteDecode<'de> for char {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, _cattr: Option<&ContainerAttrModifiers>,
                                _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_char()
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        Bytes, BufRead, ByteDecode,
    };

    #[test]
    fn test_decode_bool() {
        let bytes = Bytes::new([0x61, 0x62]);
        assert_eq!(char::decode(&bytes).unwrap(), 'a');
        assert_eq!(bytes.remaining_len(), 1);
        assert_eq!(char::decode(&bytes).unwrap(), 'b');
        assert_eq!(bytes.remaining_len(), 0);
    }
}