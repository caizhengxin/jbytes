
use jbytes_derive::{ByteEncode, ByteDecode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct SimpleExample {
    pub version: u8,
    pub cmd: u8,
    #[jbytes(branch="cmd")]
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum SimpleExampleBody {
    #[jbytes(branch_value=1)]
    Stop,                             // Set 1
    Start,                            // Increment to 2
    #[jbytes(branch_value="4..=7")]
    Read {
        address: u16,
        length: u16,
    },                                // Set 4..=7, including: 4、5、6、7
    Write {
        address: u16,
        length: u16,
        #[jbytes(count="length")]
        data: Vec<u8>,
    },                                // Increment to 8
    #[jbytes(branch_default)]
    Unknown {
        address: u16,
    },                          // _ => { ... }
}


fn main() -> JResult<()> {
    // Start 
    let input = [
        0x01,                   // version
        0x02,                   // cmd => Start
    ];
    let value: SimpleExample = jbytes::decode(&input)?;
    assert_eq!(value, SimpleExample {
        version: 1,
        cmd: 2,
        body: SimpleExampleBody::Start
    });
    assert_eq!(*jbytes::encode(value)?, input);

    // Read 
    let input = [
        0x01,                   // version
        0x05,                   // cmd => Read
        0x00, 0x01,             // Read, address
        0x00, 0x0a,             // Read, length
    ];
    let value: SimpleExample = jbytes::decode(&input)?;
    assert_eq!(value, SimpleExample {
        version: 1,
        cmd: 5,
        body: SimpleExampleBody::Read {
            address: 0x0001,
            length: 0x000a,
        }
    });
    assert_eq!(*jbytes::encode(value)?, input);

    // Write
    let input = [
        0x01,                   // version
        0x08,                   // cmd => Write
        0x00, 0x01,             // Write, address
        0x00, 0x03,             // Write, length
        0x01, 0x02, 0x03,       // Write, data
    ];
    let value: SimpleExample = jbytes::decode(&input)?;
    assert_eq!(value, SimpleExample {
        version: 1,
        cmd: 8,
        body: SimpleExampleBody::Write {
            address: 0x0001,
            length: 0x0003,
            data: vec![0x01, 0x02, 0x03],
        }
    });
    assert_eq!(*jbytes::encode(value)?, input);

    // Unknown
    let input = [
        0x01,                   // version
        0x09,                   // cmd => Unknown
        0x00, 0x01,             // Unknown, address
    ];
    let value: SimpleExample = jbytes::decode(&input).unwrap();
    assert_eq!(value, SimpleExample {
        version: 1,
        cmd: 9,
        body: SimpleExampleBody::Unknown {
            address: 0x0001,
        }
    });
    assert_eq!(*jbytes::encode(value)?, input);

    Ok(())
}