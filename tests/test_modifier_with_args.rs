use jbytes_derive::{ByteEncode, ByteDecode};
use jbytes::prelude::*;


/// This is just a demonstration, so go straight back.
fn custom_with_decode<I: BufRead>(input: &I, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>, count: u8) -> JResult<u32> {
    Ok(input.take_uint(count.into())? as u32)
}


fn custom_with_encode<B: BufWrite>(buffer: &mut B, _cattr: Option<&ContainerAttrModifiers>, _fattr: Option<&FieldAttrModifiers>, value: &u32, count: u8) -> JResult<usize> {
    buffer.push_uint(*value as u64, count.into())
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct WithExample {
    pub count: u8,
    #[jbytes(decode_with="custom_with_decode", encode_with="custom_with_encode", with_args="count")]
    pub value: u32,
}


#[test]
fn test_modifier_with() {
    let bytes = Bytes::new(b"\x03\x00\x00\x01");
    assert_eq!(WithExample::decode(&bytes).unwrap(), WithExample { count: 3, value: 1 });
    assert_eq!(bytes.remaining_len(), 0);

    assert_eq!(*jbytes::encode(WithExample { count: 3, value: 1 }).unwrap(), b"\x03\x00\x00\x01");
}