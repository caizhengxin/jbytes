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
fn get_field_name(field_name: &String) -> String {
    field_name.replace("r#", "")
}


pub fn get_function_name(field_name: &String) -> String {
    let field_name = field_name.replace("r#", "");
    let field_name = field_name.replace("get_", "");
    let field_name = field_name.replace("()", "");
    let field_name = field_name.split('.').collect::<Vec<&str>>();

    field_name.last().unwrap_or_else(|| &"").to_string()
}


pub fn get_return_type_string(_attributes: &FieldAttributes, return_type: Option<&String>, field_type: &String, is_option: bool) -> String {
    let mut field_type_tmp = field_type.to_string();


    if let Some(return_type) = return_type {
        field_type_tmp = return_type.to_string();
    }

    if !field_type_tmp.starts_with("Option<") && (is_option || field_type.starts_with("Option<")) {
        field_type_tmp = format!("Option<{field_type_tmp}>");
    }
    
    field_type_tmp
}


pub fn get_code_string(attributes: &FieldAttributes, return_type: &mut String, field_name: &String, field_type: &String, is_option: bool, is_self: bool, is_deref: bool) -> String {
    let self_args = if is_self {"self."} else {""};
    let deref_args = if is_deref {"&"} else {""};
    let field_type = &field_type.replace(" ", "");
    let field_name = get_field_name(&field_name);
    let mut code_list = vec![];

    if attributes.clone {
        code_list.push(format!("let value = {self_args}{field_name}.clone();"));
    }
    else if field_type.contains('&') {
        code_list.push(format!("let value = {self_args}{field_name};"));
    }
    else {
        code_list.push(format!("let value = {deref_args}{self_args}{field_name};"));
    }

    if let Some(expr) = &attributes.to_expr {
        code_list.push(format!("let value = {expr};"));
    }
    else if let Some(func) = &attributes.to_func {
        code_list.push(format!("let value = {func}(value);"));
    }
    else if (return_type.starts_with("Vec<u") || return_type.starts_with("Vec<i")) && return_type != field_type {
        let as_type = return_type.replace("Vec<", "").replace(">", "");

        if field_type.starts_with("Option<Vec<") {
            code_list.push(format!("let value = if let Some(value) = value {{ value.iter().map(| v | *v as {as_type}).collect::<{return_type}>().into() }} else {{ None }};"));  
        }
        else if field_type.starts_with("Vec<") {
            code_list.push(format!("let value = value.iter().map(| v | *v as {as_type}).collect::<{return_type}>();"));
        }
        else {
            code_list.push(format!("let value = vec![*value as {as_type}];"));
        }
    }
    else if (return_type.starts_with("Option<Vec<u") || return_type.starts_with("Option<Vec<i"))
            && ((field_type.starts_with("Option<") && return_type != field_type) || (!field_type.starts_with("Option<") && return_type != &format!("Option<{field_type}>"))) {
        let as_type = return_type.replace("Option<Vec<", "").replace(">>", "");
        let return_type = return_type.replace("Option<", "").replacen(">", "", 1);

        if field_type.starts_with("Option<Vec<") {
            code_list.push(format!("let value = if let Some(value) = value {{ value.iter().map(| v | *v as {as_type}).collect::<{return_type}>().into() }} else {{ None }};"));  
        }
        else if field_type.starts_with("Vec<") {
            code_list.push(format!("let value = value.iter().map(|v| *v as {as_type}).collect::<{return_type}>().into();"));
        }
        else {
            code_list.push(format!("let value = vec![*value as {as_type}];"));
        }
    }
    else if field_type.starts_with("Option<") && !(return_type.starts_with('&') || return_type.starts_with("Option<&")) {
        code_list.push(format!("let value = if let Some(value) = value {{ Some(value.clone().into()) }} else {{ None }};"));  
    }
    else if field_type.starts_with("Option<&") || field_type.starts_with("Option<") {
        code_list.push(format!("let value = if let Some(value) = value {{ Some(value.into()) }} else {{ None }};"));  
    }
    else if field_type.contains('&') || return_type.starts_with('&') || return_type.starts_with("Option<&") {
        code_list.push(format!("let value = value.into();"));
    }
    else {
        code_list.push(format!("let value = value.clone().into();"));
    }

    if field_type.starts_with("Option<") {
        code_list.push(format!("value"));
    }
    else if attributes.option || is_option {
        code_list.push(format!("Some(value)"));
    }
    else {
        code_list.push(format!("value"));
    }

    code_list.join("\n")
}


