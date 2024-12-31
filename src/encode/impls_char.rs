use crate::{
    JResult, BufWrite,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl ByteEncode for char {
    #[inline]
    fn encode_inner<T: BufWrite>(&self, buffer: &mut T, _cattr: Option<&ContainerAttrModifiers>,
                                                              _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
    {
        buffer.push_char(*self)
    }
}


impl BorrowByteEncode for char {
    #[inline]
    fn encode_inner<T: BufWrite>(&self, buffer: &mut T, _cattr: Option<&ContainerAttrModifiers>,
                                                              _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
    {
        buffer.push_char(*self)
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        Buffer, ByteEncode,
    };

    #[test]
    fn test_encode_bool() {
        let mut buffer = Buffer::new();
        assert_eq!('a'.encode(&mut buffer).unwrap(), 1);
        assert_eq!('b'.encode(&mut buffer).unwrap(), 1);
        assert_eq!(*buffer, vec![0x61, 0x62]);
    }
}