use core::mem;
use crate::{
    JResult, BufWrite,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
    ErrorKind, make_error,
};


macro_rules! impls_int_encode {
    ($type:ident, $func:tt) => {
        impl ByteEncode for $type {
            #[inline]
            fn encode_inner<T: BufWrite>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
                let r_nbytes;
                let mut value = *self;
                let byteorder = get_byteorder(cattr, fattr);
                let length = if let Some(fr) = fattr { fr.length } else { None };
                
                if let Some(fr) = fattr {
                    if let Some(bits) = fr.bits {
                        let mut bits = bits as $type;
                                                
                        for _i in 0..$type::BITS {
                            if bits & 0x01 == 0 {
                                value <<= 1;
                                bits >>= 1;
                            }
                        }

                        if !fr.bits_start {
                            let byte = ($type::BITS / 8) as usize;
                            buffer.set_position(buffer.get_position() - byte);
                            let prev_bits = buffer.take_byteorder_uint(byte, byteorder)?;
                            buffer.set_position(buffer.get_position() - byte);
                            value |= prev_bits as $type;
                        }
                    }
                }

                if let Some(length) = length {
                    if mem::size_of_val(self).checked_sub(length).is_none() {
                        return Err(make_error(buffer.get_position(), ErrorKind::InvalidByteLength));
                    }

                    r_nbytes = buffer.push_byteorder_uint(value as u64, length, byteorder)?;
                }
                else {
                    r_nbytes = buffer.$func(value, byteorder)?;
                }

                Ok(r_nbytes)            
            }
        }        


        impl BorrowByteEncode for $type {
            #[inline]
            fn encode_inner<T: BufWrite>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
                where 
                    Self: Sized
            {
                ByteEncode::encode_inner(self, buffer, cattr, fattr)
            }
        }
    };
    () => {
        impls_int_encode!(u8, push_byteorder_u8);
        impls_int_encode!(u16, push_byteorder_u16);
        impls_int_encode!(u32, push_byteorder_u32);
        impls_int_encode!(u64, push_byteorder_u64);
        impls_int_encode!(usize, push_byteorder_usize);
        impls_int_encode!(u128, push_byteorder_u128);

        impls_int_encode!(i8, push_byteorder_i8);
        impls_int_encode!(i16, push_byteorder_i16);
        impls_int_encode!(i32, push_byteorder_i32);
        impls_int_encode!(i64, push_byteorder_i64);
        impls_int_encode!(isize, push_byteorder_isize);
        impls_int_encode!(i128, push_byteorder_i128);
    }
}


impls_int_encode!();


#[cfg(test)]
mod tests {
    use crate::{
        Buffer, BorrowByteEncode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_encode_int() {
        let mut buffer = Buffer::new();
        assert_eq!(0x00000001_u32.encode(&mut buffer).unwrap(), 4);
        assert_eq!(*buffer, vec![0x00, 0x00, 0x00, 0x01]);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(0x00000001_u32.encode_inner(&mut buffer, Some(&cattr), None).unwrap(), 4);
        assert_eq!(*buffer, vec![0x01, 0x00, 0x00, 0x00]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(0x00000001_u32.encode_inner(&mut buffer, Some(&cattr), Some(&fattr)).unwrap(), 4);
        assert_eq!(*buffer, vec![0x00, 0x00, 0x00, 0x01]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            length: Some(3),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(0x000001_u32.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 3);
        assert_eq!(*buffer, vec![0x00, 0x00, 0x01]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            length: Some(3),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(0x000001_u32.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 3);
        assert_eq!(*buffer, vec![0x01, 0x00, 0x00]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            length: Some(0),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(0x000001_u32.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 0);
        assert_eq!(buffer.is_empty(), true);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            length: Some(5),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(0x000001_u32.encode_inner(&mut buffer, None, Some(&fattr)).is_err(), true);
        assert_eq!(buffer.is_empty(), true);
    }
}