use std::collections::HashMap;
use core::hash::Hash;
use core::cmp::Eq;
use crate::{
    JResult, BufRead,
    // ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    // get_byteorder,
    // ErrorKind, make_error,
};
use super::get_count_and_try_count;


impl<K, V> crate::ByteDecode for HashMap<K, V>
where
    K: crate::ByteDecode + Hash + Eq,
    V: crate::ByteDecode + Hash + Eq,
{
    #[inline]
    fn decode_inner<I: BufRead>(input: &I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let (count, try_count) = get_count_and_try_count(input, cattr, fattr)?;
        let mut hashmap = HashMap::new();

        let mut k_fattr = None;
        let mut v_fattr = None;

        if let Some(fr) = fattr {
            if fr.split.is_some() {
                k_fattr = Some(FieldAttrModifiers {
                    linend_value: fr.split,
                    ..Default::default()
                });
            }

            if fr.linend_value.is_some() {
                v_fattr = Some(FieldAttrModifiers {
                    linend_value: fr.linend_value,
                    ..Default::default()
                });    
            }
            else if fr.linend {
                v_fattr = Some(FieldAttrModifiers {
                    linend: true,
                    ..Default::default()
                });    
            }
        }
        let k_fattr = k_fattr.as_ref().or(fattr);
        let v_fattr = v_fattr.as_ref().or(fattr);

        if let Some(try_count) = try_count {
            for _ in 0..try_count {
                match K::decode_inner(input, cattr, k_fattr) {
                    Ok(key) => {
                        match V::decode_inner(input, cattr, v_fattr) {
                            Ok(value) => hashmap.insert(key, value),
                            Err(_e) => break,
                        }
                    },
                    Err(_e) => break,
                };
            }
        } else {
            for _ in 0..count {
                hashmap.insert(K::decode_inner(input, cattr, k_fattr)?, V::decode_inner(input, cattr, v_fattr)?);
            }    
        }
    
        Ok(hashmap)
    }
}


