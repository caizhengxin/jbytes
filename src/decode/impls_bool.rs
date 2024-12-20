use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl ByteDecode for bool {
    fn decode_inner<T: BufRead>(input: &T, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_bool()
    }
}


impl<'de> BorrowByteDecode<'de> for bool {
    fn decode_inner<'da: 'de, T: BufRead>(input: &'da T, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_bool()
    }
}