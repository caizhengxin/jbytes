use std::sync::Mutex;
use std::collections::HashMap;
use virtue::prelude::*;
use virtue::utils::*;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref CACHE_GET_FIELD: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}


fn parse_value_string(value: &Literal) -> Result<String> {
    let val_string = value.to_string();

    if val_string.starts_with("\"") && val_string.ends_with("\"") {
        return Ok(val_string[1..val_string.len() - 1].to_string());
    }

    Ok(val_string)
}


fn parse_value_string_list(value: &Literal) -> Result<Vec<(String, String)>> {
    let mut vlist = Vec::new();

    for v in parse_value_string(&value)?.split(',') {
        if v.contains(':') {
            if let Some((v1, v2)) = v.split_once(':') {
                vlist.push((v1.trim().to_string(), v2.trim().to_string()));
            }
        }
        else {
            vlist.push((v.trim().to_string(), "".to_string()));
        }
    }

    Ok(vlist)
}


#[derive(Debug)]
pub struct ContainerAttributes {
    // pub crate_name: String,
    pub get_default: Vec<(String, String)>, 
}


impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            // crate_name: "jget".to_string(),
            get_default: vec![],
        }
    }
}


impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "jget")? {
            Some(body) => body,
            None => return Ok(None),
        };

        let mut result = Self::default();

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // #xxx[xxx]
                    match i.to_string().as_str() {
                        // "name" => {},
                        _ => return Err(Error::custom_at("Unknown field attribute", i.span())),
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "get_default" => result.get_default.extend(parse_value_string_list(&val)?),
                        _ => return Err(Error::custom_at("Unknown field attribute", key.span())),
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}


#[derive(Debug, Default)]
pub struct FieldAttributes {
    pub name: Option<String>, // function name
    pub get: bool,
    pub get_string: Vec<(String, String)>, // (field, type)
    pub get_option: bool,
    pub get_string_option: Vec<(String, String)>, // (field, type)

    pub clone: bool,
    pub option: bool,
    pub to_expr: Option<String>,
    pub to_func: Option<String>,
}


impl FromAttribute for FieldAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "jget")? {
            Some(body) => body,
            None => return Ok(None),
        };

        let mut result = Self::default();

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // #xxx[xxx]
                    match i.to_string().as_str() {
                        "get" => result.get = true,
                        "get_option" => result.get_option = true,
                        "clone" => result.clone = true,
                        "option" => result.option = true,
                        _ => return Err(Error::custom_at("Unknown field attribute", i.span())),
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "name" => result.name = Some(parse_value_string(&val)?),
                        "get" => result.get_string.extend(parse_value_string_list(&val)?),
                        "get_option" => result.get_string_option.extend(parse_value_string_list(&val)?),
                        // "to_expr" => result.to_expr = Some(parse_value_string(&val)?),
                        _ => return Err(Error::custom_at("Unknown field attribute", key.span())),
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}
