use core::{
    ops,
    fmt,
};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};


pub type HexBytes<'a> = Hex<&'a [u8]>;


/// A Hex string conversion.
/// 
/// # Examples
/// 
/// ```
/// use jbytes::types::Hex;
/// 
/// let hex = Hex::new(b"\x00\xab\xcd");
/// assert_eq!("00abcd", hex.to_hex_lowercase());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Hex<T> {
    inner: T,
}


const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";
const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";


impl<T: AsRef<[u8]>> Hex<T> {
    /// Constructs a new `Hex<T>`.
    ///
    /// # Examples
    /// 
    /// ```
    /// use jbytes::types::Hex;
    /// 
    /// let hex = Hex::new(b"\x00\xab\xcd");
    /// assert_eq!("00abcd", hex.to_hex_lowercase());
    /// ```
    #[inline]
    pub fn new(t: T) -> Self {
        Self { inner: t }
    }

    /// Returns the lowercase equivalent of this hex slice, as a new [String].
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use jbytes::types::Hex;
    /// 
    /// let hex = Hex::new(b"\x00\xab\xcd");
    /// assert_eq!("00abcd", hex.to_hex_lowercase());
    /// ```
    pub fn to_hex_lowercase(&self) -> String {
        let mut vstring = String::new();

        for v in self.inner.as_ref() {
            vstring.push(HEX_LOWER[(*v >> 4) as usize].into());
            vstring.push(HEX_LOWER[(*v & 0xf) as usize].into());
        }

        vstring
    }

    /// Returns the uppercase equivalent of this hex slice, as a new [String].
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use jbytes::types::Hex;
    /// 
    /// let hex = Hex::new(b"\x00\xab\xcd");
    /// assert_eq!("00ABCD", hex.to_hex_uppercase());
    /// ```
    pub fn to_hex_uppercase(&self) -> String {
        let mut vstring = String::new();

        for v in self.inner.as_ref() {
            vstring.push(HEX_UPPER[(*v >> 4) as usize].into());
            vstring.push(HEX_UPPER[(*v & 0xf) as usize].into());
        }

        vstring
    }
}


impl<T: AsRef<[u8]>> fmt::Display for Hex<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_lowercase())
    }
}


impl<T: AsRef<[u8]>> ops::Deref for Hex<T> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}


impl<T: AsRef<[u8]> + AsMut<[u8]>> ops::DerefMut for Hex<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}


#[cfg(feature = "serde")]
impl<T: AsRef<[u8]>> Serialize for Hex<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}


#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use serde::Serialize;
    use super::*;

    #[test]
    fn test_hex_bytes() {
        assert_eq!(Hex::new(b"\x01\xab\xff").to_hex_lowercase(), "01abff");
        assert_eq!(Hex::new(b"\x01\xab\xff").to_hex_uppercase(), "01ABFF");
        assert_eq!(Hex::new(b"\x01\xab\xff").to_string(), "01abff");
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
    struct Example<T: AsRef<[u8]>> {
        pub value: Hex<T>,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_hex_bytes_serde() {
        let input = b"jankincai";        
        let example = Example {value: Hex::new(input)};

        let example_string = serde_json::to_string(&example).unwrap();
        assert_eq!(example_string, "{\"value\":\"6a616e6b696e636169\"}");
    }
}