mod impls_bool;

// use crate::std::*;
use crate::{
    JResult,
    ContainerAttrModifiers, FieldAttrModifiers,
    traits::BufWrite,
};


pub trait ByteEncode {
    fn encode<T: BufWrite>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;
}


pub trait BorrowByteEncode {
    fn encode<T: BufWrite>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;
}
