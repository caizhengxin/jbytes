use core::str;
use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    ErrorKind, make_error,
};
use super::impls_bytes::find_subsequence;


impl ByteDecode for String {
    #[inline]
    fn decode_inner<I: BufRead>(input: &I, cattr: Option<&ContainerAttrModifiers>,
                                               fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let value = find_subsequence(input, cattr, fattr)?;

        match str::from_utf8(value) {
            Ok(v) => Ok(v.to_string()),
            Err(_e) => Err(make_error(input.get_position(), ErrorKind::Fail))
        }
    }
}


impl<'de> BorrowByteDecode<'de> for String {
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let value = find_subsequence(input, cattr, fattr)?;

        match str::from_utf8(value) {
            Ok(v) => Ok(v.to_string()),
            Err(_e) => Err(make_error(input.get_position(), ErrorKind::Fail))
        }
    }
}


impl<'de> BorrowByteDecode<'de> for &'de str {
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let value = find_subsequence(input, cattr, fattr)?;

        match str::from_utf8(value) {
            Ok(v) => Ok(v),
            Err(_e) => Err(make_error(input.get_position(), ErrorKind::Fail))
        }
    }
}
