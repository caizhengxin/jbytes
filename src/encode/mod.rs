mod impls_bool;

// use crate::std::*;
use crate::{
    JResult,
    ContainerAttrModifiers, FieldAttrModifiers,
    traits::BufWrite,
};


pub trait ByteEncode {
    fn encode<'da, 'db, 'dc, T: BufWrite>(&'da self, input: &'db mut T, cattr: Option<&'dc ContainerAttrModifiers>, fattr: Option<&'dc FieldAttrModifiers>) -> JResult<'db, usize>;
}


pub trait BorrowByteEncode {
    fn encode<'da, 'db, 'dc, T: BufWrite>(&'da self, input: &'db mut T, cattr: Option<&'dc ContainerAttrModifiers>, fattr: Option<&'dc FieldAttrModifiers>) -> JResult<'db, usize>;
}
