use crate::{
    JResult, BufWriteMut,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl ByteEncode for bool {
    #[inline]
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, _cattr: Option<&ContainerAttrModifiers>,
                                                              _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
    {
        input.push_bool(*self)
    }
}


impl BorrowByteEncode for bool {
    #[inline]
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, _cattr: Option<&ContainerAttrModifiers>,
                                                              _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
    {
        input.push_bool(*self)
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
        false.encode_inner(&mut buffer, None, None).unwrap();
        assert_eq!(*buffer, vec![0x00]);

        let mut buffer = Buffer::new();
        true.encode_inner(&mut buffer, None, None).unwrap();
        assert_eq!(*buffer, vec![0x01]);
    }
}