impl<'de, K, V> crate::BorrowByteDecode<'de> for HashMap<K, V>
where
    K: crate::BorrowByteDecode<'de> + Hash + Eq,
    V: crate::BorrowByteDecode<'de> + Hash + Eq,
{
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let mut hashmap = HashMap::new();
        let (count, try_count) = get_count_and_try_count(input, cattr, fattr)?;

        let mut k_fattr = None;
        let mut v_fattr = None;

        if let Some(fr) = fattr {
            if fr.split.is_some() {
                k_fattr = Some(FieldAttrModifiers {
                    linend_value: fr.split,
                    ..Default::default()
                });
            }

            if fr.linend_value.is_some() {
                v_fattr = Some(FieldAttrModifiers {
                    linend_value: fr.linend_value,
                    ..Default::default()
                });    
            }
            else if fr.linend {
                v_fattr = Some(FieldAttrModifiers {
                    linend: true,
                    ..Default::default()
                });    
            }
        }
        let k_fattr = k_fattr.as_ref().or(fattr);
        let v_fattr = v_fattr.as_ref().or(fattr);
    
        if let Some(try_count) = try_count {
            for _ in 0..try_count {
                match K::decode_inner(input, cattr, k_fattr) {
                    Ok(key) => {
                        match V::decode_inner(input, cattr, v_fattr) {
                            Ok(value) => hashmap.insert(key, value),
                            Err(_e) => break,
                        }
                    },
                    Err(_e) => break,
                };
            }
        } else {
            for _ in 0..count {
                hashmap.insert(K::decode_inner(input, cattr, k_fattr)?, V::decode_inner(input, cattr, v_fattr)?);
            }    
        }
    
        Ok(hashmap)
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{
        BufRead, Bytes, BorrowByteDecode,
        FieldAttrModifiers,
    };

    #[test]
    fn test_decode_hashmap_int_and_int() {
        // test default example
        let data = [
            0x03,                      // Element quantity
            0x00, 0x01, 0x00, 0x02,
            0x00, 0x03, 0x00, 0x04,
            0x00, 0x05, 0x00, 0x06,
        ];
        let bytes = Bytes::new(data);
        let value = HashMap::from([
            (0x0001, 0x0002),
            (0x0003, 0x0004),
            (0x0005, 0x0006),
        ]);
        assert_eq!(HashMap::<u16, u16>::decode(&bytes).unwrap(), value);
        assert_eq!(bytes.remaining_len(), 0);

        // test count modifier
        let data = [
            0x00, 0x01, 0x00, 0x02,
            0x00, 0x03, 0x00, 0x04,
            0x00, 0x05, 0x00, 0x06,
        ];
        let bytes = Bytes::new(data);
        let value = HashMap::from([
            (0x0001, 0x0002),
            (0x0003, 0x0004),
            (0x0005, 0x0006),
        ]);
        let fattr = FieldAttrModifiers {
            count: Some(3),
            ..Default::default()
        };
        assert_eq!(HashMap::<u16, u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), value);
        assert_eq!(bytes.remaining_len(), 0);

        // test try_count modifier
        let data = [
            0x00, 0x01, 0x00, 0x02,
            0x00, 0x03, 0x00, 0x04,
            0x00, 0x05, 0x00, 0x06,
        ];
        let bytes = Bytes::new(data);
        let value = HashMap::from([
            (0x0001, 0x0002),
            (0x0003, 0x0004),
            (0x0005, 0x0006),
        ]);
        let fattr = FieldAttrModifiers {
            try_count: Some(10),
            ..Default::default()
        };
        assert_eq!(HashMap::<u16, u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), value);
        assert_eq!(bytes.remaining_len(), 0);

        // test byte_count_outside
        let data = [
            0x00, 0x03,
            0x00, 0x01, 0x00, 0x02,
            0x00, 0x03, 0x00, 0x04,
            0x00, 0x05, 0x00, 0x06,
        ];
        let bytes = Bytes::new(data);
        let value = HashMap::from([
            (0x0001, 0x0002),
            (0x0003, 0x0004),
            (0x0005, 0x0006),
        ]);
        let fattr = FieldAttrModifiers {
            byte_count_outside: Some(2),
            ..Default::default()
        };
        assert_eq!(HashMap::<u16, u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), value);
        assert_eq!(bytes.remaining_len(), 0);
    }


    #[test]
    fn test_decode_hashmap_str_and_int() {
        // test default example
        let data = [
            0x03,                      // Element quantity
            0x04, 'k' as u8, 'e' as u8, 'y' as u8, '1' as u8, 0x00, 0x02,
            0x04, 'k' as u8, 'e' as u8, 'y' as u8, '2' as u8, 0x00, 0x04,
            0x04, 'k' as u8, 'e' as u8, 'y' as u8, '3' as u8, 0x00, 0x06,
        ];
        let bytes = Bytes::new(data);
        let value = HashMap::from([
            ("key1", 0x0002),
            ("key2", 0x0004),
            ("key3", 0x0006),
        ]);
        assert_eq!(HashMap::<&str, u16>::decode(&bytes).unwrap(), value);
        assert_eq!(bytes.remaining_len(), 0);

        // test `split` example
        let data = [
            0x03,                      // Element quantity
            'k' as u8, 'e' as u8, 'y' as u8, '1' as u8, ':' as u8, 0x00, 0x02,
            'k' as u8, 'e' as u8, 'y' as u8, '2' as u8, ':' as u8, 0x00, 0x04,
            'k' as u8, 'e' as u8, 'y' as u8, '3' as u8, ':' as u8, 0x00, 0x06,
        ];
        let bytes = Bytes::new(data);
        let value = HashMap::from([
            ("key1", 0x0002),
            ("key2", 0x0004),
            ("key3", 0x0006),
        ]);
        let fattr = FieldAttrModifiers {
            split: Some(b":"),
            ..Default::default()
        };
        assert_eq!(HashMap::<&str, u16>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), value);
        assert_eq!(bytes.remaining_len(), 0);
    }

    #[test]
    fn test_decode_hashmap_str_and_str() {
        let data = b"K1: V1\r\nK2: V2\r\nK3: V3\r\n";
        let bytes = Bytes::new(data);
        let value = HashMap::from([
            ("K1", "V1"),
            ("K2", "V2"),
            ("K3", "V3"),
        ]);
        let fattr = FieldAttrModifiers {
            try_count: Some(50),
            split: Some(b": "),
            linend_value: Some(b"\r\n"),
            ..Default::default()
        };
        assert_eq!(HashMap::<&str, &str>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), value);
        assert_eq!(bytes.remaining_len(), 0);
    }
}