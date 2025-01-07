use virtue::prelude::*;
use virtue::utils::*;


#[inline]
fn parse_value_string(value: &Literal) -> Result<String> {
    let val_string = value.to_string();
    let val_string = val_string.replace("\\\"", "\"");

    if val_string.starts_with("\"") && val_string.ends_with("\"") {
        return Ok(val_string[1..val_string.len() - 1].to_string());
    }

    Ok(val_string)
}


#[derive(Debug)]
pub struct ContainerAttributes {
    pub is_jbytes: bool,
    // pub crate_name: String,
    pub default_value: Option<String>
}


impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            // crate_name: "jdefault".to_string(),
            is_jbytes: false,
            default_value: None,
        }
    }
}


impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let mut result = Self::default();

        let attributes = if let Some(body) = parse_tagged_attribute(group, "jbytes")? {
            result.is_jbytes = true;
            body
        }
        else if let Some(body) = parse_tagged_attribute(group, "jd")? { body }
        else { return Ok(None); };

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // #xxx[xxx]
                    match i.to_string().as_str() {
                        // "name" => {},
                        _ => {
                            if !result.is_jbytes {
                                return Err(Error::custom_at("Unknown field attribute", i.span()));
                            }
                        }
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "default_value" | "default" => result.default_value = Some(parse_value_string(&val)?),
                        _ => {
                            if !result.is_jbytes {
                                return Err(Error::custom_at("Unknown field attribute", val.span()));
                            }
                        }
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
    pub is_jbytes: bool,
    pub default_bool: bool,
    pub default_value: Option<String>
}


impl FromAttribute for FieldAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let mut result = Self::default();

        let attributes = if let Some(body) = parse_tagged_attribute(group, "jbytes")? {
            result.is_jbytes = true;
            body
        }
        else if let Some(body) = parse_tagged_attribute(group, "jppe")? {
            result.is_jbytes = true;
            body
        }
        else if let Some(body) = parse_tagged_attribute(group, "jd")? { body }
        else { return Ok(None); };

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // #xxx[xxx]
                    match i.to_string().as_str() {
                        "default_value" | "default" | "enum_default" | "branch_default" => result.default_bool = true,
                        _ => { 
                            if !result.is_jbytes {
                                return Err(Error::custom_at("Unknown field attribute", i.span()));
                            }
                        }
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "default_value" | "default" => result.default_value = Some(parse_value_string(&val)?),
                        _ => {
                            if !result.is_jbytes {
                                return Err(Error::custom_at("Unknown field attribute", val.span()));
                            }
                        }
                    }
                }
                _ => { }
            }
        }

        Ok(Some(result))
    }
}
