use crate::{
    JResult, BufWrite,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<T: ByteEncode> ByteEncode for Option<T> {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        if let Some(value) = self {
            return value.encode_inner(buffer, cattr, fattr);
        }

        Ok(0)
    }
}


impl<T: BorrowByteEncode> BorrowByteEncode for Option<T> {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        if let Some(value) = self {
            return value.encode_inner(buffer, cattr, fattr);
        }

        Ok(0)
    }
}


#[cfg(test)]
mod tests {
    use crate::std::*;
    use crate::{
        Buffer, BorrowByteEncode,
    };

    #[test]
    fn test_encode_option() {
        let mut buffer = Buffer::new();
        let value: Option<u16> = Some(1);
        assert_eq!(value.encode(&mut buffer).unwrap(), 2);
        assert_eq!(*buffer, vec![0x00, 0x01]);

        let mut buffer = Buffer::new();
        let value: Option<u16> = None;
        assert_eq!(value.encode(&mut buffer).unwrap(), 0);
        assert_eq!(buffer.is_empty(), true);
    }
}