use core::{
    ops,
    fmt,
    str::FromStr,
};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error as DeError};
use crate::errors::ThisError;


#[derive(Debug, ThisError)]
pub enum MacAddressParseError {
    #[error("invalid mac address: `{0}`")]
    InvalidMacAddress(String),
}


/// This is a Ethernet MacAddress type.
/// 
/// # Example
/// 
/// ```
/// use core::str::FromStr;
/// use jbytes::types::MacAddress;
/// assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap().to_bits(), 0xaabbccddeeff);
/// assert_eq!(MacAddress::from_bits(0xaabbccddeeff).to_string(), "aa:bb:cc:dd:ee:ff");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct MacAddress([u8; 6]);


impl MacAddress {
    // Constructs a new MacAddress.
    #[inline]
    pub const fn new(v: [u8; 6]) -> Self {
        Self(v)
    }

    /// Returns a unsigned integer.
    #[inline]
    pub fn to_bits(&self) -> u64 {
        let mut buf = [0; 8];
        buf[2..].clone_from_slice(&self.0);
        u64::from_be_bytes(buf)
    }

    // Converts to this type from the input type.
    #[inline]
    pub fn from_bits(value: u64) -> Self {
        let value = value.to_be_bytes();
        let mut buf = [0; 6];
        buf.clone_from_slice(&value[2..]);
        
        Self (buf)
    }

    // Returns [true] if this is a unicast address (xxxxxxx0 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx).
    #[inline]
    pub fn is_unicast(&self) -> bool {
        self.to_bits() & 0x010000000000 == 0x000000000000
    }

    // Returns [true] if this is a broadcast address (ff:ff:ff:ff:ff:ff).
    #[inline]
    pub fn is_broadcast(&self) -> bool {
        self.to_bits() == 0xffffffffffff
    }

    // Returns [true] if this is a multicast address (xxxxxxx1 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx).
    #[inline]
    pub fn is_multicast(&self) -> bool {
        self.to_bits() & 0x010000000000 == 0x010000000000
    }

    // Returns [true] if this is a address (00:00:00:00:00:00).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.to_bits() == 0
    }
}


impl From<[u8; 6]> for MacAddress {
    fn from(value: [u8; 6]) -> Self {
        Self(value)
    }
}


impl ops::Deref for MacAddress {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl ops::DerefMut for MacAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl FromStr for MacAddress {
    type Err = MacAddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mac = MacAddress::default();

        for (i, v) in s.split(':').enumerate() {
            if i > 5 || v.len() != 2 {
                return Err(MacAddressParseError::InvalidMacAddress(v.to_string()));
            }

            match u8::from_str_radix(v, 16) {
                Ok(v) => {
                    mac.0[i] = v;
                },
                Err(e) => {
                    return Err(MacAddressParseError::InvalidMacAddress(format!("{e:?}: {v:?}")));
                }
            }
        }

        Ok(mac)
    }
}


impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mac = self.0;

        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", mac[0], mac[1], mac[2], mac[3], mac[4], mac[5])        
    }
}


#[cfg(feature = "serde")]
impl Serialize for MacAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}


#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for MacAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value = Deserialize::deserialize(deserializer)?;

        MacAddress::from_str(value).map_err(D::Error::custom)
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};
    use super::MacAddress;

    #[test]
    fn test_parse_mac_address() {
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap().to_string(), "aa:bb:cc:dd:ee:ff");

        assert_eq!(MacAddress::from_str("").is_ok(), false);
        assert_eq!(MacAddress::from_str(":").is_ok(), false);
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:").is_ok(), false);
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:f").is_ok(), false);
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:ff:").is_ok(), false);
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:fff").is_ok(), false);

        // Unicast address
        assert_eq!(MacAddress::from_str("01:aa:bb:cc:dd").unwrap().is_unicast(), false);
        assert_eq!(MacAddress::from_str("00:aa:bb:cc:dd").unwrap().is_unicast(), true);
        assert_eq!(MacAddress::from_str("0c:73:eb:92:53:f0").unwrap().is_unicast(), true);
        // Broadcast address
        assert_eq!(MacAddress::from_str("ff:ff:ff:ff:ff:ff").unwrap().is_broadcast(), true);
        assert_eq!(MacAddress::from_str("ff:ff:ff:ff:ff:ef").unwrap().is_broadcast(), false);
        // Multicast address
        assert_eq!(MacAddress::from_str("01:aa:bb:cc:dd").unwrap().is_multicast(), true);
        assert_eq!(MacAddress::from_str("00:aa:bb:cc:dd").unwrap().is_multicast(), false);

        assert_eq!(MacAddress::from_str("ff:ff:ff:ff:ff:ef").unwrap().is_zero(), false);
        assert_eq!(MacAddress::from_str("00:00:00:00:00:00").unwrap().is_zero(), true);

        // From
        assert_eq!(MacAddress::from([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]).to_string(), "01:02:03:04:05:06");

        // from_bits
        assert_eq!(MacAddress::from_bits(0xaabbccddeeff).to_string(), "aa:bb:cc:dd:ee:ff");

        // to_bits
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap().to_bits(), 0xaabbccddeeff);
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct Example {
        pub mac: MacAddress,
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_mac_address_serde() {
        // MacAddress
        let example = Example {mac: MacAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap()};

        let example_string = serde_json::to_string(&example).unwrap();
        assert_eq!(example_string, "{\"mac\":\"aa:bb:cc:dd:ee:ff\"}");
        let example_new: Example = serde_json::from_str(&example_string).unwrap();
        assert_eq!(example, example_new);
    }
}
