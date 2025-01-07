use virtue::generate::Generator;
use virtue::parse::Fields;
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};


pub(crate) struct DeriveStruct {
    pub fields: Option<Fields>,
    pub attributes: ContainerAttributes,
}


#[inline]
pub fn generate_struct_body(fn_body: &mut StreamBuilder, fields: &Option<Fields>, value: String) -> Result<()> {
    if let Some(fields) = fields {
        match fields {
            Fields::Struct(fields) => {
                // pub struct XXX {
                //     field1: u16,
                //     field2: u16,
                // }

                fn_body.push_parsed(format!("Self{value}"))?;

                fn_body.group(Delimiter::Brace, |fn_body_field| {
                    for (ident, field) in fields {
                        let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                        let field_name = ident.to_string();
                        let field_type = field.type_string();
    
                        if let Some(default_value) = attributes.default_value {
                            fn_body_field.push_parsed(format!("{field_name}: {default_value},"))?;
                        }
                        else {
                            fn_body_field.push_parsed(format!("{field_name}: {field_type}::default(),"))?;
                        }
                    }
    
                    Ok(())
                })?;
            },
            Fields::Tuple(fields) => {
                // pub struct XXX(field1, field2);
                // pub struct XXX((field1, field2));

                fn_body.push_parsed(format!("Self{value}"))?;
                fn_body.group(Delimiter::Parenthesis, |fn_body_field| {
                    for field in fields {
                        let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();
                        let field_type = field.type_string();

                        if let Some(default_value) = attributes.default_value {
                            fn_body_field.push_parsed(format!("{default_value},"))?;
                        }
                        else if field_type.contains('(') {
                            fn_body_field.push_parsed(format!("<{field_type}>::default(),"))?;
                        }
                        else {
                            fn_body_field.push_parsed(format!("{field_type}::default(),"))?;
                        }
                    }

                    Ok(())
                })?;   
            },
        }
    }
    else {
        fn_body.push_parsed(format!("Self{value}"))?;
    }

    Ok(())
}


impl DeriveStruct {
    pub fn generate_jdefault(&self, generator: &mut Generator) -> Result<()> {
        generator
            .impl_for("Default")
            .generate_fn("default")
            // .with_self_arg(FnSelfArg::RefSelf)
            // .with_arg("args_name", "args_type")
            .with_return_type("Self")
            .body(|fn_body| {
                if let Some(default_value) = &self.attributes.default_value {
                    fn_body.push_parsed(format!("Self({default_value})"))?;
                }
                else {
                    generate_struct_body(fn_body, &self.fields, "".to_string())?;
                }
                Ok(())
            })?;

        Ok(())
    }
}
