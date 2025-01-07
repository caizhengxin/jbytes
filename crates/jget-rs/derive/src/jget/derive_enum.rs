use std::collections::HashMap;
#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{ContainerAttributes, FieldAttributes};
use super::derive_struct::*;


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

    pub fn generate_jget(&self, generator: &mut Generator) -> Result<()> {
        generate_create_get_default(generator, &self.attributes)?;
        self.generate_crate_get_function(generator)?;
        Ok(())
    }

    pub fn generate_crate_get_function(&self, generator: &mut Generator) -> Result<()> {
        // let crate_name = format!("{}::Jget", self.attributes.crate_name);

        // pub fn get_xxx(&self) -> Option<type> {
        //     match self {
        //         Self::XXX(value) => Some(value),
        //         Self::VVV(value) => Some(value), 
        //         _ => None,
        //     }
        // }

        // HashMap<function_name, return_type>
        let mut cache_return_type: HashMap<String, (String, bool)> = HashMap::new();
        // HashMap<function_name, Vec[(enum_field_name, enum_value, function_body), (enum_field_name, enum_value, function_body)]>
        let mut cache_enum_element: HashMap<String, Vec<(String, String, String)>> = HashMap::new();

        for (_variant_index, variant) in self.iter_fields() {
            let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

            if !attributes.get_string.is_empty() {
                for (field_name, return_type) in &attributes.get_string {
                    let function_name = get_function_name(field_name);
                    let return_type = get_return_type_string(&attributes, Some(return_type), &return_type, false);
                    let function_body = format!("v.{field_name}");

                    cache_return_type.insert(function_name.clone(), (return_type, false));

                    if let Some(value) = cache_enum_element.get_mut(&function_name) {
                        value.push((variant.name.to_string(), "".to_string(), function_body));
                    }
                    else {
                        cache_enum_element.insert(function_name, vec![(variant.name.to_string(), "".to_string(), function_body)]);
                    }    
                }
            }
    
            if !attributes.get_string_option.is_empty() {
                for (field_name, return_type) in &attributes.get_string_option {
                    let function_name = get_function_name(field_name);
                    let return_type = get_return_type_string(&attributes, Some(return_type), &return_type, true);
                    let function_body = if !return_type.contains("Option<") && field_name.contains('(') { format!("v.{field_name}") } else { format!("Some(v.{field_name})") };

                    cache_return_type.insert(function_name.clone(), (return_type, true));

                    if let Some(value) = cache_enum_element.get_mut(&function_name) {
                        value.push((variant.name.to_string(), "".to_string(), function_body));
                    }
                    else {
                        cache_enum_element.insert(function_name, vec![(variant.name.to_string(), "".to_string(), function_body)]);
                    }
                }
            }

            for (function_name, return_type, function_body, ident) in generate_struct_body(&variant.fields, false, false)? {
                cache_return_type.insert(function_name.clone(), (return_type, true));

                if let Some(value) = cache_enum_element.get_mut(&function_name) {
                    value.push((variant.name.to_string(), ident, function_body));
                }
                else {
                    cache_enum_element.insert(function_name, vec![(variant.name.to_string(), ident, function_body)]);
                }
            }
        }

        if cache_return_type.is_empty() {
            return Ok(());
        }

        let mut generator_impl = generator.r#impl();

        for (function_name, (return_type, is_match)) in &cache_return_type {
            generator_impl
                .generate_fn(format!("get_{function_name}"))
                .make_pub()
                .with_self_arg(FnSelfArg::RefSelf)
                .with_return_type(return_type)
                .body(|fn_body| {
                    fn_body.push_parsed("match self")?;

                    fn_body.group(Delimiter::Brace, |variant_case| {
                        if let Some(element) = cache_enum_element.get(function_name) {
                            for (variant_name, ident, function_body) in element {
                                if ident.is_empty() {
                                    variant_case.push_parsed(format!("Self::{variant_name} (v) => {{ {function_body} }},", ))?;
                                }
                                else {
                                    variant_case.push_parsed(format!("Self::{variant_name} {{{ident}, ..}} => {{ {function_body} }},", ))?;
                                }
                            }    
                        }

                        if *is_match {
                            variant_case.push_parsed(format!("_ => None,", ))?;
                        }
    
                        Ok(())
                    })?;    

                    Ok(())
                })?;
        }
    
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
