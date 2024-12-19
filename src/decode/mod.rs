mod impls_bool;

use crate::{
    JResult,
    ContainerAttrModifiers, FieldAttrModifiers,
    BufRead,
};


pub trait ByteDecode {
    fn decode<'da, 'db, T: BufRead>(input: &'da mut T, cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<'da, Self>
    where 
        Self: Sized
    ;
}


pub trait BorrowByteDecode<'de> {
    fn decode<'da: 'de, 'db, T: BufRead>(input: &'da mut T, cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<'da, Self>
    where 
        Self: Sized
    ;
}
