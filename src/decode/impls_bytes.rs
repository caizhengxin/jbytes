use crate::{
    JResult, BufRead,
    BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<'de> BorrowByteDecode<'de> for &'de [u8] {
    fn decode_inner<'da: 'de, T: BufRead>(input: &'da T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_bytes(2)
    }
}