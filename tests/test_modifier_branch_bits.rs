use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct BranchExample {
    pub cmd: u16,
    #[jbytes(branch="cmd")]
    pub body: BranchBitsExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
enum BranchBitsExampleBody {
    #[jbytes(branch_bits=0x0001)]    // xxxx xxxx xxxx xxx1
    V0,
    #[jbytes(branch_bits=0x0002)]    // xxxx xxxx xxxx xx10
    V1 {
        a: u8,
        b: u16,
    },
    #[jbytes(branch_default)]         // match xxx { _ => xxxx }
    Unknown,
}


#[test]
fn test_modifier_branch_bits() {
    // cmd = 1
    let data = [
        0x00, 0x01,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 1, body: BranchBitsExampleBody::V0 };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // cmd = 2
    let data = [
        0x00, 0x02,                   // cmd
        0x01,                         // a
        0x00, 0x02,                   // b
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 2, body: BranchBitsExampleBody::V1 { a: 1, b: 2 } };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // cmd = 3
    let data = [
        0x00, 0x03,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 3, body: BranchBitsExampleBody::V0 };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // cmd = 0x000a
    let data = [
        0x00, 0x0a,                   // cmd
        0x01,                         // a
        0x00, 0x02,                   // b
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 0x000a, body: BranchBitsExampleBody::V1 { a: 1, b: 2 } };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // Unknown
    let data = [
        0x00, 0x50,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 0x50, body: BranchBitsExampleBody::Unknown};
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    let data = [
        0x00, 0xf0,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 0xf0, body: BranchBitsExampleBody::Unknown};
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);
}
