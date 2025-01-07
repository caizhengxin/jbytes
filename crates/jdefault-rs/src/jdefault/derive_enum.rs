#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};
use super::derive_struct::generate_struct_body;


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

    pub fn generate_jdefault(&self, generator: &mut Generator) -> Result<()> {
        generator
            .impl_for("Default")
            .generate_fn("default")
            // .with_self_arg(FnSelfArg::RefSelf)
            // .with_arg("args_name", "args_type")
            .with_return_type("Self")
            .body(|fn_body| {
                // self.attributes

                // pub enum Example {
                //     Value1,
                //     Value2(u8),
                //     Value3 {
                //         value1: u8,
                //         value2: u8,
                //     }
                // }

                let mut default_bool = false;

                for (mut _variant_index, variant) in self.iter_fields() {
                    let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                    if attributes.default_bool {
                        generate_struct_body(fn_body, &variant.fields, format!("::{}", variant.name))?;
                        default_bool = true;
                    }
                    else if let Some(default_value) = attributes.default_value {
                        fn_body.push_parsed(format!("Self::{}({default_value})", variant.name))?;
                        default_bool = true;
                    }
                }

                if !default_bool {
                    return Err(Error::custom("The enumeration default value is not set."));
                }

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
