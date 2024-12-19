#![allow(clippy::needless_borrow)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "jbytes_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jbytes_derive;

#[cfg(feature = "jbytes_derive")]
pub use jbytes_derive::ByteDecode;


pub mod errors;
pub mod buffer;
pub mod bytes;
pub mod traits;
pub mod std;
mod impls;

pub mod modifiers;
pub mod byteorder;
pub mod decode;
pub mod encode;

pub use buffer::Buffer;
pub use bytes::Bytes;
pub use traits::{BufRead, BufWrite};
pub use errors::{JResult, ErrorKind, make_error};

pub use modifiers::{ContainerAttrModifiers, FieldAttrModifiers,  get_byteorder};
pub use byteorder::ByteOrder;
pub use decode::{ByteDecode, BorrowByteDecode};
pub use encode::{ByteEncode, BorrowByteEncode};

// use crate::std::*;


#[inline]
pub fn decode<'a, I: BufRead, T: ByteDecode>(input: &'a mut I) -> JResult<T> {
    T::decode(input, None, None)
}


#[inline]
pub fn encode<T: ByteEncode>(t: T) -> JResult<Buffer> {
    let mut buf = Buffer::new(Vec::new());

    t.encode(&mut buf, None, None)?;

    Ok(buf)
}