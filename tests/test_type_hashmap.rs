#[cfg(feature = "std")]
use std::collections::HashMap;
#[allow(unused_imports)]
use jbytes_derive::{BorrowByteEncode, BorrowByteDecode};
#[allow(unused_imports)]
use jbytes::prelude::*;


#[cfg(feature = "std")]
#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct HashMapExample<'a> {
    #[jbytes(split=b":", linend=b"\r\n", count=3)]
    pub kv: HashMap<&'a str, &'a str>,
}


#[cfg(feature = "std")]
#[test]
fn test_type_hashmap() {
    let mut hashmap_value = HashMap::new();
    hashmap_value.insert("A1", "jkc1");
    hashmap_value.insert("A2", "jkc2");
    hashmap_value.insert("A3", "");

    let hashmap_value = HashMapExample { 
        kv: hashmap_value,
    };

    // Note: The HashMap type is out of order

    let mut buffer = Buffer::new();
    let _ = hashmap_value.encode(&mut buffer).unwrap();

    let bytes = Bytes::new((*buffer).clone());
    assert_eq!(HashMapExample::decode(&bytes).unwrap(), hashmap_value);
}