use std::collections::HashSet;
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


#[inline]
fn hashset_decode<I, T>(input: &I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<HashSet<T>>
where
    I: BufRead,
    T: crate::ByteDecode + Hash + Eq,
{
    let mut hashset = HashSet::new();
    let (count, try_count) = get_count_and_try_count(input, cattr, fattr)?;

    if let Some(try_count) = try_count {
        for _ in 0..try_count {
            match T::decode_inner(input, cattr, fattr) {
                Ok(value) => hashset.insert(value),
                Err(_e) => break,
            };
        }
    } else {
        for _ in 0..count {
            hashset.insert(T::decode_inner(input, cattr, fattr)?);
        }    
    }

    Ok(hashset)
}


impl<T: crate::ByteDecode> crate::ByteDecode for HashSet<T>
where
    T: crate::ByteDecode + Hash + Eq,
{
    #[inline]
    fn decode_inner<I: BufRead>(input: &I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        hashset_decode(input, cattr, fattr)
    }
}


impl<'de, T: crate::BorrowByteDecode<'de>> crate::BorrowByteDecode<'de> for HashSet<T>
where
    T: crate::ByteDecode + Hash + Eq,
{
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        hashset_decode(input, cattr, fattr)
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        BufRead, Bytes, BorrowByteDecode,
    };
    use super::*;

    #[test]
    fn test_decode_hashset() {
        // test default example
        let bytes = Bytes::new([0x02, 0x00, 0x01, 0x00, 0x02]);
        assert_eq!(<HashSet<u16>>::decode(&bytes).unwrap(), HashSet::from([0x0001, 0x0002]));
        assert_eq!(bytes.remaining_len(), 0);

        // test `count` example
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
        let fattr = FieldAttrModifiers {
            count: Some(2),
            ..Default::default()
        };
        assert_eq!(<HashSet<u16>>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), HashSet::from([0x0001, 0x0002]));
        assert_eq!(bytes.remaining_len(), 0);

        // test error example
        let bytes = Bytes::new([0x00, 0x01]);
        let fattr = FieldAttrModifiers {
            count: Some(2),
            ..Default::default()
        };
        assert_eq!(<HashSet<u16>>::decode_inner(&bytes, None, Some(&fattr)).is_err(), true);
        assert_eq!(bytes.remaining_len(), 0);

        // test `try_count` example
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
        let fattr = FieldAttrModifiers {
            try_count: Some(10),
            ..Default::default()
        };
        assert_eq!(<HashSet<u16>>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), HashSet::from([0x0001, 0x0002]));
        assert_eq!(bytes.remaining_len(), 0);

        // test `byte_count_outside` example
        let bytes = Bytes::new([0x00, 0x02, 0x00, 0x01, 0x00, 0x02]);
        let fattr = FieldAttrModifiers {
            byte_count_outside: Some(2),
            ..Default::default()
        };
        assert_eq!(<HashSet<u16>>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), HashSet::from([0x0001, 0x0002]));
        assert_eq!(bytes.remaining_len(), 0);
    }
}