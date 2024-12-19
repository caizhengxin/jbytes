mod impls_bool;

use crate::{
    JResult,
    ContainerAttrModifiers, FieldAttrModifiers,
    BufRead,
};


pub trait ByteDecode {
    fn decode<T: BufRead>(input: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;
}


pub trait BorrowByteDecode<'de> {
    fn decode<'da: 'de, T: BufRead>(input: &'da mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;
}
