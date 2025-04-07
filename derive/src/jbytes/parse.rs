use virtue::prelude::*;


#[derive(Debug, Clone)]
pub enum AttrValue {
    String(String),
    Bytes(String),
    Var(String),
    Usize(usize),
    Option(String),
    List(Vec<AttrValue>),
}


#[inline]
pub fn parse_value_string(s: &Literal) -> Result<String> {
    let val_string = s.to_string().replace("\\\"", "\"").to_string();

    if val_string.starts_with("\"") && val_string.ends_with("\"") {
        return Ok(val_string[1..val_string.len() - 1].to_string());
    }

    Ok(val_string)
}


impl AttrValue {
    #[allow(dead_code)]
    #[inline]
    pub fn parse_string(s: &Literal) -> Result<Self> {
        Ok(Self::String(parse_value_string(s)?))
    }

    #[inline]
    pub fn parse_bytes(s: &Literal) -> Result<Self> {
        Ok(Self::Bytes(parse_value_string(s)?))
    }

    #[allow(dead_code)]
    #[inline]
    pub fn parse_option_string(s: &Literal) -> Result<Self> {
        Ok(Self::Option(parse_value_string(s)?))
    }

    #[inline]
    pub fn parse_usize(s: &Literal) -> Result<Self> {
        let value = parse_value_string(s)?;
        let value_type = if value.starts_with("0x") {16} else {10};

        if let Ok(v) = usize::from_str_radix(value.trim_start_matches("0x"), value_type) {
            return Ok(Self::Usize(v));
        }

        Ok(Self::Var(value))
    }

    #[inline]
    pub fn parse_list(s: &Literal) -> Result<Self> {
        let value = parse_value_string(s)?;
        let mut vlist = vec![];

        for v in value.split(',') {
            let value_type = if v.starts_with("0x") {16} else {10};

            if let Ok(v) = usize::from_str_radix(v.trim_start_matches("0x"), value_type) {
                vlist.push(Self::Usize(v));
            }

            vlist.push(Self::String(v.to_string()))
        }

        Ok(Self::List(vlist))
    }

    #[inline]
    pub fn parse_byteorder(s: &Literal) -> Result<Self> {
        let value = parse_value_string(s)?;

        match value.as_str() {
            "BE" | "LE" | "0" | "1" | ">" | "<" => Ok(Self::String(value)),
            _ => Ok(Self::Var(value)),
        }
    }

    pub fn to_code(&self, is_self: bool, is_deref: bool, is_string: bool) -> String {
        let self_arg = if is_self { "self." } else { "" };
        let deref_arg = if is_deref { "*" } else { "" };
        let is_string = if is_string { "\"" } else { "" };

        let code = match self {
            Self::String(v) => format!("{deref_arg}{self_arg}{is_string}{v}{is_string}.into()"),
            Self::Bytes(v) => format!("{deref_arg}{self_arg}{is_string}{v}{is_string}"),
            Self::Var(v) => format!("({deref_arg}{self_arg}{is_string}{v}{is_string}) as usize"),
            Self::Usize(v) => format!("{v} as usize"),
            Self::Option(v) => format!("if let Some(v) = {deref_arg}{self_arg}{is_string}{v} {{Some(v as usize)}} else {{None}}"),
            Self::List(v) =>  {
                let value = v.iter().map(|v| format!("{}", v.to_code(is_self, is_deref, true))).collect::<Vec<String>>().join(", ");

                format!("vec![{value}]")
            },
        };

        code
    }

    pub fn to_code2(&self, is_self: bool, is_string: bool) -> String {
        let self_arg = if is_self { "self." } else { "*" };
        let is_string = if is_string { "\"" } else { "" };

        let code = match self {
            Self::String(v) => format!("{self_arg}{is_string}{v}{is_string}.into()"),
            Self::Bytes(v) => format!("{self_arg}{is_string}{v}{is_string}"),
            Self::Var(v) => format!("({self_arg}{is_string}{v}{is_string}) as usize"),
            Self::Usize(v) => format!("({v}) as usize"),
            Self::Option(v) => format!("if let Some(v) = {self_arg}{is_string}{v} {{Some(v as usize)}} else {{None}}"),
            Self::List(v) =>  {
                let value = v.iter().map(|v| format!("{}", v.to_code2(is_self, true))).collect::<Vec<String>>().join(", ");

                format!("vec![{value}]")
            },
        };

        code
    }

    pub fn to_byteorder(&self, is_self: bool) -> String {
        let self_arg = if is_self { "self." } else { "" };

        let code = match self {
            Self::String(v) => format!("jbytes::ByteOrder::parse({v:?}).unwrap()"),
            Self::Var(v) => format!("jbytes::ByteOrder::parse_int({self_arg}{v} as isize).unwrap()"),
            _ => "".to_string(),
        };

        code
    }
}


impl ToString for AttrValue {
    fn to_string(&self) -> String {
        match self {
            Self::String(v) => v.to_string(),
            Self::Bytes(v) => v.to_string(),
            Self::Var(v) => v.to_string(),
            Self::Usize(v) => v.to_string(),
            Self::Option(v) => v.to_string(),
            Self::List(v) => v.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", "),
        }
    }
}


pub trait AttrValueTrait {
    type Value;

    fn to_code(&self, is_self: bool, is_deref: bool) -> String;

    #[allow(dead_code)]
    fn to_code_string(&self, is_self: bool, is_deref: bool, is_string: bool) -> String;

    #[allow(dead_code)]
    fn to_code_option_string(&self, is_self: bool, is_deref: bool, is_string: bool) -> String;

    fn to_byteorder(&self, is_self: bool) -> String;
}


impl AttrValueTrait for Option<AttrValue> {
    type Value = AttrValue;

    #[inline]
    fn to_code(&self, is_self: bool, is_deref: bool) -> String {
        if let Some(value) = self {
            return format!("Some({})", value.to_code(is_self, is_deref, false));
        }

        "None".to_string()
    }

    #[inline]
    fn to_code_string(&self, is_self: bool, is_deref: bool, is_string: bool) -> String {
        if let Some(value) = self {
            return format!("Some({})", value.to_code(is_self, is_deref, is_string));
        }

        "None".to_string()
    }

    #[inline]
    fn to_code_option_string(&self, is_self: bool, is_deref: bool, is_string: bool) -> String {
        if let Some(value) = self {
            return value.to_code(is_self, is_deref, is_string);
        }

        "None".to_string()
    }

    #[inline]
    fn to_byteorder(&self, is_self: bool) -> String {
        if let Some(value) = self {
            return format!("Some({})", value.to_byteorder(is_self));
        }

        "None".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attrvalue_parse() {
        let value = Some(AttrValue::List(vec![
            AttrValue::String("jkc".to_string()),
            AttrValue::String("jkc".to_string()),
        ]));

        println!("{:?}", value.to_code(false, false));
        assert_eq!(value.to_code(false, false), r#"Some(vec!["jkc".into(), "jkc".into()])"#);

        let value = Some(AttrValue::String("jkc".to_string()));
        assert_eq!(value.to_code(false, false), r#"Some(jkc.into())"#);
    }
}