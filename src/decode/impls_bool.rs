use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<'de> ByteDecode<'de> for bool {
    fn decode<T: BufRead>(input: &'de T, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        input.take_bool()
    }
}


// impl<'de> BorrowByteDecode<'de> for bool {
//     fn decode<'da: 'de, T: BufRead>(input: &'da T, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
//     where 
//         Self: Sized
//     {
//         input.take_bool()
//     }
// }