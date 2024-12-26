mod impls_bool;
mod impls_int;
mod impls_float;
mod impls_char;
mod impls_string;
mod impls_tuple;
mod impls_array;
mod impls_bytes;
mod impls_vec;
mod impls_other;
mod impls_option;
mod impls_ipaddress;
mod impls_macaddress;
mod impls_netaddress;

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
///     fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>,
///                                                               _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
///     {
///         buffer.push_bool(*self)
///     }
/// }
/// ```
pub trait ByteEncode {
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                              fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;

    #[inline]
    fn encode<B: BufWriteMut>(&self, buffer: &mut B) -> JResult<usize> {
        self.encode_inner(buffer, None, None)
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
///     fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>,
///                                                               _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
///     {
///         buffer.push_bool(*self)
///     }
/// }
/// ```
pub trait BorrowByteEncode {
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                              fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;

    #[inline]
    fn encode<B: BufWriteMut>(&self, buffer: &mut B) -> JResult<usize> {
        self.encode_inner(buffer, None, None)
    }
}
