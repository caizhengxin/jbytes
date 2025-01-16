//!
//! ```
//! use jbytes::{ByteEncode, ByteDecode};
//! use jbytes_derive::{ByteEncode, ByteDecode};
//!
//!
//! #[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
//! pub struct SimpleExample {
//!     pub length: u16,
//!     #[jbytes(length="length")]
//!     pub value: String,
//!     pub cmd: u8,
//!     #[jbytes(branch="cmd")]
//!     pub body: SimpleExampleBody,
//! }
//! 
//! 
//! #[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
//! pub enum SimpleExampleBody {
//!     #[jbytes(branch_value=1)]
//!     Read {
//!         address: u8,
//!     },
//!     Write {
//!         address: u8,
//!         value: [u8; 3],
//!     },
//!     #[jbytes(branch_default)]
//!     Unknown, 
//! }
//! 
//! 
//! fn main() {
//!     let input = b"\x00\x03\x31\x32\x33\x01\x05";
//!     let value: SimpleExample = jbytes::decode(input).unwrap();
//!     assert_eq!(value, SimpleExample { length: 3, value: "123".to_string(), cmd: 1, body: SimpleExampleBody::Read { address: 5 } });
//!     assert_eq!(*jbytes::encode(value).unwrap(), input);
//! }
//! ```
#![allow(clippy::needless_borrow)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "jbytes_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jbytes_derive;

#[cfg(feature = "jbytes_derive")]
pub use jbytes_derive::{ByteDecode, ByteEncode, BorrowByteDecode, BorrowByteEncode};


#[cfg(feature = "jdefault_derive")]
extern crate jdefault_derive;
#[cfg(feature = "jdefault_derive")]
pub use jdefault_derive::Jdefault;


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
pub use bytes::{Bytes, ToBytes};
pub use buf_mut_traits::{BufReadMut, BufWriteMut};
pub use buf_traits::{BufRead, BufWrite};
pub use errors::{JResult, ErrorKind, make_error};

pub use modifiers::{ContainerAttrModifiers, FieldAttrModifiers,  get_byteorder};
pub use byteorder::ByteOrder;
pub use decode::{ByteDecode, BorrowByteDecode};
pub use encode::{ByteEncode, BorrowByteEncode};

pub mod prelude {
    pub use crate::buffer::Buffer;
    pub use crate::bytes::{Bytes, ToBytes};
    pub use crate::buf_mut_traits::{BufReadMut, BufWriteMut};
    pub use crate::buf_traits::{BufRead, BufWrite};
    pub use crate::errors::{JResult, ErrorKind, make_error};
    
    pub use crate::modifiers::{ContainerAttrModifiers, FieldAttrModifiers,  get_byteorder};
    pub use crate::byteorder::ByteOrder;
    pub use crate::decode::{ByteDecode, BorrowByteDecode};
    pub use crate::encode::{ByteEncode, BorrowByteEncode};

    pub use crate::types::{MacAddress, NetAddress, HexString, HexBytes};

    #[cfg(feature = "jbytes_derive")]
    pub use jbytes_derive::{ByteDecode, ByteEncode, BorrowByteDecode, BorrowByteEncode};    

    #[cfg(feature = "jdefault_derive")]
    pub use jdefault_derive::Jdefault;
}


/// This is a decode function of byte stream.
/// 
/// # Example
/// 
/// ```
/// use jbytes_derive::{ByteEncode, ByteDecode};
///
///
/// #[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
/// pub struct SimpleExample {
///    pub length: u16,
///    #[jbytes(length="length")]
///    pub value: String,
///    pub cmd: u8,
///    #[jbytes(branch="cmd")]
///    pub body: SimpleExampleBody,
/// }
///
///
/// #[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
/// pub enum SimpleExampleBody {
///     #[jbytes(branch_value=1)]
///     Read {
///         address: u8,
///     },
///     Write {
///         address: u8,
///         value: [u8; 3],
///     },
///     #[jbytes(branch_default)]
///     Unknown, 
/// }
///
///
/// fn main() {
///     let input = b"\x00\x03\x31\x32\x33\x01\x05";
///     let value: SimpleExample = jbytes::decode(input).unwrap();
///     assert_eq!(value, SimpleExample { length: 3, value: "123".to_string(), cmd: 1, body: SimpleExampleBody::Read { address: 5 } });
///     assert_eq!(*jbytes::encode(value).unwrap(), input);
/// }
/// ```
#[inline]
pub fn decode<I: AsRef<[u8]>, T: ByteDecode>(input: I) -> JResult<T> {
    T::decode_inner(&Bytes::new(input), None, None)
}


