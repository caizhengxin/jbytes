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
pub mod byteorder;
pub mod buffer;
pub mod bytes;
pub mod traits;
pub mod std;
mod impls;

pub use byteorder::ByteOrder;
pub use buffer::Buffer;
pub use bytes::Bytes;
pub use traits::{BufRead, BufWrite};


pub trait ByteDecode {
    fn hello_world(&self) -> String;
}
