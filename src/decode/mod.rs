mod impls_bool;
mod impls_int;
mod impls_float;
mod impls_char;
mod impls_string;
mod impls_bytes;
mod impls_tuple;
mod impls_array;
mod impls_vec;
#[cfg(feature = "std")]
mod impls_hashmap;
#[cfg(feature = "std")]
mod impls_hashset;
mod impls_hex;
mod impls_other;
mod impls_option;
mod impls_ipaddress;
mod impls_macaddress;
mod impls_netaddress;

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


#[inline]
fn get_count_and_try_count<I: BufRead>(input: &I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<(usize, Option<usize>)>
{
    let mut count = 0;
    let mut try_count = None;

    if let Some(fr) = fattr {
        if let Some(count_tmp) = fr.count {
            count = count_tmp;
        }
        else if fr.try_count.is_some() {
            try_count = fr.try_count;
        } else if let Some(byte_count) = fr.byte_count_outside {
            count = input.take_byteorder_uint(byte_count, crate::get_byteorder(cattr, fattr))? as usize;
        }
        else {
            // default: take 1 byte
            count = input.take_u8()? as usize;
        }
    } else {
        // default: take 1 byte
        count = input.take_u8()? as usize;
    }

    Ok((count, try_count))
}