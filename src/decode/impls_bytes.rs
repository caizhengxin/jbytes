use crate::{
    JResult, BufRead,
    ByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<'de> ByteDecode<'de> for &'de [u8] {
    fn decode<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_bytes(2)
    }
}


// impl<'de> BorrowByteDecode<'de> for &'de [u8] {
//     fn decode<'da: 'de, T: BufRead>(input: &'da T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
//     where 
//         Self: Sized
//     {
//         input.take_bytes(2)
//     }
// }