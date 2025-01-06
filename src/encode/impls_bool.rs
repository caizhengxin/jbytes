use crate::{
    JResult, BufWrite,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl ByteEncode for bool {
    #[inline]
    fn encode_inner<T: BufWrite>(&self, buffer: &mut T, _cattr: Option<&ContainerAttrModifiers>,
                                                              _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
    {
        buffer.push_bool(*self)
    }
}


impl BorrowByteEncode for bool {
    #[inline]
    fn encode_inner<T: BufWrite>(&self, buffer: &mut T, _cattr: Option<&ContainerAttrModifiers>,
                                                              _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
    {
        buffer.push_bool(*self)
    }
}


#[cfg(test)]
mod tests {
    use crate::std::*;
    use crate::{
        Buffer, ByteEncode,
    };

    #[test]
    fn test_encode_bool() {
        let mut buffer = Buffer::new();
        false.encode_inner(&mut buffer, None, None).unwrap();
        assert_eq!(*buffer, vec![0x00]);

        let mut buffer = Buffer::new();
        true.encode_inner(&mut buffer, None, None).unwrap();
        assert_eq!(*buffer, vec![0x01]);
    }
}