/// This is a decode function of byte stream.
/// 
/// # Example
/// 
/// ```rust
/// use jbytes_derive::{BorrowByteEncode, BorrowByteDecode};
/// use jbytes::prelude::*;
///
///
/// #[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
/// pub struct Http<'a> {
///     #[jbytes(linend=b"\x20")]
///     pub method: &'a str,
///     #[jbytes(linend=b"\x20")]
///     pub uri: &'a str,
///     #[jbytes(linend=b"\r\n")]
///     pub version: &'a str,
///     #[jbytes(try_count=20)]
///     pub headers: Vec<HttpHeader<'a>>,
/// }
///
///
/// #[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
/// pub struct HttpHeader<'a> {
///     #[jbytes(linend=b": ")]
///     pub key: &'a str,
///     #[jbytes(linend=b"\r\n")]
///     pub value: &'a str,
/// }
///
///
/// fn main() {
///     // decode
///     let data = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
///     let bytes = Bytes::new(data);
///     let value: Http = jbytes::decode_borrow(&bytes).unwrap();
///     println!("{value:?}");

///     // encode
///     assert_eq!(*jbytes::encode_borrow(value).unwrap(), data);
/// }
/// ```
#[inline]
pub fn decode_borrow<'de, I: AsRef<[u8]>, T: BorrowByteDecode<'de>>(input: &'de Bytes<I>) -> JResult<T> {
    T::decode_inner(input, None, None)
}


#[inline]
pub fn decode_borrow2<'de, I: BufRead, T: BorrowByteDecode<'de>>(input: &'de I) -> JResult<T> {
    T::decode_inner(input, None, None)
}


/// This is a encode function of byte stream.
/// 
/// # Example
/// 
/// ```
/// use jbytes_derive::{ByteEncode, ByteDecode};
///
///
/// #[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
/// pub struct SimpleExample {
///    pub length: u16,
///    #[jbytes(length="length")]
///    pub value: String,
///    pub cmd: u8,
///    #[jbytes(branch="cmd")]
///    pub body: SimpleExampleBody,
/// }
///
///
/// #[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
/// pub enum SimpleExampleBody {
///     #[jbytes(branch_value=1)]
///     Read {
///         address: u8,
///     },
///     Write {
///         address: u8,
///         value: [u8; 3],
///     },
///     #[jbytes(branch_default)]
///     Unknown, 
/// }
///
///
/// fn main() {
///     let input = b"\x00\x03\x31\x32\x33\x01\x05";
///     let value: SimpleExample = jbytes::decode(input).unwrap();
///     assert_eq!(value, SimpleExample { length: 3, value: "123".to_string(), cmd: 1, body: SimpleExampleBody::Read { address: 5 } });
///     assert_eq!(*jbytes::encode(value).unwrap(), input);
/// }
/// ```
#[inline]
pub fn encode<T: ByteEncode>(t: T) -> JResult<Buffer> {
    let mut buf = Buffer::new();

    t.encode_inner(&mut buf, None, None)?;

    Ok(buf)
}


/// This is a encode function of byte stream.
/// 
/// # Example
/// 
/// ```rust
/// use jbytes_derive::{BorrowByteEncode, BorrowByteDecode};
/// use jbytes::prelude::*;
///
///
/// #[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
/// pub struct Http<'a> {
///     #[jbytes(linend=b"\x20")]
///     pub method: &'a str,
///     #[jbytes(linend=b"\x20")]
///     pub uri: &'a str,
///     #[jbytes(linend=b"\r\n")]
///     pub version: &'a str,
///     #[jbytes(try_count=20)]
///     pub headers: Vec<HttpHeader<'a>>,
/// }
///
///
/// #[derive(Debug, Default, PartialEq, Eq, BorrowByteEncode, BorrowByteDecode)]
/// pub struct HttpHeader<'a> {
///     #[jbytes(linend=b": ")]
///     pub key: &'a str,
///     #[jbytes(linend=b"\r\n")]
///     pub value: &'a str,
/// }
///
///
/// fn main() {
///     // decode
///     let data = b"GET http://www.jankincai.com/ HTTP/1.1\r\nHost: www.jankincai.com\r\nAccept-Encoding: gzip, deflate\r\n";
///     let bytes = Bytes::new(data);
///     let value: Http = jbytes::decode_borrow(&bytes).unwrap();
///     println!("{value:?}");

///     // encode
///     assert_eq!(*jbytes::encode_borrow(value).unwrap(), data);
/// }
/// ```
#[inline]
pub fn encode_borrow<T: BorrowByteEncode>(t: T) -> JResult<Buffer> {
    let mut buf = Buffer::new();

    t.encode_inner(&mut buf, None, None)?;

    Ok(buf)
}