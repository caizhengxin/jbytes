mod impls_bool;
mod impls_int;
mod impls_float;
mod impls_char;
mod impls_string;
mod impls_tuple;
mod impls_array;
mod impls_bytes;
mod impls_vec;
#[cfg(feature = "std")]
mod impls_hashmap;
mod impls_other;
mod impls_option;
mod impls_ipaddress;
mod impls_macaddress;
mod impls_netaddress;

// use crate::std::*;
use crate::{
    JResult, BufWrite,
    ContainerAttrModifiers, FieldAttrModifiers,
};


/// This is bytes encoding trait. 
/// 
/// # Example
/// 
/// ```no_test
/// use jbytes::{
///     JResult, BufWrite,
///     ByteDecode, BorrowByteDecode,
///     ContainerAttrModifiers, FieldAttrModifiers,
/// };
/// 
/// 
/// impl ByteEncode for bool {
///     fn encode_inner<B: BufWrite>(&self, buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>,
///                                                               _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
///     {
///         buffer.push_bool(*self)
///     }
/// }
/// ```
pub trait ByteEncode {
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                              fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;

    #[inline]
    fn encode<B: BufWrite>(&self, buffer: &mut B) -> JResult<usize> {
        self.encode_inner(buffer, None, None)
    }
}


/// This is bytes encoding trait of borrow type. 
/// 
/// # Example
/// 
/// ```no_test
/// use jbytes::{
///     JResult, BufWrite,
///     ByteDecode, BorrowByteDecode,
///     ContainerAttrModifiers, FieldAttrModifiers,
/// };
/// 
/// 
/// impl BorrowByteEncode for bool {
///     fn encode_inner<B: BufWrite>(&self, buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>,
///                                                               _fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
///     {
///         buffer.push_bool(*self)
///     }
/// }
/// ```
pub trait BorrowByteEncode {
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                              fattr: Option<&FieldAttrModifiers>) -> JResult<usize>;

    #[inline]
    fn encode<B: BufWrite>(&self, buffer: &mut B) -> JResult<usize> {
        self.encode_inner(buffer, None, None)
    }
}


#[allow(clippy::if_same_then_else)]
#[inline]
fn push_count_and_try_count<B: BufWrite>(buffer: &mut B, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>, value: usize) -> JResult<usize> {
    let mut r_nbytes = 0;

    if let Some(fr) = fattr {
        if let Some(byte_count) = fr.byte_count_outside {
            r_nbytes += buffer.push_byteorder_uint(value as u64, byte_count, crate::get_byteorder(cattr, fattr))?;
        }
        else if fr.count.is_some() { }
        else if fr.try_count.is_some() { }
        else {
            r_nbytes += buffer.push_u8(value as u8)?;
        }
    }
    else {
        r_nbytes += buffer.push_u8(value as u8)?;
    }

    Ok(r_nbytes)
}