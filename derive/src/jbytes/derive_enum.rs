#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};
use super::parse::AttrValue;
use super::derive_struct::{generate_decode_struct_body, generate_decode_return};
use super::encode::{generate_encode_body, generate_encode_body2};
use super::decode::generate_decode_body2;


#[allow(dead_code)]
pub(crate) struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
    pub attributes: ContainerAttributes,
    pub lifetimes: Option<String>,
}


impl DeriveEnum {
    fn iter_fields(&self) -> EnumVariantIterator {
        EnumVariantIterator {
            idx: 0,
            variants: &self.variants,
            curruent_idx: 0,
        }
    }

    pub fn generate_decode(&self, generator: &mut Generator) -> Result<()> {
        let crate_name = "jbytes::ByteDecode";

        generator
            .impl_for(crate_name)
            .generate_fn("decode_inner")
            .with_generic_deps("I", ["jbytes::BufRead"])
            .with_arg("input", "&I")
            .with_arg("cattr", "Option<&jbytes::ContainerAttrModifiers>")
            .with_arg("fattr", "Option<&jbytes::FieldAttrModifiers>")
            .with_return_type("jbytes::JResult<Self>")
            .body(|fn_body| {
                self.generate_decode_body(crate_name, fn_body)?;

                Ok(())
            })?;

        Ok(())
    }


    pub fn generate_borrow_decode(&self, generator: &mut Generator) -> Result<()> {
        let crate_name = "jbytes::BorrowByteDecode";

        let mut impl_for = if let Some(lifetimes) = &self.lifetimes {
            generator
            .impl_for(format!("{crate_name}{lifetimes}"))    
        }
        else {
            generator
            .impl_for_with_lifetimes(crate_name, ["de"])
        };

        let lifetimes = if let Some(lifetimes) = &self.lifetimes {
            lifetimes.trim_start_matches('<').trim_end_matches('>')
        }
        else { 
            "'de"
        };

        impl_for
            .generate_fn("decode_inner")
            .with_generic_deps("I", ["jbytes::BufRead"])
            .with_arg("input", format!("&{} I", lifetimes))
            .with_arg("cattr", "Option<&jbytes::ContainerAttrModifiers>")
            .with_arg("fattr", "Option<&jbytes::FieldAttrModifiers>")
            .with_return_type("jbytes::JResult<Self>")
            .body(|fn_body| {
                self.generate_decode_body(crate_name, fn_body)?;
                Ok(())
            })?;

        Ok(())
    }

