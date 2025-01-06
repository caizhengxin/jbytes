// use jbytes::prelude::*;
use jbytes_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub enum Example {
    #[jbytes(branch_value = 1)]
    Read {
        v1: u8,
        v2: u16,
        v3: u32,
        v4: u64,
        v5: u128,
        #[jbytes(length = 3)]
        v6: u32,
    }
}


#[test]
fn test_type_int() {
    let input = [
        0x01,
        0x01,
        0x00, 0x02,
        0x00, 0x00, 0x00, 0x03,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x06,
    ];
    let mt_value = Example::Read {
        v1: 0x01,
        v2: 0x0002,
        v3: 0x00000003,
        v4: 0x0000000000000004,
        v5: 0x00000000000000000000000000000005,
        v6: 0x000006,
    };

    let value: Example = jbytes::decode(&input).unwrap();
    assert_eq!(value, mt_value);
    assert_eq!(*jbytes::encode(value).unwrap(), input);
}