use core::marker::PhantomData;
use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


impl<T> ByteDecode for PhantomData<T> {
    #[inline]
    fn decode_inner<I: BufRead>(_input: &I, _cattr: Option<&ContainerAttrModifiers>,
                                               _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(PhantomData)
    }
}


impl<'de, T> BorrowByteDecode<'de> for PhantomData<T> {
    #[inline]
    fn decode_inner<I: BufRead>(_input: &'de I, _cattr: Option<&ContainerAttrModifiers>,
                                    _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        Ok(PhantomData)
    }
}