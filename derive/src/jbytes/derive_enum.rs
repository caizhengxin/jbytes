#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};


#[allow(dead_code)]
pub(crate) struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
    pub attributes: ContainerAttributes,
}


impl DeriveEnum {
    fn iter_fields(&self) -> EnumVariantIterator {
        EnumVariantIterator {
            idx: 0,
            variants: &self.variants,
            curruent_idx: 0,
        }
    }

    // pub fn generate_enum_to_string(&self, generator: &mut Generator) -> Result<()> {
    //     generator
    //         .impl_for("ToString")
    //         .generate_fn("to_string")
    //         .with_self_arg(FnSelfArg::RefSelf)
    //         .with_return_type("String")
    //         .body(|fn_builder| {
    //             fn_builder.push_parsed("match self")?;

    //             // Brace = {...}
    //             // Parenthesis = (...)
    //             fn_builder.group(Delimiter::Brace, |variant_case| {
    //                 for (mut _variant_index, variant) in self.iter_fields() {
    //                     variant_case.push_parsed(format!("Self::{}", &variant.name))?;
    //                     variant_case.puncts("=>");

    //                     let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

    //                     if let Some(name) = attributes.name {
    //                         variant_case.push_parsed(format!("\"{}\".to_string(),", &name))?;
    //                     }
    //                     else {
    //                         variant_case.push_parsed(format!("\"{}\".to_string(),", &variant.name))?;
    //                     }
    //                 }

    //                 Ok(())
    //             })?;

    //             Ok(())
    //         })?;

    //     Ok(())
    // }

    // pub fn generate_enum_from_str(&self, generator: &mut Generator) -> Result<()> {
    //     let mut generator = generator.impl_for("std::str::FromStr");
    //         generator.impl_type("Err", "enum_type::FromStrParseError")?;
    //         generator.generate_fn("from_str")
    //         .with_arg("s", "&str")
    //         .with_return_type("Result<Self, enum_type::FromStrParseError>")
    //         .body(|fn_builder| {
    //             fn_builder.push_parsed("match s")?;

    //             // Brace = {...}
    //             // Parenthesis = (...)
    //             fn_builder.group(Delimiter::Brace, |variant_case| {
    //                 for (mut _variant_index, variant) in self.iter_fields() {
    //                     let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

    //                     if let Some(name) = attributes.name {
    //                         variant_case.push_parsed(format!("\"{name}\""))?;
    //                     }
    //                     else {
    //                         variant_case.push_parsed(format!("\"{}\"", &variant.name))?;
    //                     }

    //                     variant_case.puncts("=>");

    //                     variant_case.push_parsed(format!("Ok(Self::{}),", &variant.name))?;
    //                 }

    //                 variant_case.push_parsed("_ => Err(enum_type::FromStrParseError::InvalidStr(s.to_string()))")?;

    //                 Ok(())
    //             })?;

    //             Ok(())
    //         })?;

    //     Ok(())
    // }

    // pub fn generate_enum_string(&self, generator: &mut Generator) -> Result<()> {
    //     self.generate_enum_to_string(generator)?;
    //     self.generate_enum_from_str(generator)?;

    //     Ok(())
    // }

    pub fn generate_bytedecode(&self, generator: &mut Generator) -> Result<()> {
        let crate_name = format!("{}::ByteDecode", self.attributes.crate_name);        

        generator
            .impl_for(crate_name)
            .generate_fn("hello_world")
            .with_self_arg(FnSelfArg::RefSelf)
            // .with_arg("args_name", "args_type")
            .with_return_type("String")
            .body(|fn_body| {
                // self.attributes

                fn_body.push_parsed("\"hello, world!\".to_string()")?;

                Ok(())
            })?;

        Ok(())
    }
}


struct EnumVariantIterator<'a> {
    variants: &'a [EnumVariant],
    idx: usize,
    curruent_idx: isize,
}


impl<'a> Iterator for EnumVariantIterator<'a> {
    type Item = (TokenTree, &'a EnumVariant);

    fn next(&mut self) -> Option<Self::Item> {
        // let mut idx = self.idx;
        let variant = self.variants.get(self.idx)?;

        if let Some(value) = &variant.value {
            // Literal
            let val_string = value.to_string();

            if val_string.starts_with("0x") {
                self.curruent_idx = isize::from_str_radix(&val_string[2..], 16).unwrap();
            }
            else {
                self.curruent_idx = val_string.parse::<isize>().unwrap();
            }
        }

        let tokens = TokenTree::Literal(Literal::isize_suffixed(self.curruent_idx));

        self.curruent_idx += 1;
        self.idx += 1;

        Some((tokens, variant))
    }
}
