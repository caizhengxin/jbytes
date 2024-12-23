use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<T: ByteDecode> ByteDecode for Option<T> {
    #[inline]
    fn decode_inner<I: BufRead>(input: &I, cattr: Option<&ContainerAttrModifiers>,
                                               fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        if let Some(fattr) = fattr {
            if let Some(length) = fattr.length {
                if length == 0 {
                    return Ok(None);
                }
            }
        }

        let value = if let Ok(value) = T::decode_inner(input, cattr, fattr) {
            Some(value)
        } else {
            None
        };

        Ok(value)
    }
}


impl<'de, T: BorrowByteDecode<'de>> BorrowByteDecode<'de> for Option<T> {
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>,
                                               fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        if let Some(fattr) = fattr {
            if let Some(length) = fattr.length {
                if length == 0 {
                    return Ok(None);
                }
            }
        }

        let value = if let Ok(value) = T::decode_inner(input, cattr, fattr) {
            Some(value)
        } else {
            None
        };

        Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        Bytes, BufRead, ByteDecode,
    };

    #[test]
    fn test_decode_option() {
        let bytes = Bytes::new([0x01]);
        assert_eq!(<Option<u16>>::decode(&bytes).unwrap(), None);
        assert_eq!(bytes.remaining_len(), 1);

        let bytes = Bytes::new([0x00, 0x01]);
        assert_eq!(<Option<u16>>::decode(&bytes).unwrap(), Some(0x0001));
        assert_eq!(bytes.remaining_len(), 0);
    }
}