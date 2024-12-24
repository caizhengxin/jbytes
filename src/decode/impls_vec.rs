use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
};


impl<T: ByteDecode> ByteDecode for Vec<T> {
    #[inline]
    fn decode_inner<I>(input: &I, cattr: Option<&ContainerAttrModifiers>,
                                               fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized,
        I: BufRead,
    {
        let mut value_list = Vec::new();
        let byteorder = get_byteorder(cattr, fattr);
        let count;
        let mut try_count = None;

        if let Some(fr) = fattr {
            try_count = fr.try_count;

            count = if let Some(count) = fr.count {
                count
            } else if let Some(count) = fr.try_count {
                count
            } else if let Some(byte_count) = fr.byte_count {
                input.take_byteorder_uint(byte_count, byteorder)?
            } else {
                input.take_u8()? as usize
            };
        } else {
            count = input.take_u8()? as usize;
        }

        if try_count.is_some() {
            for _i in 0..count {
                match T::decode_inner(input, cattr, fattr) {
                    Ok(value) => value_list.push(value),
                    Err(_e) => break,
                }
            }
        }
        else {
            for _i in 0..count {
                value_list.push(T::decode_inner(input, cattr, fattr)?);    
            }      
        }

        Ok(value_list)
    }
}


impl<'de, T: BorrowByteDecode<'de>> BorrowByteDecode<'de> for Vec<T> {
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>,
                                               fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let mut value_list = Vec::new();
        let byteorder = get_byteorder(cattr, fattr);
        let count;
        let mut try_count = None;

        if let Some(fr) = fattr {
            try_count = fr.try_count;

            count = if let Some(count) = fr.count {
                count
            } else if let Some(count) = fr.try_count {
                count
            } else if let Some(byte_count) = fr.byte_count {
                input.take_byteorder_uint(byte_count, byteorder)?
            } else {
                input.take_u8()? as usize
            };
        } else {
            count = input.take_u8()? as usize;
        }

        if try_count.is_some() {
            for _i in 0..count {
                match T::decode_inner(input, cattr, fattr) {
                    Ok(value) => value_list.push(value),
                    Err(_e) => break,
                }
            }
        }
        else {
            for _i in 0..count {
                value_list.push(T::decode_inner(input, cattr, fattr)?);    
            }      
        }

        Ok(value_list)
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::{
        Bytes, BufRead, ByteDecode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_decode_vec() {
        // test default example
        let bytes = Bytes::new([0x02, 0x00, 0x01, 0x00, 0x02]);
        assert_eq!(Vec::<u16>::decode(&bytes).unwrap(), vec![0x0001, 0x0002]);
        assert_eq!(bytes.remaining_len(), 0);

        // test default error example
        let bytes = Bytes::new([0x03, 0x00, 0x01, 0x00, 0x02]);
        assert_eq!(Vec::<u16>::decode(&bytes).is_err(), true);
        assert_eq!(bytes.remaining_len(), 0);

        // test little-endian example
        let bytes = Bytes::new([0x02, 0x00, 0x01, 0x00, 0x02]);
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        assert_eq!(Vec::<u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), vec![0x0100, 0x0200]);
        assert_eq!(bytes.remaining_len(), 0);

        // test `count` example
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
        let fattr = FieldAttrModifiers {
            count: Some(2),
            ..Default::default()
        };
        assert_eq!(Vec::<u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), vec![0x0001, 0x0002]);
        assert_eq!(bytes.remaining_len(), 0);

        // test `count` error example
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
        let fattr = FieldAttrModifiers {
            count: Some(3),
            ..Default::default()
        };
        assert_eq!(Vec::<u16>::decode_inner(&bytes, None, Some(&fattr)).is_err(), true);
        assert_eq!(bytes.remaining_len(), 0);

        // test `try_count` example
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
        let fattr = FieldAttrModifiers {
            try_count: Some(10),
            ..Default::default()
        };
        assert_eq!(Vec::<u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), vec![0x0001, 0x0002]);
        assert_eq!(bytes.remaining_len(), 0);

        // test `byte_count` example
        let bytes = Bytes::new([0x00, 0x02, 0x00, 0x01, 0x00, 0x02]);
        let fattr = FieldAttrModifiers {
            byte_count: Some(2),
            ..Default::default()
        };
        assert_eq!(Vec::<u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), vec![0x0001, 0x0002]);
        assert_eq!(bytes.remaining_len(), 0);

        // test `byte_count` little-endian example
        let bytes = Bytes::new([0x02, 0x00, 0x01, 0x00, 0x02, 0x00]);
        let fattr = FieldAttrModifiers {
            byte_count: Some(2),
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        assert_eq!(Vec::<u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), vec![0x0001, 0x0002]);
        assert_eq!(bytes.remaining_len(), 0);
    }
}