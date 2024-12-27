#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::parse::Fields;
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};
use super::decode::generate_decode_body;
use super::encode::generate_encode_body;
use super::parse::AttrValue;


pub(crate) struct DeriveStruct {
    pub fields: Option<Fields>,
    pub attributes: ContainerAttributes,
    pub lifetimes: Option<String>,
}


pub fn get_field_type(field: &UnnamedField) -> String {
    let field_type: Vec<String> = field.r#type.iter().map(|f|f.to_string()).collect();
    let field_type = field_type.iter().map(|v| { if v == "'" { v.to_string() } else { format!("{} ", v) } }).collect::<Vec<String>>();
    let field_type = field_type.join("");

    field_type
}


pub fn generate_decode_return(fn_body: &mut StreamBuilder, fields: &Option<Fields>, variant: Option<&EnumVariant>) -> Result<()> {
    if variant.is_some() {
        fn_body.push_parsed("return")?;
    }

    fn_body.ident_str("Ok");
    fn_body.group(Delimiter::Parenthesis, |ok_group| {
        let mut is_enum = false;

        if let Some(variant) = variant {
            ok_group.push_parsed(format!("Self::{}", variant.name.clone()))?;
            is_enum = true;
        }
        else {
            ok_group.push_parsed("Self")?;
        }

        if let Some(fields) = fields.as_ref() {
            match fields {
                Fields::Struct(value) => {
                    let args = value
                                        .iter()
                                        .map(|(ident, _v)| ident.to_string())
                                        .collect::<Vec<String>>()
                                        .join(", ");

                    ok_group.push_parsed(format!("{{{args}}}"))?;
                },
                Fields::Tuple(value) => {
                    let args = value
                                        .iter()
                                        .enumerate()
                                        .map(|(index, _v)| format!("v{index}"))
                                        .collect::<Vec<String>>()
                                        .join(", ");

                    ok_group.push_parsed(format!("({args})"))?;
                },
            }
        }
        else if !is_enum {
            // Ok((input, Self {}))
            ok_group.push_parsed("{{}}")?;
        }

        Ok(())
    })?;

    Ok(())
}


pub fn generate_decode_struct_body(fn_body: &mut StreamBuilder, crate_name: &str, fields: &Option<Fields>, _cattr: &ContainerAttributes, _is_enum: bool) -> Result<()> {
    if let Some(fields) = fields.as_ref() {
        match fields {
            Fields::Struct(value) => {
                for (ident, field) in value {
                    let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                    fn_body.push_parsed(attributes.to_code(false, false))?;
                    generate_decode_body(fn_body, crate_name, &attributes, ident.to_string(), &get_field_type(field), false)?;
                }
            },
            Fields::Tuple(value) => {
                for (index, field) in value.iter().enumerate() {
                    let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                    if attributes.is_use {
                        fn_body.push_parsed(attributes.to_code(false, false))?;
                    }
                    generate_decode_body(fn_body, crate_name, &attributes, index.to_string(), &get_field_type(field), true)?;
                }
            },
        }   
    }

    Ok(())
}


impl DeriveStruct {
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
                // fn_body.push_parsed(self.attributes.to_code(false))?;
                self.generate_byte_decode_body(crate_name, fn_body)?;

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
                // fn_body.push_parsed(self.attributes.to_code(false))?;
                self.generate_byte_decode_body(crate_name, fn_body)?;

                Ok(())
            })?;

        Ok(())
    }

    fn generate_byte_decode_body(&self, crate_name: &str, fn_body: &mut StreamBuilder) -> Result<()> {
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
                        "))?;                    }    
                }
            }

            generate_decode_struct_body(fn_body, crate_name, &self.fields, &self.attributes, false)?;
            generate_decode_return(fn_body, &self.fields, None)?;
        }

        Ok(())
    }

    pub fn generate_encode(&self, generator: &mut Generator) -> Result<()> {
        self.generate_byte_encode("jbytes::ByteEncode", generator)?;
        Ok(())
    }

    pub fn generate_borrow_encode(&self, generator: &mut Generator) -> Result<()> {
        self.generate_byte_encode("jbytes::BorrowByteEncode", generator)?;
        Ok(())
    }

    fn generate_byte_encode(&self, crate_name: &str, generator: &mut Generator) -> Result<()> {
        generator
            .impl_for(crate_name)
            .generate_fn("encode_inner")
            .with_generic_deps("B", ["jbytes::BufWriteMut"])
            .with_self_arg(FnSelfArg::RefSelf)
            .with_arg("buffer", "&mut B")
            .with_arg("cattr", "Option<&jbytes::ContainerAttrModifiers>")
            .with_arg("fattr", "Option<&jbytes::FieldAttrModifiers>")
            .with_return_type("jbytes::JResult<usize>")
            .body(|fn_body| {
                fn_body.push_parsed("let mut r_nbytes = 0;")?;

                if let Some(func) = &self.attributes.with_encode {
                    fn_body.push_parsed(format!("r_nbytes += {func}(buffer, cattr, fattr, self)?;"))?;
                }
                else if let Some(func) = &self.attributes.with {
                    fn_body.push_parsed(format!("r_nbytes += {func}::encode(buffer, cattr, fattr, self)?;"))?;
                }
                else {
                    fn_body.push_parsed(self.attributes.to_code(true))?;

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

                    if let Some(fields) = self.fields.as_ref() {
                        for field in fields.names() {
                            let mut attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();
                            attributes.get_variable_name = self.attributes.get_variable_name.clone();
                            fn_body.push_parsed(attributes.to_code(true, false))?;
                            generate_encode_body(fn_body, &attributes, crate_name, &field.to_string(), true)?;
                        }
                    }
                }

                fn_body.push_parsed("Ok(r_nbytes)")?;
        
                Ok(())
            })?;
        Ok(())
    }
}
