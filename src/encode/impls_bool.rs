use crate::{
    JResult, BufWriteMut,
    ByteEncode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl ByteEncode for bool {
    fn encode<T: BufWriteMut>(&self, input: &mut T, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        input.push_bool(*self)
    }
}