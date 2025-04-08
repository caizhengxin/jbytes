use crate::std::*;
use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    // get_byteorder,
};
use super::get_count_and_try_count;


impl<T: ByteDecode> ByteDecode for Vec<T> {
    #[inline]
    fn decode_inner<I>(input: &I, cattr: Option<&ContainerAttrModifiers>,
                                               fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized,
        I: BufRead,
    {
        let mut value_list = Vec::new();
        let loop_skip_starts = if let Some(fr) = fattr { fr.loop_skip_starts } else { None };

        if let Some(loop_skip_starts) = loop_skip_starts {
            while let Ok(_) = input.take_bytes_starts(loop_skip_starts) {
                value_list.push(T::decode_inner(input, cattr, fattr)?);    
            }

            return Ok(value_list);
        }

        let (count, try_count) = get_count_and_try_count(input, cattr, fattr)?;

        if let Some(try_count) = try_count {
            for _i in 0..try_count { 
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
        let loop_skip_starts = if let Some(fr) = fattr { fr.loop_skip_starts } else { None };

        if let Some(loop_skip_starts) = loop_skip_starts {
            while let Ok(_) = input.take_bytes_starts(loop_skip_starts) {
                value_list.push(T::decode_inner(input, cattr, fattr)?);    
            }

            return Ok(value_list);
        }   
        
        let (count, try_count) = get_count_and_try_count(input, cattr, fattr)?;
        
        if let Some(try_count) = try_count {
            for _i in 0..try_count {
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
    use crate::std::*;
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

        // test `byte_count_outside` example
        let bytes = Bytes::new([0x00, 0x02, 0x00, 0x01, 0x00, 0x02]);
        let fattr = FieldAttrModifiers {
            byte_count_outside: Some(2),
            ..Default::default()
        };
        assert_eq!(Vec::<u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), vec![0x0001, 0x0002]);
        assert_eq!(bytes.remaining_len(), 0);

        // test `byte_count_outside` little-endian example
        let bytes = Bytes::new([0x02, 0x00, 0x01, 0x00, 0x02, 0x00]);
        let fattr = FieldAttrModifiers {
            byte_count_outside: Some(2),
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        assert_eq!(Vec::<u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), vec![0x0001, 0x0002]);
        assert_eq!(bytes.remaining_len(), 0);
    }
}