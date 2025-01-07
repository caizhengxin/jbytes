//! ```
//! use jdefault_derive::Jdefault;
//!
//!
//! #[derive(Debug, PartialEq, Eq, Jdefault)]
//! pub struct StructExample<'a> {
//!     pub value1: u16,
//!     #[jd(default=18)]
//!     pub value2: u16,
//!     #[jd(default="\"jankincai\".to_string()")]
//!     pub value3: String,
//!     #[jd(default="\"jankincai\"")]
//!     pub value4: &'a str,
//!     #[jd(default=b"\x00\x01\x02")]
//!     pub value5: &'a [u8],
//!     pub body: StructExampleBody,
//! }
//!
//!
//! #[derive(Debug, PartialEq, Eq, Jdefault)]
//! pub struct StructExampleBody {
//!     #[jd(default=1)]
//!     pub value: u16,
//! }
//!
//!
//! fn main() {
//!     let value = StructExample::default();
//!
//!     assert_eq!(value, StructExample {
//!         value1: 0,
//!         value2: 18,
//!         value3: "jankincai".to_string(),
//!         value4: "jankincai",
//!         value5: b"\x00\x01\x02",
//!         body: StructExampleBody {
//!             value: 1,
//!         }
//!     });
//! }
//! ```
// 
extern crate proc_macro;

mod jdefault;

use jdefault::attribute::ContainerAttributes;
use jdefault::derive_enum;
use jdefault::derive_struct;

use proc_macro::TokenStream;
use virtue::prelude::*;


#[proc_macro_derive(Jdefault, attributes(jd, jbytes))]
pub fn derive_jdefault(input: TokenStream) -> TokenStream {
    derive_jdefault_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_jdefault_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
                attributes,
            }.generate_jdefault(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_jdefault(&mut generator)?;
        }
    }

    generator.export_to_file("jdefault", "Jdefault");
    generator.finish()
}
