use core::{
    ops,
    fmt,
    // str::FromStr,
};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HexBytes<'a> {
    inner: &'a [u8],
}


const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";
const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";


impl<'a> HexBytes<'a> {
    pub fn new(v: &'a [u8]) -> Self {
        Self{ inner: v }
    }

    pub fn to_hex_lowercase(&self) -> String {
        let mut vstring = String::new();

        for v in self.inner {
            vstring.push(HEX_LOWER[(*v >> 4) as usize].into());
            vstring.push(HEX_LOWER[(*v & 0xf) as usize].into());
        }

        vstring
    }

    pub fn to_hex_uppercase(&self) -> String {
        let mut vstring = String::new();

        for v in self.inner {
            vstring.push(HEX_UPPER[(*v >> 4) as usize].into());
            vstring.push(HEX_UPPER[(*v & 0xf) as usize].into());
        }

        vstring
    }
}


impl<'a> fmt::Display for HexBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_lowercase())
    }
}


impl<'a> ops::Deref for HexBytes<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


#[cfg(feature = "serde")]
impl<'a> Serialize for HexBytes<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}


#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use serde::Serialize;
    use super::*;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
    struct Example<'a> {
        pub value: HexBytes<'a>,
    }

    #[test]
    fn test_hex_bytes_serde() {
        let input = b"jankincai";        
        let example = Example {value: HexBytes::new(input)};

        let example_string = serde_json::to_string(&example).unwrap();
        assert_eq!(example_string, "{\"value\":\"6a616e6b696e636169\"}");
    }
}