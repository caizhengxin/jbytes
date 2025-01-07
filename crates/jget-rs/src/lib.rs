//!```
//! use jget_derive::Jget;
//!
//!
//! #[derive(Debug, Jget)]
//! pub struct SimpleStructExample {
//!     #[jget(get="body.get_value():Option<u8>, body.get_data():Option<u16>")]
//!     pub body: SimpleEnumExample,
//! }
//!
//! #[derive(Debug, Jget)]
//! pub enum SimpleEnumExample {
//!     Read {
//!         #[jget(get_option)]
//!         value: u8,
//!     },
//!     Write {
//!         #[jget(get_option)]
//!         value: u8,
//!         #[jget(get_option)]
//!         data: u16,
//!     }
//! }
//!
//!
//!
//! fn main() {
//!     let value = SimpleStructExample{ body: SimpleEnumExample::Read { value: 1 } };
//!     assert_eq!(value.get_value(), Some(1));
//!     assert_eq!(value.get_data(), None);
//!
//!     let value = SimpleStructExample { body: SimpleEnumExample::Write { value: 1, data: 2 }};
//!     assert_eq!(value.get_value(), Some(1));
//!     assert_eq!(value.get_data(), Some(2));
//! }
//! ```
// #![cfg_attr(feature = "nightly", feature(let_chains))]

#[cfg(feature = "jget_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jget_derive;

#[cfg(feature = "jget_derive")]
pub use jget_derive::Jget;
