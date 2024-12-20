mod impls_bool;

// use crate::std::*;
use crate::{
    JResult, BufWriteMut,
    ContainerAttrModifiers, FieldAttrModifiers,
};


pub trait ByteEncode {
    fn encode<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;
}


pub trait BorrowByteEncode {
    fn encode<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;
}
