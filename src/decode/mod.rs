mod impls_bool;
mod impls_float;
mod impls_bytes;
mod impls_tuple;

use crate::{
    JResult,
    ContainerAttrModifiers, FieldAttrModifiers,
    BufRead
};


/// This is bytes decoding trait. 
/// 
/// # Example
/// 
/// ```no_test
/// use jbytes::{
///     JResult, BufRead,
///     ByteDecode, BorrowByteDecode,
///     ContainerAttrModifiers, FieldAttrModifiers,
/// };
/// 
/// 
/// impl ByteDecode for bool {
///     fn decode_inner<T: BufRead>(input: &T, _cattr: Option<&ContainerAttrModifiers>,
///                                            _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
///     where 
///         Self: Sized
///     {
///         input.take_bool()
///     }
/// }
/// ```
pub trait ByteDecode {
    fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>,
                                           fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;

    #[inline]
    fn decode<T: BufRead>(input: &T) -> JResult<Self>
    where 
        Self: Sized
    {
        Self::decode_inner(input, None, None)
    }
}


/// This is bytes decoding trait of borrow type.
/// 
/// # Example
/// 
/// ```no_test
/// use jbytes::{
///     JResult, BufRead,
///     ByteDecode, BorrowByteDecode,
///     ContainerAttrModifiers, FieldAttrModifiers,
/// };
/// 
/// 
/// impl<'de> BorrowByteDecode<'de> for bool {
///     fn decode_inner<'da: 'de, T: BufRead>(input: &'da T, _cattr: Option<&ContainerAttrModifiers>,
///                                                          _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
///     where 
///         Self: Sized
///     {
///         input.take_bool()
///     }
/// }
/// ```
pub trait BorrowByteDecode<'de> {
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>,
                                fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;

    #[inline]
    fn decode<T: BufRead>(input: &'de T) -> JResult<Self>
    where 
        Self: Sized
    {
        Self::decode_inner(input, None, None)
    }
}
