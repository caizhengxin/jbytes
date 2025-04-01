use crate::{
    JResult, BufWrite,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    types::Mark,
};


impl<'a> ByteEncode for Mark<'a> {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, _buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>,
                                                                  _fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        Ok(0)
    }
}


impl<'a> BorrowByteEncode for Mark<'a> {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, _buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>,
                                                                  _fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        Ok(0)
    }
}