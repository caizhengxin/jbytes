mod impls_bool;
mod impls_int;
mod impls_float;
mod impls_tuple;

// use crate::std::*;
use crate::{
    JResult, BufWriteMut,
    ContainerAttrModifiers, FieldAttrModifiers,
};


/// This is bytes encoding trait. 
/// 
/// # Example
/// 
/// ```no_test
/// use jbytes::{
///     JResult, BufWriteMut,
///     ByteDecode, BorrowByteDecode,
///     ContainerAttrModifiers, FieldAttrModifiers,
/// };
/// 
/// 
/// impl ByteEncode for bool {
///     fn encode_inner<T: BufWriteMut>(&self, input: &mut T, _cattr: Option<&ContainerAttrModifiers>,
///                                                               _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
///     {
///         input.push_bool(*self)
///     }
/// }
/// ```
pub trait ByteEncode {
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>,
                                                              fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;

    #[inline]
    fn encode<T: BufWriteMut>(&self, input: &mut T) -> JResult<usize> {
        self.encode_inner(input, None, None)
    }
}


/// This is bytes encoding trait of borrow type. 
/// 
/// # Example
/// 
/// ```no_test
/// use jbytes::{
///     JResult, BufWriteMut,
///     ByteDecode, BorrowByteDecode,
///     ContainerAttrModifiers, FieldAttrModifiers,
/// };
/// 
/// 
/// impl BorrowByteEncode for bool {
///     fn encode_inner<T: BufWriteMut>(&self, input: &mut T, _cattr: Option<&ContainerAttrModifiers>,
///                                                               _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
///     {
///         input.push_bool(*self)
///     }
/// }
/// ```
pub trait BorrowByteEncode {
    fn encode_inner<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>,
                                                              fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;

    #[inline]
    fn encode<T: BufWriteMut>(&self, input: &mut T) -> JResult<usize> {
        self.encode_inner(input, None, None)
    }
}
