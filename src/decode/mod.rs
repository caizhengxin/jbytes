mod impls_bool;
mod impls_bytes;
mod impls_tuple;

use crate::{
    JResult,
    ContainerAttrModifiers, FieldAttrModifiers,
    BufRead
};


pub trait ByteDecode {
    fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;

    #[inline]
    fn decode<T: BufRead>(input: &T) -> JResult<Self>
    where 
        Self: Sized
    {
        Self::decode_inner(input, None, None)
    }
}


pub trait BorrowByteDecode<'de> {
    fn decode_inner<'da: 'de, T: BufRead>(input: &'da T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;

    fn decode<'da: 'de, T: BufRead>(input: &'da T) -> JResult<Self>
    where 
        Self: Sized
    {
        Self::decode_inner(input, None, None)
    }
}
