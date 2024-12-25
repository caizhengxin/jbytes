use crate::{
    JResult, BufWriteMut,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
};
use super::impls_bytes::encode_inner;


impl ByteEncode for String {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        encode_inner(buffer, cattr, fattr, self.as_bytes())
    }
}


impl BorrowByteEncode for String {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        encode_inner(buffer, cattr, fattr, self.as_bytes())
    }
}


impl<'de> BorrowByteEncode for &'de str {
    #[inline]
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        encode_inner(buffer, cattr, fattr, self.as_bytes())
    }
}
