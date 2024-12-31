use jbytes_derive::{ByteEncode, ByteDecode};
use jbytes::prelude::*;


mod custom_with {
    use jbytes::prelude::*;
    use super::WithExample;


    /// This is just a demonstration, so go straight back.
    pub fn decode<I: BufRead>(input: &I, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>) -> JResult<WithExample> {
        let key = match input.take_bytes(3)? {
            b"get" => 1,
            b"put" => 2,
            _ => return Err(make_error(input.get_position(), ErrorKind::Fail)),
        };

        let value = input.take_u32()?;

        Ok(WithExample {key, value})
    }


    pub fn encode<B: BufWrite>(buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>, value: &WithExample) -> JResult<usize> {
        let mut r_nbytes = match value.key {
            1 => buffer.push_bytes(b"get")?,
            2 => buffer.push_bytes(b"put")?,
            _ => 0,
        };

        r_nbytes += buffer.push_u32(value.value)?;

        Ok(r_nbytes)
    }
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[jbytes(with="custom_with")]
pub struct WithExample {
    pub key: u8,
    pub value: u32,
}


#[test]
fn test_modifier_with() {
    let bytes = Bytes::new(b"get\x00\x00\x00\x01");
    assert_eq!(WithExample::decode(&bytes).unwrap(), WithExample { key: 1, value: 1 });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(WithExample { key: 1, value: 1 }).unwrap(), b"get\x00\x00\x00\x01");
}