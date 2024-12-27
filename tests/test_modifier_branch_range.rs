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
    #[jbytes(branch_range="1..=3")]
    V0,                               // Set 1,2,3
    V1 {                              // Increment to 4
        a: u8,
        b: u16,
    },
    #[jbytes(branch_default)]         // match xxx { _ => xxxx }
    Unknown,
}


#[test]
fn test_modifier_branch_value() {
    // cmd = 1
    let data = [
        0x00, 0x01,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 1, body: BranchValueExampleBody::V0 };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // cmd = 2
    let data = [
        0x00, 0x02,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 2, body: BranchValueExampleBody::V0 };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // cmd = 3
    let data = [
        0x00, 0x03,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 3, body: BranchValueExampleBody::V0 };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // cmd = 4
    let data = [
        0x00, 0x04,                   // cmd
        0x01,                         // a
        0x00, 0x02,                   // b
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 4, body: BranchValueExampleBody::V1 { a: 1, b: 2 } };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // Unknown
    let data = [
        0x00, 0x05,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 5, body: BranchValueExampleBody::Unknown};
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
