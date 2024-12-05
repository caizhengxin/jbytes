use virtue::generate::Generator;
use virtue::parse::Fields;
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};


pub(crate) struct DeriveStruct {
    pub fields: Option<Fields>,
    pub attributes: ContainerAttributes,
}


impl DeriveStruct {
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

                if let Some(fields) = self.fields.as_ref() {
                    // pub struct XXX {
                    //     field1: u16,
                    //     field2: u16,
                    // }
                    for field in fields.names() {
                        let attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();

                        // TODO
                    }
                }

                fn_body.push_parsed("\"hello, world!\".to_string()")?;

                Ok(())
            })?;

        Ok(())
    }
}
