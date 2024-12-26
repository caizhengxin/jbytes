use core::{
    fmt,
    ops,
    str, str::FromStr,
};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error as DeError};
use crate::errors::ThisError;


#[derive(Debug, ThisError)]
pub enum HexStringParseError {
    #[error("invalid hex string: `{0}`")]
    InvalidHexString(String),
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HexString {
    inner: Vec<u8>,
}


const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";
const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";


#[inline]
fn is_hex(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - 0x30),
        b'a'..=b'f' => Some(value - 0x57),
        b'A'..=b'F' => Some(value - 0x37),
        _ => None,
    }
}


#[inline]
fn parse_hex<T: AsRef<[u8]>>(t: T) -> Result<Vec<u8>, HexStringParseError> {
    let s = t.as_ref();

    if s.len() % 2 != 0 {
        return Err(HexStringParseError::InvalidHexString(str::from_utf8(s).unwrap_or_default().to_string()));
    }

    let mut vlist = vec![];

    for v in s.chunks(2) {
        if let Some(v0) = is_hex(v[0]) {
            if let Some(v1) = is_hex(v[1]) {
                vlist.push(v0 << 4 | v1);
            }
            else {
                return Err(HexStringParseError::InvalidHexString(str::from_utf8(s).unwrap_or_default().to_string()));
            }
        }
        else {
            return Err(HexStringParseError::InvalidHexString(str::from_utf8(s).unwrap_or_default().to_string()));
        }
    }

    Ok(vlist)
}


impl HexString {
    pub fn new<T: AsRef<[u8]>>(t: T) -> Self {
        Self{ inner: t.as_ref().to_vec() }
    }

    pub fn from_bytes<T: AsRef<[u8]>>(t: T) -> Result<Self, HexStringParseError> {
        Ok(Self { inner: parse_hex(t)? })
    }

    pub fn push(&mut self, c: u8) {
        self.inner.push(c);
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), HexStringParseError> {
        let value = parse_hex(s)?;

        self.inner.extend(value);

        Ok(())
    }

    pub fn to_hex_lowercase(&self) -> String {
        let mut vstring = String::new();

        for v in &self.inner {
            vstring.push(HEX_LOWER[(*v >> 4) as usize].into());
            vstring.push(HEX_LOWER[(*v & 0xf) as usize].into());
        }

        vstring
    }

    pub fn to_hex_uppercase(&self) -> String {
        let mut vstring = String::new();

        for v in &self.inner {
            vstring.push(HEX_UPPER[(*v >> 4) as usize].into());
            vstring.push(HEX_UPPER[(*v & 0xf) as usize].into());
        }

        vstring
    }
}


impl FromStr for HexString {
    type Err = HexStringParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { inner: parse_hex(s)? })
    }
}


impl fmt::Display for HexString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_lowercase())
    }
}


impl ops::Deref for HexString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


impl ops::DerefMut for HexString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


#[cfg(feature = "serde")]
impl Serialize for HexString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}


#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for HexString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value = Deserialize::deserialize(deserializer)?;

        HexString::from_str(value).map_err(D::Error::custom)
    }
}


pub fn encode<T: AsRef<[u8]>>(s: T) -> Result<String, HexStringParseError> {
    Ok(HexString{ inner: s.as_ref().to_vec() }.to_string())
}


pub fn decode<T: AsRef<[u8]>>(s: T) -> Result<String, HexStringParseError> {
    Ok(String::from_utf8(HexString::from_bytes(s)?.inner).unwrap_or_default())
}


#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};
    use super::*;

    #[test]
    fn test_hex_string() {
        assert_eq!(HexString::from_bytes(b"09afAF").unwrap().to_string(), "09afaf");
        assert_eq!(HexString::from_bytes("09afAF".as_bytes()).unwrap().to_string(), "09afaf");
        assert_eq!(HexString::from_bytes("09afAF").unwrap().to_string(), "09afaf");
        assert_eq!(HexString::from_bytes("09afAF".to_string()).unwrap().to_string(), "09afaf");

        assert_eq!(HexString::from_str("09afAF").unwrap().to_string(), "09afaf");

        assert_eq!(HexString::from_str("0").is_err(), true);
        assert_eq!(HexString::from_str("0g").is_err(), true);
        assert_eq!(HexString::from_str("0G").is_err(), true);
        assert_eq!(HexString::from_str("0z").is_err(), true);
        assert_eq!(HexString::from_str("0Z").is_err(), true);

        let mut value = HexString::from_str("09af").unwrap();
        value.push(0x01);
        value.push(0x02);
        value.push_str("ff").unwrap();
        assert_eq!(value.push_str("fg").is_err(), true);
        assert_eq!(value.to_string(), "09af0102ff")
    }

    #[test]
    fn test_hex_string_encode_decode() {
        assert_eq!(encode("jankincai").unwrap(), "6a616e6b696e636169");
        assert_eq!(encode(b"jankincai\x00\xff").unwrap(), "6a616e6b696e63616900ff");
        assert_eq!(encode("jankincai".to_string()).unwrap(), "6a616e6b696e636169");
        assert_eq!(encode("jankincai".as_bytes()).unwrap(), "6a616e6b696e636169");

        assert_eq!(decode("6a616e6b696e636169").unwrap(), "jankincai");
        assert_eq!(decode(b"6a616e6b696e636169").unwrap(), "jankincai");
        assert_eq!(decode("6a616e6b696e636169".to_string()).unwrap(), "jankincai");
        assert_eq!(decode("6a616e6b696e636169".as_bytes()).unwrap(), "jankincai");

        assert_eq!(decode("6a616e6b696e636169fg").is_err(), true);
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct Example {
        pub value: HexString,
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_hex_string_serde() {
        // HexString
        let example = Example {value: HexString::from_str("01afaf").unwrap()};

        let example_string = serde_json::to_string(&example).unwrap();
        assert_eq!(example_string, "{\"value\":\"01afaf\"}");
        let example_new: Example = serde_json::from_str(&example_string).unwrap();
        assert_eq!(example, example_new);
    }
}