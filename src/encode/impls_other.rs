use core::marker::PhantomData;
use crate::{
    JResult, BufWriteMut,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<T> ByteEncode for PhantomData<T> {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, _buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>,
                                                                  _fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        Ok(0)
    }
}


impl<T> BorrowByteEncode for PhantomData<T> {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, _buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>,
                                                                  _fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        Ok(0)
    }
}