use core::mem;
use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
    ErrorKind, make_error,
};


macro_rules! impls_int_decode {
    ($type:ident, $func:tt) => {
        impl ByteDecode for $type {
            #[inline]
            fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
                where 
                    Self: Sized
            {
                let mut value;
                let byteorder = get_byteorder(cattr, fattr);
                let length = if let Some(fr) = fattr { fr.length } else { None };
                

                if let Some(length) = length {
                    if mem::size_of::<$type>().checked_sub(length).is_none() {
                        return Err(make_error(input.get_position(), ErrorKind::InvalidByteLength));
                    }

                    value = input.take_byteorder_uint(length, byteorder)? as $type;
                }
                else {
                    value = input.$func(byteorder)?;
                }

                if let Some(fr) = fattr {
                    if let Some(bits) = fr.bits {
                        let mut bits = bits as $type;
                        value &= bits;
                                                
                        for _i in 0..$type::BITS {
                            if bits & 0x01 == 0 {
                                value >>= 1;
                                bits >>= 1;
                            }
                        }    
                    }
                }

                Ok(value)            
            }
        }        


        impl<'de> BorrowByteDecode<'de> for $type {
            #[inline]
            fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
                where 
                    Self: Sized
            {
                ByteDecode::decode_inner(input, cattr, fattr)
            }
        }
    };
    () => {
        impls_int_decode!(u8, take_byteorder_u8);
        impls_int_decode!(u16, take_byteorder_u16);
        impls_int_decode!(u32, take_byteorder_u32);
        impls_int_decode!(u64, take_byteorder_u64);
        impls_int_decode!(usize, take_byteorder_usize);
        impls_int_decode!(u128, take_byteorder_u128);

        impls_int_decode!(i8, take_byteorder_i8);
        impls_int_decode!(i16, take_byteorder_i16);
        impls_int_decode!(i32, take_byteorder_i32);
        impls_int_decode!(i64, take_byteorder_i64);
        impls_int_decode!(isize, take_byteorder_isize);
        impls_int_decode!(i128, take_byteorder_i128);
    }
}


impls_int_decode!();


#[cfg(test)]
mod tests {
    use crate::{
        Bytes, BufRead, ByteDecode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_decode_int() {
        let bytes = Bytes::new([0x00, 0x00, 0x00, 0x01]);
        assert_eq!(u32::decode(&bytes).unwrap(), 1);
        assert_eq!(bytes.remaining_len(), 0);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([0x01, 0x00, 0x00, 0x00]);
        assert_eq!(u32::decode_inner(&bytes, Some(&cattr), None).unwrap(), 0x00000001);
        assert_eq!(bytes.remaining_len(), 0);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x00, 0x00, 0x01]);
        assert_eq!(u32::decode_inner(&bytes, Some(&cattr), Some(&fattr)).unwrap(), 0x00000001);
        assert_eq!(bytes.remaining_len(), 0);

        // test length
        let fattr = FieldAttrModifiers {
            length: Some(3),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x00, 0x01, 0x02]);
        assert_eq!(u32::decode_inner(&bytes, None, Some(&fattr)).unwrap(), 0x000001);
        assert_eq!(bytes.remaining_len(), 1);

        // test length and LE
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            length: Some(3),
            ..Default::default()
        };
        let bytes = Bytes::new([0x01, 0x00, 0x00, 0x02]);
        assert_eq!(u32::decode_inner(&bytes, None, Some(&fattr)).unwrap(), 0x000001);
        assert_eq!(bytes.remaining_len(), 1);

        // test length = 0
        let fattr = FieldAttrModifiers {
            length: Some(0),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x00, 0x01, 0x02]);
        assert_eq!(u32::decode_inner(&bytes, None, Some(&fattr)).unwrap(), 0);
        assert_eq!(bytes.remaining_len(), 4);

        // test length = 4
        let fattr = FieldAttrModifiers {
            length: Some(4),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x00, 0x00, 0x01]);
        assert_eq!(u32::decode_inner(&bytes, None, Some(&fattr)).unwrap(), 0x00000001);
        assert_eq!(bytes.remaining_len(), 0);

        // test length = 5, error
        let fattr = FieldAttrModifiers {
            length: Some(5),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x00, 0x00, 0x01, 0x00]);
        assert_eq!(u32::decode_inner(&bytes, None, Some(&fattr)).is_err(), true);
        assert_eq!(bytes.remaining_len(), 5);
    }
}