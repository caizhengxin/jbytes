use core::mem;
use crate::{
    JResult, BufWriteMut,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    ByteOrder, get_byteorder,
    ErrorKind, make_error,
};


macro_rules! impls_int_encode {
    ($type:ident, $be_func:tt, $le_func:tt) => {
        impl ByteEncode for $type {
            #[inline]
            fn encode_inner<T: BufWriteMut>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
                let value;
                let byteorder = get_byteorder(cattr, fattr);
                let length = if let Some(fr) = fattr { fr.length } else { None };
                

                if let Some(length) = length {
                    if mem::size_of_val(self).checked_sub(length).is_none() {
                        return Err(make_error(buffer.get_position(), ErrorKind::InvalidByteLength));
                    }

                    if byteorder == ByteOrder::Be {
                        value = buffer.push_be_uint(*self as u64, length)?;
                    }
                    else {
                        value = buffer.push_le_uint(*self as u64, length)?;
                    }
                }
                else if byteorder == ByteOrder::Be {
                    value = buffer.$be_func(*self)?;
                }
                else {
                    value = buffer.$le_func(*self)?;
                }


                Ok(value)            
            }
        }        


        impl BorrowByteEncode for $type {
            #[inline]
            fn encode_inner<T: BufWriteMut>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
                where 
                    Self: Sized
            {
                ByteEncode::encode_inner(self, buffer, cattr, fattr)
            }
        }
    };
    () => {
        impls_int_encode!(u8, push_u8, push_u8);
        impls_int_encode!(u16, push_be_u16, push_le_u16);
        impls_int_encode!(u32, push_be_u32, push_le_u32);
        impls_int_encode!(u64, push_be_u64, push_le_u64);
        impls_int_encode!(usize, push_be_usize, push_le_usize);
        impls_int_encode!(u128, push_be_u128, push_le_u128);

        impls_int_encode!(i8, push_i8, push_i8);
        impls_int_encode!(i16, push_be_i16, push_le_i16);
        impls_int_encode!(i32, push_be_i32, push_le_i32);
        impls_int_encode!(i64, push_be_i64, push_le_i64);
        impls_int_encode!(isize, push_be_isize, push_le_isize);
        impls_int_encode!(i128, push_be_i128, push_le_i128);
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
        assert_eq!(*buffer, vec![]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            length: Some(5),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(0x000001_u32.encode_inner(&mut buffer, None, Some(&fattr)).is_err(), true);
        assert_eq!(*buffer, vec![]);
    }
}