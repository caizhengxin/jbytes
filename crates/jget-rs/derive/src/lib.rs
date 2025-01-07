// #![cfg_attr(feature = "nightly", feature(let_chains))]
extern crate proc_macro;

mod jget;

use jget::attribute::ContainerAttributes;
use jget::derive_enum;
use jget::derive_struct;

use proc_macro::TokenStream;
use virtue::prelude::*;


#[proc_macro_derive(Jget, attributes(jget))]
pub fn derive_jget(input: TokenStream) -> TokenStream {
    derive_jget_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_jget_inner(input: TokenStream) -> Result<TokenStream> {
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
            }.generate_jget(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_jget(&mut generator)?;
        }
    }

    generator.export_to_file("jget", "Jget");
    generator.finish()
}
