use crate::{
    JResult, BufRead,
    BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<'de> BorrowByteDecode<'de> for &'de [u8] {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>,
                                fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_bytes(2)
    }
}