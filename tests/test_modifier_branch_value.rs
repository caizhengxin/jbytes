use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct BranchExample {
    pub cmd: u16,
    #[jbytes(branch="cmd")]
    pub body: BranchValueExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
enum BranchValueExampleBody {
    #[jbytes(branch_value=1)]
    V0,                               // Set 1
    V1(u8),                           // Increment to 2
    #[jbytes(branch_value=5)]
    V2(u8, u16),                      // Set 5
    V3((u8, u16)),                    // Increment to 6
    V4 {                              // Increment to 7
        a: u8,
        b: u16,
    },
    #[jbytes(branch_default)]         // match xxx { _ => xxxx }
    Unknown,
}


#[test]
fn test_modifier_branch_value() {
    // V0
    let data = [
        0x00, 0x01,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 1, body: BranchValueExampleBody::V0 };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // V1
    let data = [
        0x00, 0x02,                   // cmd
        0x01,                         // V1(u8)
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 2, body: BranchValueExampleBody::V1(1) };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // V2
    let data = [
        0x00, 0x05,                   // cmd
        0x01,                         // V2(u8)
        0x00, 0x02,                   // v2(u16)
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 5, body: BranchValueExampleBody::V2(1, 2) };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // V3
    let data = [
        0x00, 0x06,                   // cmd
        0x01,                         // V3(u8)
        0x00, 0x02,                   // v3(u16)
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 6, body: BranchValueExampleBody::V3((1, 2)) };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // V4
    let data = [
        0x00, 0x07,                   // cmd
        0x01,                         // a
        0x00, 0x02,                   // b
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 7, body: BranchValueExampleBody::V4 { a: 1, b: 2 } };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // Unknown
    let data = [
        0x00, 0x08,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 8, body: BranchValueExampleBody::Unknown};
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    let data = [
        0x00, 0x1f,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 0x1f, body: BranchValueExampleBody::Unknown};
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);
}
