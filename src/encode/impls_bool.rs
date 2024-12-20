use crate::{
    JResult, BufWriteMut,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl ByteEncode for bool {
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        input.push_bool(*self)
    }
}


impl BorrowByteEncode for bool {
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        input.push_bool(*self)
    }
}