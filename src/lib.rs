#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "jbyte_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jbyte_derive;

#[cfg(feature = "jbyte_derive")]
pub use jbyte_derive::ByteDecode;


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
