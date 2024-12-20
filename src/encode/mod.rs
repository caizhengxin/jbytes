mod impls_bool;

// use crate::std::*;
use crate::{
    JResult, BufWriteMut,
    ContainerAttrModifiers, FieldAttrModifiers,
};


pub trait ByteEncode {
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;

    #[inline]
    fn encode<T: BufWriteMut>(&self, input: &mut T) -> JResult<usize> {
        self.encode_inner(input, None, None)
    }
}


pub trait BorrowByteEncode {
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;

    #[inline]
    fn encode<T: BufWriteMut>(&self, input: &mut T) -> JResult<usize> {
        self.encode_inner(input, None, None)
    }
}