pub fn generate_struct_body(fields: &Option<Fields>, is_self: bool, is_deref: bool) -> Result<Vec<(String, String, String, String)>> {
    let mut cache_list = vec![];

    if let Some(fields) = fields {
        match fields {
            Fields::Struct(fields) => {
                for (ident, field) in fields {
                    let field_type: Vec<String> = field.r#type.iter().map(|f|f.to_string()).collect();
                    let field_type = field_type.iter().map(|v| { if v == "'" { v.to_string() } else { format!("{} ", v) } }).collect::<Vec<String>>();
                    let field_type = field_type.join("");

                    let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                    if attributes.get {
                        let function_name = get_function_name(&ident.to_string());
                        let mut return_type = get_return_type_string(&attributes, None, &field_type, false);
                        let function_body = get_code_string(&attributes, &mut return_type, &ident.to_string(), &field_type, false, is_self, is_deref);

                        cache_list.push((function_name, return_type, function_body, ident.to_string()));
                    }
                    
                    if attributes.get_option {
                        let function_name = get_function_name(&ident.to_string());
                        let mut return_type = get_return_type_string(&attributes, None, &field_type, true);
                        let function_body = get_code_string(&attributes, &mut return_type, &ident.to_string(), &field_type, true, is_self, is_deref);

                        cache_list.push((function_name, return_type, function_body, ident.to_string()));
                    }

                    if !attributes.get_string.is_empty() {
                        for (field_name, return_type) in &attributes.get_string {
                            let function_name = get_function_name(field_name);
                            let mut return_type = get_return_type_string(&attributes, Some(return_type), &field_type, false);
                            let field_name = if field_name.contains('.') {field_name.to_string()} else { ident.to_string() };
                            let function_body = get_code_string(&attributes, &mut return_type, &field_name, &field_type, false, is_self, is_deref);

                            cache_list.push((function_name, return_type, function_body, ident.to_string()));
                        }
                    }
                    
                    if !attributes.get_string_option.is_empty() {
                        for (field_name, return_type) in &attributes.get_string_option {
                            let function_name = get_function_name(field_name);
                            let mut return_type = get_return_type_string(&attributes, Some(return_type), &field_type, true);
                            let field_name = if field_name.contains('.') {field_name.to_string()} else { ident.to_string() };
                            let function_body = get_code_string(&attributes, &mut return_type, &field_name, &field_type, true, is_self, is_deref);

                            cache_list.push((function_name, return_type, function_body, ident.to_string()));
                        }
                    }
                }
            },
            Fields::Tuple(_fields) => {
                // enum Example {
                //     V1(xxx),
                //     V2(xxx),
                // }

                // pub fn get_value(&self) -> Option<xxx> {
                //     match self {
                //         Self::V1(v) => Some(v.value),
                //         Self::V2(v) => Some(v.value),
                //     }
                // }

                // for field in fields {
                //     let field_type: Vec<String> = field.r#type.iter().map(|f|f.to_string()).collect();
                //     let field_type = field_type.iter().map(|v| { if v == "'" { v.to_string() } else { format!("{} ", v) } }).collect::<Vec<String>>();
                //     let field_type = field_type.join("");

                //     let attributes = field.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                //     if !attributes.get_string.is_empty() {
                //         for (field_name, return_type) in &attributes.get_string {
                //             let function_name = get_function_name(field_name);
                //             let mut return_type = get_return_type_string(&attributes, Some(return_type), &field_type, false);
                //             let function_body = get_code_string(&attributes, &mut return_type, &field_name, &field_type, false, is_self, is_deref);

                //             cache_list.push((function_name, return_type, function_body, field_name.to_string()));
                //         }
                //     }
                //     else if !attributes.get_string_option.is_empty() {
                //         for (field_name, return_type) in &attributes.get_string_option {
                //             let function_name = get_function_name(field_name);
                //             let mut return_type = get_return_type_string(&attributes, Some(return_type), &field_type, true);
                //             let function_body = get_code_string(&attributes, &mut return_type, &field_name, &field_type, true, is_self, is_deref);

                //             cache_list.push((function_name, return_type, function_body, field_name.to_string()));
                //         }
                //     }
                // }
            },
        }
    }

    Ok(cache_list)
}


#[inline]
pub fn generate_create_get_default(generator: &mut Generator, attributes: &ContainerAttributes) -> Result<()> {
    let c_attributes = attributes;

    if c_attributes.get_default.is_empty() {
        return Ok(());
    }

    let mut generator_impl = generator.r#impl();

    for (field_name, return_type) in &c_attributes.get_default {
        let field_name = get_field_name(field_name);

        let return_type = if !return_type.starts_with("Option<") {
            format!("Option<{return_type}>")
        }
        else {
            return_type.to_string()
        };

        generator_impl
            .generate_fn(format!("get_{field_name}"))
            .make_pub()
            .with_self_arg(FnSelfArg::RefSelf)
            .with_return_type(return_type)
            .body(|fn_body| {
                fn_body.push_parsed("None")?;
                Ok(())
            })?;
    }

    Ok(())
}


impl DeriveStruct {
    pub fn generate_create_get_function(&self, generator: &mut Generator) -> Result<()> {
        let cache_list = generate_struct_body(&self.fields, true, true)?;

        if cache_list.is_empty() {
            return Ok(());
        }

        let mut generator_impl = generator.r#impl();

        for (function_name, return_type, function_body, _) in &cache_list {
            // println!("{function_name:?} {return_type:>} {function_body:?}");
            generator_impl
                .generate_fn(format!("get_{function_name}"))
                .make_pub()
                .with_self_arg(FnSelfArg::RefSelf)
                .with_return_type(return_type)
                .body(|fn_body| {
                    fn_body.push_parsed(function_body)?;
                    Ok(())
                })?;
        }

        Ok(())
    }

    pub fn generate_jget(&self, generator: &mut Generator) -> Result<()> {
        generate_create_get_default(generator, &self.attributes)?;
        self.generate_create_get_function(generator)?;

        Ok(())
    }
}
