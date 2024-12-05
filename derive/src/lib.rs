// #![feature(let_chains)]
extern crate proc_macro;

mod jbyte;

use jbyte::attribute::ContainerAttributes;
use jbyte::derive_enum;
use jbyte::derive_struct;

use proc_macro::TokenStream;
use virtue::prelude::*;


#[proc_macro_derive(ByteDecode, attributes(jbyte))]
pub fn derive_bytedecode(input: TokenStream) -> TokenStream {
    derive_bytedecode_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_bytedecode_inner(input: TokenStream) -> Result<TokenStream> {
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
            }.generate_bytedecode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_bytedecode(&mut generator)?;
        }
    }

    generator.export_to_file("jbyte", "ByteDecode");
    generator.finish()
}
