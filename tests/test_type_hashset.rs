#[cfg(feature = "std")]
use std::collections::HashSet;
#[allow(unused_imports)]
use jbytes_derive::{ByteEncode, ByteDecode};
#[allow(unused_imports)]
use jbytes::prelude::*;


#[cfg(feature = "std")]
#[derive(Debug, PartialEq, Eq, ByteDecode)]
pub struct HashSetExample {
    pub count: u8,
    #[jbytes(count="count")] // or #[jbytes(count=3)]
    pub hashset: HashSet<u16>,
}


#[cfg(feature = "std")]
#[test]
fn test_type_hashset() {
    let data = b"\x03\x00\x01\x00\x02\x00\x02";
    let bytes = Bytes::new(data);
    let value = HashSetExample { count: 3, hashset: HashSet::from([1,2]) };
    assert_eq!(HashSetExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);
}