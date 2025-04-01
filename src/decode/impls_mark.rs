use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    types::Mark,
};


impl<'a> ByteDecode for Mark<'a> {
    #[inline]
    fn decode_inner<I: BufRead>(_input: &I, _cattr: Option<&ContainerAttrModifiers>,
                                               _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Mark::default())
    }
}


impl<'de, 'a> BorrowByteDecode<'de> for Mark<'a> {
    #[inline]
    fn decode_inner<I: BufRead>(_input: &'de I, _cattr: Option<&ContainerAttrModifiers>,
                                    _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(Mark::default())
    }
}