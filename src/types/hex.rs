// use core::{
//     fmt,
//     ops,
//     str, str::FromStr,
// };
// #[cfg(feature = "serde")]
// use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error as DeError};
// use crate::errors::ThisError;


// #[derive(Debug, ThisError)]
// pub enum HexParseError {
//     #[error("invalid hex string: `{0}`")]
//     InvalidHexString(String),
// }


// #[derive(Debug, PartialEq, Eq, Clone, Hash)]
// pub struct Hex<T> {
//     inner: T,
// }


// const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";
// const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";


// #[inline]
// fn is_hex(value: u8) -> Option<u8> {
//     match value {
//         b'0'..=b'9' => Some(value - 0x30),
//         b'a'..=b'f' => Some(value - 0x57),
//         b'A'..=b'F' => Some(value - 0x37),
//         _ => None,
//     }
// }


// #[inline]
// fn parse_hex<T: AsRef<[u8]>>(t: T) -> Result<Vec<u8>, HexParseError> {
//     let s = t.as_ref();

//     if s.len() % 2 != 0 {
//         return Err(HexParseError::InvalidHexString(str::from_utf8(s).unwrap_or_default().to_string()));
//     }

//     let mut vlist = vec![];

//     for v in s.chunks(2) {
//         if let Some(v0) = is_hex(v[0]) {
//             if let Some(v1) = is_hex(v[1]) {
//                 vlist.push(v0 << 4 | v1);
//             }
//             else {
//                 return Err(HexParseError::InvalidHexString(str::from_utf8(s).unwrap_or_default().to_string()));
//             }
//         }
//         else {
//             return Err(HexParseError::InvalidHexString(str::from_utf8(s).unwrap_or_default().to_string()));
//         }
//     }

//     Ok(vlist)
// }


// impl<T> Hex<T> {
//     pub fn new(t: T) -> Self {
//         Self{ inner: t }
//     }
// }


// impl<T: AsRef<[u8]>> Hex<T> {
//     pub fn from_bytes<T: AsRef<[u8]>>(t: T) -> Result<Self, HexParseError> {
//         Ok(Self { inner: parse_hex(t)? })
//     }

//     pub fn push_str(&mut self, s: &str) -> Result<(), HexParseError> {
//         let value = parse_hex(s)?;

//         self.inner.extend(value);

//         Ok(())
//     }

//     pub fn to_hex_lowercase(&self) -> String {
//         let mut vstring = String::new();

//         for v in &self.inner {
//             vstring.push(HEX_LOWER[(*v >> 4) as usize].into());
//             vstring.push(HEX_LOWER[(*v & 0xf) as usize].into());
//         }

//         vstring
//     }

//     pub fn to_hex_uppercase(&self) -> String {
//         let mut vstring = String::new();

//         for v in &self.inner {
//             vstring.push(HEX_UPPER[(*v >> 4) as usize].into());
//             vstring.push(HEX_UPPER[(*v & 0xf) as usize].into());
//         }

//         vstring
//     }
// }


// impl<T: AsMut<[u8]>> Hex<T> {
//     pub fn push(&mut self, value: u8) {
//         self.inner.as_mut()
//     }
// }


// impl FromStr for HexString {
//     type Err = HexParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self { inner: parse_hex(s)? })
//     }
// }


// impl fmt::Display for HexString {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.to_hex_lowercase())
//     }
// }


// impl ops::Deref for HexString {
//     type Target = [u8];

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }


// impl ops::DerefMut for HexString {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }