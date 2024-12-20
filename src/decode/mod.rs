mod impls_bool;
mod impls_bytes;
mod impls_tuple;

use crate::{
    JResult,
    ContainerAttrModifiers, FieldAttrModifiers,
    BufRead
};


pub trait ByteDecode<'de> {
    fn decode<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;
}


pub trait BorrowByteDecode<'de> {
    fn decode<'da: 'de, T: BufRead>(input: &'da T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;
}
