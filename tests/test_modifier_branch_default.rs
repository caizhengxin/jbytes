use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct BranchExample {
    pub cmd: u16,
    #[jbytes(branch="cmd")]
    pub body: BranchDefaultExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
enum BranchDefaultExampleBody {
    #[jbytes(branch_value=1)]
    V0,
    #[jbytes(branch_default)]         // match xxx { _ => xxxx }
    Unknown,
}


#[test]
fn test_modifier_branch_default() {
    // cmd = 1
    let data = [
        0x00, 0x01,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 1, body: BranchDefaultExampleBody::V0 };
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    let data = [
        0x00, 0x02,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 0x0002, body: BranchDefaultExampleBody::Unknown};
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    let data = [
        0x00, 0x1f,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample { cmd: 0x1f, body: BranchDefaultExampleBody::Unknown};
    assert_eq!(BranchExample::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct BranchExample2 {
    pub cmd: u16,
    #[jbytes(branch="cmd")]
    pub body: BranchDefaultExampleBody2,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
enum BranchDefaultExampleBody2 {
    #[jbytes(branch_value=1)]
    V0,
    V1,
}


#[test]
fn test_modifier_branch_default2() {
    // cmd = 1
    let data = [
        0x00, 0x01,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample2 { cmd: 1, body: BranchDefaultExampleBody2::V0 };
    assert_eq!(BranchExample2::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // cmd = 2
    let data = [
        0x00, 0x02,                   // cmd
    ];
    let bytes = Bytes::new(data);
    let value = BranchExample2 { cmd: 2, body: BranchDefaultExampleBody2::V1 };
    assert_eq!(BranchExample2::decode(&bytes).unwrap(), value);
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), data);

    // error
    let data = [
        0x00, 0x03,                   // cmd
    ];
    let bytes = Bytes::new(data);
    assert_eq!(BranchExample2::decode(&bytes).is_err(), true);
    assert_eq!(bytes.remaining_len(), 0);
}
