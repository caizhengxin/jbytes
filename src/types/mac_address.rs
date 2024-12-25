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


#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct MacAddress([u8; 6]);


impl MacAddress {
    #[inline]
    pub const fn new(v: [u8; 6]) -> Self {
        Self(v)
    }

    #[inline]
    pub fn to_bits(&self) -> u64 {
        let mut value: u64 = 0;

        for v in self.0 {
            value = (value << 8) + v as u64;
        }

        value
    }

    #[inline]
    pub fn from_bits(v: u64) -> Self {
        let mut mac = Self::default();
        let mut v = v;

        for i in 0..6 {
            mac.0[5 - i] = v as u8;
            v >>= 8;
        }

        mac
    }

    #[inline]
    pub fn is_broadcast(&self) -> bool {
        let mac = self.0;

        if mac[0] == 0xff &&
           mac[1] == 0xff &&
           mac[2] == 0xff &&
           mac[3] == 0xff &&
           mac[4] == 0xff &&
           mac[5] == 0xff
        {
            return true;
        }

        false
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        let mac = self.0;

        if mac[0] == 0x00 &&
           mac[1] == 0x00 &&
           mac[2] == 0x00 &&
           mac[3] == 0x00 &&
           mac[4] == 0x00 &&
           mac[5] == 0x00
        {
            return true;
        }

        false
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
        let mac_str = "aa:bb:cc:dd:ee:ff";
        let mac = MacAddress::from_str(mac_str).unwrap();
        assert_eq!(mac.to_string(), mac_str);

        assert_eq!(MacAddress::from_str("").is_ok(), false);
        assert_eq!(MacAddress::from_str(":").is_ok(), false);
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:").is_ok(), false);
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:f").is_ok(), false);
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:ff:").is_ok(), false);
        assert_eq!(MacAddress::from_str("aa:bb:cc:dd:ee:fff").is_ok(), false);

        assert_eq!(MacAddress::from_str("ff:ff:ff:ff:ff:ff").unwrap().is_broadcast(), true);
        assert_eq!(MacAddress::from_str("ff:ff:ff:ff:ff:ef").unwrap().is_broadcast(), false);
        assert_eq!(MacAddress::from_str("ff:ff:ff:ff:ff:ef").unwrap().is_zero(), false);
        assert_eq!(MacAddress::from_str("00:00:00:00:00:00").unwrap().is_zero(), true);

        assert_eq!(MacAddress::from_str("00:00:00:00:00:00").unwrap().to_bits(), 0);
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
