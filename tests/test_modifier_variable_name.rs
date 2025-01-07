use jbytes_derive::{ByteDecode, ByteEncode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct VariableExample {
    pub cmd: u8,
    #[jbytes(variable_name="length")]
    pub length: u8,
    pub body: VariableExampleBody,
    #[jbytes(branch="cmd")]
    pub enum_body: VariableExampleEnumBody,
}


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
#[jbytes(get_variable_name="length")]
pub struct VariableExampleBody {
    #[jbytes(length="length")]
    value: String,
}


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
#[jbytes(get_variable_name="length")]
pub enum VariableExampleEnumBody {
    #[jbytes(branch_value=1)]
    Read {
        #[jbytes(length="length")]
        value: String,
    },
    Write {
        #[jbytes(length="length")]
        value: String,
    },
    #[jbytes(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_variable_name() {
    let bytes = Bytes::new(b"\x01\x02abcd");
    let value = VariableExample::decode(&bytes).unwrap();
    assert_eq!(value, VariableExample {
        cmd: 1,
        length: 2,
        body: VariableExampleBody { value: "ab".to_string() },
        enum_body: VariableExampleEnumBody::Read { value: "cd".to_string() },
    });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(value).unwrap(), b"\x01\x02abcd");
}