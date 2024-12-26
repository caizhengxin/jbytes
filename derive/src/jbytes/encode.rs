use virtue::prelude::*;
#[allow(unused_imports)]
use super::attribute::{FieldAttributes, ContainerAttributes};


#[inline]
pub fn generate_encode_body2(fn_body: &mut StreamBuilder, attributes: &FieldAttributes, is_self: bool) -> Result<()> {
    // offset and full
    if let Some(offset) = &attributes.offset {
        let full_value = if let Some(full_tmp) = &attributes.full { full_tmp.to_string() } else { "0x00".to_string() };

        fn_body.push_parsed(format!("
            for i in 0..{} {{ buffer.push_u8({full_value})?; }}
        ", offset.to_code2(is_self, false)))?;
    }

    Ok(())
}


#[inline]
pub fn generate_encode_body(fn_body: &mut StreamBuilder, attributes: &FieldAttributes, crate_name: &str, field: &String, is_self: bool) -> Result<()> {
    let der_arg = if is_self {"&"} else {""};
    let self_arg = if is_self {"self."} else {""};

    let with_args = if let Some(value) = &attributes.with_args {format!("{self_arg}{value}")} else { "".to_string() };

    if let Some(func) = &attributes.with_encode {
        fn_body.push_parsed(format!("{func}(buffer, cattr_new, fattr_new, {der_arg}{self_arg}{field}, {with_args});"))?;
        return Ok(());
    }
    else if let Some(func) = &attributes.with {
        fn_body.push_parsed(format!("{func}::encode(buffer, cattr_new, fattr_new, {der_arg}{self_arg}{field}, {with_args});"))?;
        return Ok(());
    }
    else if attributes.skip || attributes.skip_encode {
        return Ok(());
    }
    else if attributes.from_str_bool {
        fn_body.push_parsed(format!("String::encode_inner(&{der_arg}{self_arg}{field}.to_string(), buffer, cattr_new, fattr_new);"))?;
    }
    else if let Some(_) = &attributes.from_str {
        fn_body.push_parsed(format!("String::encode_inner(&{der_arg}{self_arg}{field}.to_string(), buffer, cattr_new, fattr_new);"))?;
    }

    if attributes.bits.is_some() || !attributes.untake {
        generate_encode_body2(fn_body, attributes, is_self)?;

        if let Some(value_expr) = &attributes.value_encode {
            fn_body.push_parsed(format!("let {field} = {der_arg}{self_arg}{field};"))?;
            fn_body.push_parsed(format!("let {field} = {value_expr};"))?;
            fn_body.push_parsed(format!("r_nbytes += {crate_name}::encode_inner(&{field}, buffer, cattr_new, fattr_new)?;"))?;        
        }
        else {
            fn_body.push_parsed(format!("r_nbytes += {crate_name}::encode_inner({der_arg}{self_arg}{field}, buffer, cattr_new, fattr_new)?;"))?;
        }    
    }

    Ok(())
}
