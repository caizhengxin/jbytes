use core::mem::MaybeUninit;
use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<T: ByteDecode, const N: usize> ByteDecode for [T; N] {
    #[inline]
    fn decode_inner<I: BufRead>(input: &I, cattr: Option<&ContainerAttrModifiers>,
                                               fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where
        Self: Sized
    {
        let mut array = MaybeUninit::<[T; N]>::uninit();

        let ptr = unsafe { &mut *array.as_mut_ptr() };

        for i in 0..N {
            ptr[i] = T::decode_inner(input, cattr, fattr)?;
        }

        Ok(unsafe { array.assume_init() })
    }
}


impl<'de, T: BorrowByteDecode<'de>, const N: usize> BorrowByteDecode<'de> for [T; N] {
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>,
                                    fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let mut array = MaybeUninit::<[T; N]>::uninit();

        let ptr = unsafe { &mut *array.as_mut_ptr() };

        for i in 0..N {
            ptr[i] = T::decode_inner(input, cattr, fattr)?;
        }

        Ok(unsafe { array.assume_init() })
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        Bytes, BufRead, ByteDecode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_decode_array() {
        // test default big-endian
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x2]);
        assert_eq!(<[u16; 2]>::decode_inner(&bytes, None, None).unwrap(), [0x0001, 0x0002]);
        assert_eq!(bytes.remaining_len(), 0);

        // test little-endian
        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x2]);
        assert_eq!(<[u16; 2]>::decode_inner(&bytes, Some(&cattr), None).unwrap(), [0x0100, 0x0200]);
        assert_eq!(bytes.remaining_len(), 0);

        // test big-endian
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x2]);
        assert_eq!(<[u16; 2]>::decode_inner(&bytes, Some(&cattr), Some(&fattr)).unwrap(), [0x0001, 0x0002]);
        assert_eq!(bytes.remaining_len(), 0);

        // test error
        let bytes = Bytes::new([0x00, 0x01, 0x00]);
        assert_eq!(<[u16; 2]>::decode_inner(&bytes, None, None).is_err(), true);
        assert_eq!(bytes.remaining_len(), 1);
    }
}