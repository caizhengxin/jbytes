#![allow(clippy::needless_borrow)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "jbytes_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jbytes_derive;

#[cfg(feature = "jbytes_derive")]
pub use jbytes_derive::{ByteDecode, ByteDecode, BorrowByteDecode, BorrowByteEncode};


pub mod errors;
pub mod buffer;
pub mod bytes;
pub mod buf_mut_traits;
pub mod buf_traits;
pub mod std;
mod impls;

pub mod modifiers;
pub mod byteorder;
pub mod decode;
pub mod encode;
pub mod types;

pub use buffer::Buffer;
pub use bytes::Bytes;
pub use buf_mut_traits::{BufReadMut, BufWriteMut};
pub use buf_traits::{BufRead, BufWrite};
pub use errors::{JResult, ErrorKind, make_error};

pub use modifiers::{ContainerAttrModifiers, FieldAttrModifiers,  get_byteorder};
pub use byteorder::ByteOrder;
pub use decode::{ByteDecode, BorrowByteDecode};
pub use encode::{ByteEncode, BorrowByteEncode};

pub mod prelude {
    pub use crate::buffer::Buffer;
    pub use crate::bytes::Bytes;
    pub use crate::buf_mut_traits::{BufReadMut, BufWriteMut};
    pub use crate::buf_traits::{BufRead, BufWrite};
    pub use crate::errors::{JResult, ErrorKind, make_error};
    
    pub use crate::modifiers::{ContainerAttrModifiers, FieldAttrModifiers,  get_byteorder};
    pub use crate::byteorder::ByteOrder;
    pub use crate::decode::{ByteDecode, BorrowByteDecode};
    pub use crate::encode::{ByteEncode, BorrowByteEncode};

    pub use crate::types::{MacAddress, NetAddress, HexString, HexBytes};

    #[cfg(feature = "jbytes_derive")]
    pub use jbytes_derive::{ByteDecode, ByteDecode, BorrowByteDecode, BorrowByteEncode};    
}


#[inline]
pub fn decode<I: AsRef<[u8]>, T: ByteDecode>(input: &I) -> JResult<T> {
    T::decode_inner(&Bytes::new(input), None, None)
}


#[inline]
pub fn decode_borrow<'de, I: AsRef<[u8]>, T: BorrowByteDecode<'de>>(input: &'de Bytes<I>) -> JResult<T> {
    T::decode_inner(input, None, None)
}


#[inline]
pub fn encode<T: ByteEncode>(t: T) -> JResult<Buffer> {
    let mut buf = Buffer::new();

    t.encode_inner(&mut buf, None, None)?;

    Ok(buf)
}


#[inline]
pub fn encode_borrow<T: BorrowByteEncode>(t: T) -> JResult<Buffer> {
    let mut buf = Buffer::new();

    t.encode_inner(&mut buf, None, None)?;

    Ok(buf)
}