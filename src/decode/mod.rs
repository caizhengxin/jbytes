mod impls_bool;
mod impls_int;
mod impls_float;
mod impls_bytes;
mod impls_tuple;
mod impls_array;
mod impls_vec;
mod impls_other;
mod impls_option;

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
///     fn decode_inner<I: BufRead>(input: &I, _cattr: Option<&ContainerAttrModifiers>,
///                                            _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
///     where 
///         Self: Sized
///     {
///         input.take_bool()
///     }
/// }
/// ```
pub trait ByteDecode {
    fn decode_inner<I: BufRead>(input: &I, cattr: Option<&ContainerAttrModifiers>,
                                           fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;

    #[inline]
    fn decode<I: BufRead>(input: &I) -> JResult<Self>
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
///     fn decode_inner<I: BufRead>(input: &'de I, _cattr: Option<&ContainerAttrModifiers>,
///                                                          _fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
///     where 
///         Self: Sized
///     {
///         input.take_bool()
///     }
/// }
/// ```
pub trait BorrowByteDecode<'de> {
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>,
                                fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    ;

    #[inline]
    fn decode<I: BufRead>(input: &'de I) -> JResult<Self>
    where 
        Self: Sized
    {
        Self::decode_inner(input, None, None)
    }
}
