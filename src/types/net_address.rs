use core::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
    fmt,
};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error as DeError};
use super::MacAddress;
use crate::errors::ThisError;


#[derive(Debug, ThisError)]
pub enum NetAddressParseError {
    #[error("invalid address: `{0}`")]
    InvalidAddress(String),
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum NetAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    Mac(MacAddress),
    Usize(usize),
}


impl NetAddress {
    /// Returns [true] if this is a ipv4 address.
    #[inline]
    pub fn is_ipv4(&self) -> bool {
        matches!(self, Self::V4(_))
    }

    /// Returns [true] if this is a ipv6 address.
    #[inline]
    pub fn is_ipv6(&self) -> bool {
        matches!(self, Self::V6(_))
    }

    /// Returns [true] if this is a mac address.
    #[inline]
    pub fn is_mac(&self) -> bool {
        matches!(self, Self::Mac(_))
    }

    /// Returns [true] if this is a integer address.
    #[inline]
    pub fn is_usize(&self) -> bool {
        matches!(self, Self::Usize(_))
    }

    /// Returns [true] if this is a broadcast address.
    #[inline]
    pub fn is_broadcast(&self) -> bool {
        match self {
            Self::V4(addr) => addr.is_broadcast(),
            Self::Mac(addr) => addr.is_broadcast(),
            _ => false,
        }
    }

    /// Returns [true] if this is a multicast address.
    #[inline]
    pub fn is_multicast(&self) -> bool {
        match self {
            Self::V4(addr) => addr.is_multicast(),
            Self::V6(addr) => addr.is_multicast(),
            Self::Mac(addr) => addr.is_multicast(),
            _ => false,
        }
    }
}


impl Default for NetAddress {
    fn default() -> Self {
        Self::Usize(0)
    }
}


impl FromStr for NetAddress {
    type Err = NetAddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            // Ipv6
            if let Ok(v) = Ipv6Addr::from_str(s) {
                return Ok(Self::V6(v));
            }

            if let Ok(v) = MacAddress::from_str(s) {
                return Ok(Self::Mac(v));
            }
        }
        else if s.contains('.') {
            // Ipv4
            if let Ok(v) = Ipv4Addr::from_str(s) {
                return Ok(Self::V4(v));
            }
        }
        else if let Ok(v) = s.parse::<usize>() {
            return Ok(Self::Usize(v));
        }

        Err(Self::Err::InvalidAddress(s.to_string()))
    }
}


impl fmt::Display for NetAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V4(v) => write!(f, "{v}"),
            Self::V6(v) => write!(f, "{v}"),
            Self::Usize(v) => write!(f, "{v}"),
            Self::Mac(v) => write!(f, "{v}"),
        }   
    }
}


#[cfg(feature = "serde")]
impl Serialize for NetAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}


#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for NetAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value = Deserialize::deserialize(deserializer)?;

        NetAddress::from_str(value).map_err(D::Error::custom)
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};
    use super::*;

    #[test]
    fn test_ppe_address_parse() {
        assert_eq!(NetAddress::from_str("192.168.1.1").unwrap(), NetAddress::V4(Ipv4Addr::from_str("192.168.1.1").unwrap()));
        assert_eq!(NetAddress::from_str("fe80::4159:f7b2:b9ed:968a").unwrap(), NetAddress::V6(Ipv6Addr::from_str("fe80::4159:f7b2:b9ed:968a").unwrap()));
        assert_eq!(NetAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap(), NetAddress::Mac(MacAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap()));
        assert_eq!(NetAddress::from_str("12").unwrap(), NetAddress::Usize(12));

        assert_eq!(NetAddress::from_str("").is_err(), true);
        assert_eq!(NetAddress::from_str(":").is_err(), true);
        assert_eq!(NetAddress::from_str("aa:bb:cc:dd:ee:").is_err(), true);
        assert_eq!(NetAddress::from_str("aa:bb:cc:dd:ee:f").is_err(), true);
        assert_eq!(NetAddress::from_str("aa:bb:cc:dd:ee:ff:").is_err(), true);
        assert_eq!(NetAddress::from_str("aa:bb:cc:dd:ee:fff").is_err(), true);

        assert_eq!(NetAddress::from_str("192.168.1.1345").is_err(), true);
        assert_eq!(NetAddress::from_str("192.168.1.").is_err(), true);
        assert_eq!(NetAddress::from_str("a").is_err(), true);
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct Example {
        pub addr: NetAddress,
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_ppe_address_serde() {
        // NetAddress
        let example = Example {addr: NetAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap()};

        let example_string = serde_json::to_string(&example).unwrap();
        assert_eq!(example_string, "{\"addr\":\"aa:bb:cc:dd:ee:ff\"}");
        let example_new: Example = serde_json::from_str(&example_string).unwrap();
        assert_eq!(example, example_new);
    }
}