    #[inline]
    fn generate_decode_body(&self, crate_name: &str, fn_body: &mut StreamBuilder) -> Result<()> {
        if let Some(func) = &self.attributes.with_decode {
            fn_body.push_parsed(format!("{func}(input, cattr, fattr)"))?;
        }
        else if let Some(func) = &self.attributes.with {
            fn_body.push_parsed(format!("{func}::decode(input, cattr, fattr)"))?;
        }
        else {
            fn_body.push_parsed(self.attributes.to_code(false))?;

            if let Some(value) = &self.attributes.get_variable_name {
                if let AttrValue::List(variable_names) = value {
                    for variable_name in variable_names {
                        let variable_name_str = variable_name.to_string();
    
                        fn_body.push_parsed(format!("
                            let mut {variable_name_str} = 0;
                            if let Some(cr) = cattr {{
                                if let Some(value) = cr.variable_name.borrow().get(&\"{variable_name_str}\".to_string()) {{
                                    {variable_name_str} = *value;
                                }}
                            }}
                        "))?;  
                    }    
                }
            }

            if self.attributes.branch_starts_with || self.attributes.branch_starts_with_untake {
                self.generate_byte_decode_body2(crate_name, fn_body)?;
            }
            else {
                self.generate_byte_decode_body(crate_name, fn_body)?;
            }
        }

        Ok(())
    }

    fn generate_byte_decode_body2(&self, crate_name: &str, fn_body: &mut StreamBuilder) -> Result<()> {
        // #[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
        // #[jbytes(branch_starts_with)]
        // pub enum EnumExample {
        //     #[jbytes(branch_value=b"read")]
        //     Read {
        //         address: u8,
        //     },
        //     #[jbytes(branch_value=b"write")]
        //     Write {
        //         address: u8,
        //         value: u16,
        //     },
        //     #[jbytes(branch_default)]
        //     Unknown,
        // }

        // let data = b"";

        // if data.starts_with(b"read") {
        //     // TODO
        // }
        // else if data.starts_with(b"write") {
        //     // TODO
        // }
        // else {

        // }

        if self.variants.is_empty() {
            fn_body.push_parsed("Ok(Self {{}})")?;
            return Ok(());
        }

        let code = "let data = input.remaining();";

        fn_body.push_parsed(code)?;

        let mut branch_default = false;

        for (variant_index, variant) in self.iter_fields() {
            let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

            let mut branch_value_tmp = None;

            if attributes.branch_default {
                fn_body.push_parsed(format!("else"))?;
                branch_default = true;
            }
            else if let Some(branch_value) = &attributes.branch_value {
                branch_value_tmp = Some(branch_value.to_string());

                if variant_index.to_string() == "0usize" {
                    fn_body.push_parsed(format!("if data.starts_with({branch_value})"))?;
                }
                else {
                    fn_body.push_parsed(format!("else if data.starts_with({branch_value})"))?;
                }
            }
            else {
                let branch_value = format!("b\"{}\"", variant.name.to_string().to_lowercase());

                fn_body.push_parsed(format!("else if data.starts_with({branch_value})"))?;

                branch_value_tmp = Some(branch_value);
            }

            if self.attributes.branch_starts_with_untake {
                branch_value_tmp = None;
            }

            fn_body.group(Delimiter::Brace, |variant_body| {
                if let Some(branch_value) = branch_value_tmp {
                    variant_body.push_parsed(format!("input.advance({branch_value}.len());"))?;
                }

                variant_body.push_parsed(attributes.to_code(true, true))?;
                generate_decode_body2(variant_body, &attributes)?;
                generate_decode_struct_body(variant_body, crate_name, &variant.fields, &self.attributes, true)?;
                generate_decode_return(variant_body, &variant.fields, Some(variant))?;        

                Ok(())
            })?;
        }

        if !branch_default {
            fn_body.push_parsed(format!("else {{
                Err(jbytes::make_error(input.get_position(), jbytes::ErrorKind::Fail))
            }}"))?;
        }
    
        Ok(())
    }

    fn generate_byte_decode_body(&self, crate_name: &str, fn_body: &mut StreamBuilder) -> Result<()> {
        // enum Example {
        //    V0,
        //    V1(u8),
        //    V2(u8, 16),
        //    #[jbytes(byteorder="LE")]
        //    V3((u8, u16)),
        //    V4 {
        //        a: u8,
        //        #[jbytes(length=2)]
        //        b: u16,
        //    }
        // }

        // match value {
        //     0 => {
        //         return Ok(Self::V0);
        //     },
        //     1 => {
        //         let v0: u8 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         return Ok(Self::V1(v0));
        //     },
        //     2 => {
        //         let v0: u8 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         let v1: u8 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         return Ok(Self::V2(v0, v1));
        //     },
        //     3 => {
        //         let v0: (u8, u16) = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         return Ok(Self::V3(v0));
        //     },
        //     4 => {
        //         let a: u8 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         let b: u16 = ByteDecode::decode(input, Some(&cattr), Some(&fattr));
        //         return Ok(Self::V4 { a, b });
        //     },
        //     _ => {

        //     },
        // }

        if self.variants.is_empty() {
            fn_body.push_parsed("Ok(Self {{}})")?;
            return Ok(());
        }
        
        let code = if let Some(branch_take_bytes) = &self.attributes.branch_take_bytes {
            let branch_take_bytes = branch_take_bytes.to_code2(false, false);
            format!("let value = input.take_bytes({branch_take_bytes})?;")
        }
        // else if let Some(byte_count) = &self.attributes.byte_count {
        //     let byte_count = byte_count.to_code2(false, false);

        //     format!("let value = input.take_byteorder_uint({byte_count}, jbytes::get_byteorder(cattr_new, fattr))? as usize;")
        // }
        else {
            format!("
                let value;
                let cr_byte_count = if let Some(cr) = cattr_new {{ cr.byte_count }} else {{ None }};

                if let Some(fr) = fattr {{
                    if let Some(branch) = fr.branch {{
                        value = (branch) as usize;
                    }}
                    else if let Some(byte_count) = fr.byte_count {{
                        value = input.take_byteorder_uint(byte_count, jbytes::get_byteorder(cattr, fattr))? as usize;
                    }}
                    else if let Some(byte_count) = cr_byte_count {{
                        value = input.take_byteorder_uint(byte_count, jbytes::get_byteorder(cattr, fattr))? as usize;
                    }}
                    else {{
                        value = input.take_u8()? as usize;
                    }}
                }}
                else if let Some(byte_count) = cr_byte_count {{
                    value = input.take_byteorder_uint(byte_count, jbytes::get_byteorder(cattr, fattr))? as usize;
                }}
                else {{
                    value = input.take_u8()? as usize;
                }}
            ")
        };

        fn_body.push_parsed(code)?;
        fn_body.push_parsed("match value")?;
        fn_body.group(Delimiter::Brace, |variant_case| {
            let mut branch_default = false;

            for (variant_index, variant) in self.iter_fields() {
                let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                if attributes.branch_default {
                    variant_case.push_parsed("_")?;
                    branch_default = true;
                }
                else if let Some(branch_bits) = &attributes.branch_bits {
                    // match value {
                    //     value if (value & 0x80) == 0x80 => {},
                    //     value if (value & 0x01) == 0x80 => {},
                    //     0x02 => {},
                    //     _ => {},
                    // }

                    if let Some(branch_bits_value) = &attributes.branch_bits_value {
                        variant_case.push_parsed(format!("value if (value & {branch_bits}) == {branch_bits_value}"))?;
                    }
                    else {
                        variant_case.push_parsed(format!("value if (value & {branch_bits}) == {branch_bits}"))?;
                    }
                }
                else if let Some(branch_value) = &attributes.branch_value {
                    variant_case.push_parsed(branch_value.to_string())?;
                }
                else if let Some(branch_range) = &attributes.branch_range {
                    variant_case.push_parsed(branch_range.to_string())?;
                }
                else {
                    variant_case.push_parsed( variant_index.to_string())?;
                }

                variant_case.puncts("=>");
                variant_case.group(Delimiter::Brace, |variant_body| {
                    variant_body.push_parsed(attributes.to_code(true, true))?;
                    generate_decode_body2(variant_body, &attributes)?;
                    generate_decode_struct_body(variant_body, crate_name, &variant.fields, &self.attributes, true)?;
                    generate_decode_return(variant_body, &variant.fields, Some(variant))?;
                    Ok(())
                })?;
            }

            if !branch_default {
                variant_case.push_parsed("_ => Err(jbytes::make_error(input.get_position(), jbytes::ErrorKind::InvalidByteLength))")?;
            }

            Ok(())
        })?;

        Ok(())
    }

    pub fn generate_encode(&self, generator: &mut Generator) -> Result<()> {
        self.generate_byte_encode_body("jbytes::ByteEncode", generator)?;
        Ok(())
    }

    pub fn generate_borrow_encode(&self, generator: &mut Generator) -> Result<()> {
        self.generate_byte_encode_body("jbytes::BorrowByteEncode", generator)?;
        Ok(())
    }

    fn generate_byte_encode_body(&self, crate_name: &str, generator: &mut Generator) -> Result<()> {
        generator
            .impl_for(crate_name)
            .generate_fn("encode_inner")
            .with_generic_deps("B", ["jbytes::BufWrite"])
            .with_self_arg(FnSelfArg::RefSelf)
            .with_arg("buffer", "&mut B")
            .with_arg("cattr", "Option<&jbytes::ContainerAttrModifiers>")
            .with_arg("fattr", "Option<&jbytes::FieldAttrModifiers>")
            .with_return_type("jbytes::JResult<usize>")
            .body(|fn_body| {
                fn_body.push_parsed("let mut r_nbytes = 0;")?;

                if let Some(func) = &self.attributes.with_encode {
                    fn_body.push_parsed(format!("r_nbytes += {func}(buffer, cattr, fattr, self)?;"))?;
                    return Ok(());
                }
                else if let Some(func) = &self.attributes.with {
                    fn_body.push_parsed(format!("r_nbytes += {func}::encode(buffer, cattr, fattr, self)?;"))?;
                    return Ok(());
                }

                // enum Example {
                //    V0,
                //    V1(u8),
                //    V2(u8, 16),
                //    #[jbytes(byteorder="LE")]
                //    V3((u8, u16)),
                //    V4 {
                //        a: u8,
                //        #[jbytes(length=2)]
                //        b: u16,
                //    }
                // }

                // match self {
                //     Self::V0 => {},
                //     Self::V1(v) => v.encode(input, Some(&cattr), Some(&fattr)),
                //     Self::V2(v1, v2) => {
                //         v1.encode(input, Some(&cattr), Some(&fattr));
                //         v2.encode(input, Some(&cattr), Some(&fattr));
                //     },
                //     Self::V3(v) => v.encode(input, Some(&cattr), Some(&fattr)),
                //     Self::V4 {a, b} => {
                //         a.encode(input, Some(&cattr), Some(&fattr));
                //         b.encode(input, Some(&cattr), Some(&fattr));
                //     }
                // }

                if let Some(value) = &self.attributes.get_variable_name {
                    if let AttrValue::List(variable_names) = value  {
                        for variable_name in variable_names {
                            let variable_name_str = variable_name.to_string();
        
                            fn_body.push_parsed(format!("
                                let mut {variable_name_str} = 0;
                                if let Some(cr) = cattr {{
                                    if let Some(value) = cr.variable_name.borrow().get(&\"{variable_name_str}\".to_string()) {{
                                        {variable_name_str} = *value;
                                    }}
                                }}
                            "))?;  
                        }    
                    }
                }        

                fn_body.push_parsed(self.attributes.to_code(true))?;
                fn_body.push_parsed("match self")?;
                fn_body.group(Delimiter::Brace, |variant_case| {    
                    for (variant_index, variant) in self.iter_fields() {
                        // Enum element attributes
                        let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                        if let Some(fields) = &variant.fields {
                            match fields {
                                Fields::Struct(value) => {
                                    let args = value
                                                        .iter()
                                                        .map(|(ident, _v)| ident.to_string())
                                                        .collect::<Vec<String>>()
                                                        .join(", ");

                                    variant_case.push_parsed(format!("Self::{}{{{args}}}", variant.name))?;
                                },
                                Fields::Tuple(value) => {
                                    let args = value
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(index, _v)| format!("v{index}"))
                                                        .collect::<Vec<String>>()
                                                        .join(", ");

                                    variant_case.push_parsed(format!("Self::{}({args})", variant.name))?;
                                },
                            }
                        }
                        else {
                            variant_case.push_parsed(format!("Self::{}", variant.name))?;
                        }

                        variant_case.puncts("=>");

                        variant_case.group(Delimiter::Brace, |variant_body| {
                            variant_body.push_parsed(attributes.to_code(true, false))?;

                            if self.attributes.branch_starts_with {
                                // #[jbytes(branch_vlaue=b"xx")
                                if let Some(branch_value) = &attributes.branch_value {
                                    variant_body.push_parsed(format!("r_nbytes += buffer.push_bytes({branch_value})?;"))?;
                                }
                                else {
                                    let branch_value = format!("b\"{}\"", variant.name.to_string().to_lowercase());
                                    variant_body.push_parsed(format!("r_nbytes += buffer.push_bytes({branch_value})?;"))?;
                                }
                            }
                            else if self.attributes.branch_starts_with_untake {
                            }
                            else if self.attributes.branch_take_bytes.is_some() {
                                // #[jbytes(branch_vlaue=b"xx")
                                if let Some(branch_value) = &attributes.branch_value {
                                    variant_body.push_parsed(format!("r_nbytes += buffer.push_bytes({branch_value})?;"))?;
                                }                    
                                else {
                                    let branch_value = format!("b\"{}\"", variant.name.to_string().to_lowercase());
                                    variant_body.push_parsed(format!("r_nbytes += buffer.push_bytes({branch_value})?;"))?;
                                }
                            }            
                            else {
                                let default_byte_count_1byte_code = if self.attributes.byte_count_disable { "".to_string() } else { format!("r_nbytes += buffer.push_u8({variant_index} as u8)?;")};
                                let code = format!("
                                    let cr_byte_count = if let Some(cr) = cattr_new {{ cr.byte_count }} else {{ None }};
                        
                                    if let Some(fr) = fattr {{
                                        if let Some(_branch) = fr.branch {{
                                            // This is a placeholder condition
                                        }}
                                        else if let Some(byte_count) = cr_byte_count {{
                                            r_nbytes += buffer.push_byteorder_uint({variant_index} as u64, byte_count, jbytes::get_byteorder(cattr, fattr))?;
                                        }}
                                        else if let Some(byte_count) = fr.byte_count {{
                                            r_nbytes += buffer.push_byteorder_uint({variant_index} as u64, byte_count, jbytes::get_byteorder(cattr, fattr))?;
                                        }}
                                        else {{
                                            {default_byte_count_1byte_code}
                                        }}
                                    }}
                                    else if let Some(byte_count) = cr_byte_count {{
                                        r_nbytes += buffer.push_byteorder_uint({variant_index} as u64, byte_count, jbytes::get_byteorder(cattr, fattr))?;
                                    }}
                                    else {{
                                        {default_byte_count_1byte_code}
                                    }}
                                ");
                                variant_body.push_parsed(code)?;    
                            }

                            generate_encode_body2(variant_body, &attributes, false)?;

                            if let Some(fields) = &variant.fields {
                                match fields {
                                    Fields::Struct(value) => {
                                        for (ident, field) in value {
                                            let mut attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                                            attributes.get_variable_name = self.attributes.get_variable_name.clone();

                                            if attributes.is_use {
                                                variant_body.push_parsed(attributes.to_code(false, true))?;
                                            }

                                            generate_encode_body(variant_body, &attributes, crate_name, &ident.to_string(), false)?;
                                        }
                                    },
                                    Fields::Tuple(value) => {
                                        for (index, field) in value.iter().enumerate() {
                                            let mut attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                                            attributes.get_variable_name = self.attributes.get_variable_name.clone();

                                            if attributes.is_use {
                                                variant_body.push_parsed(attributes.to_code(false, false))?;
                                            }

                                            generate_encode_body(variant_body, &attributes, crate_name, &format!("v{index}"), false)?;
                                        }
                                    },
                                }
                            }
                            Ok(())
                        })?;

                        variant_case.puncts(",");
                    }

                    Ok(())
                })?;

                fn_body.push_parsed("Ok(r_nbytes)")?;

                Ok(())
            })?;
        Ok(())
    }
}


struct EnumVariantIterator<'a> {
    variants: &'a [EnumVariant],
    idx: usize,
    curruent_idx: usize,
}


impl<'a> Iterator for EnumVariantIterator<'a> {
    type Item = (TokenTree, &'a EnumVariant);

    fn next(&mut self) -> Option<Self::Item> {
        // let mut idx = self.idx;
        let variant = self.variants.get(self.idx)?;

        let val_string = if let Some(value) = &variant.value {
            // Literal
            Some(value.to_string())
        } else if let Ok(Some(attributes)) = variant.attributes.get_attribute::<FieldAttributes>() {
            if attributes.branch_range.is_some() {
                attributes.branch_range
            }
            else if attributes.branch_value.is_some() {
                attributes.branch_value
            }
            else {
                None
            }
        } else { None };

        if let Some(val_string) = val_string {
            if val_string.starts_with("0x") {
                self.curruent_idx = usize::from_str_radix(&val_string[2..], 16).unwrap();
            }
            else if let Some(offset) = val_string.find("..=") {
                self.curruent_idx = val_string[offset + 3..].parse::<usize>().unwrap();
            }
            else if let Some(offset) = val_string.find("..") {
                self.curruent_idx = val_string[offset + 2..].parse::<usize>().unwrap();
            }
            else if val_string.contains("b\"") {
            }
            else {
                self.curruent_idx = val_string.parse::<usize>().unwrap();
            }    
        }

        let tokens = TokenTree::Literal(Literal::usize_suffixed(self.curruent_idx));

        self.curruent_idx += 1;
        self.idx += 1;

        Some((tokens, variant))
    }
}
