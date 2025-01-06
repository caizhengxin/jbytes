use crate::{
    JResult, BufWrite,
    BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
};


#[inline]
pub(super) fn encode_inner<B: BufWrite>(buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                            fattr: Option<&FieldAttrModifiers>, data: &[u8]) -> JResult<usize> {
    let mut r_nbytes = 0;
    let mut linend_value = None;

    if let Some(fr) = fattr {
        if let Some(key) = fr.key {
            r_nbytes += buffer.push(key)?;
        }

        if let Some(split) = fr.split {
            r_nbytes += buffer.push(split)?;
        }

        if let Some(_length) = fr.length {
        }
        else if let Some(byte_count) = fr.byte_count {
            r_nbytes += buffer.push_byteorder_uint(data.len() as u64, byte_count, get_byteorder(cattr, fattr))?;
        }
        else if fr.linend {
            linend_value = Some(b"\r\n".as_ref());
        }
        else if fr.linend_value.is_some() {
            linend_value = fr.linend_value;
        }
        else if fr.remaining {

        }
        else {
            r_nbytes += buffer.push_u8(data.len() as u8)?;
        }
    }
    else {
        r_nbytes += buffer.push_u8(data.len() as u8)?;
    }

    r_nbytes += buffer.push(data)?;

    if let Some(linend_value) = linend_value {
        r_nbytes += buffer.push(linend_value)?;
    }

    Ok(r_nbytes)
}


impl<'de> BorrowByteEncode for &'de [u8] {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        encode_inner(buffer, cattr, fattr, self)
    }
}


#[cfg(test)]
mod tests {
    use crate::std::*;
    use crate::{
        Buffer, BorrowByteEncode, FieldAttrModifiers,
    };

    #[test]
    fn test_encode_bytes() {
        // test default modifier example
        let mut buffer = Buffer::new();
        let value = b"\x01\x02".as_ref();
        assert_eq!(value.encode(&mut buffer).unwrap(), 3);
        assert_eq!(*buffer, vec![0x02, 0x01, 0x02]);

        // test length example
        let fattr = FieldAttrModifiers {
            length: Some(5),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        let value = b"\x01\x02\x03\x04\x05".as_ref();
        assert_eq!(value.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 5);
        assert_eq!(*buffer, vec![0x01, 0x02, 0x03, 0x04, 0x05]);

        // test byte_count example
        let fattr = FieldAttrModifiers {
            byte_count: Some(2),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        let value = b"\x01\x02\x03\x04\x05".as_ref();
        assert_eq!(value.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 7);
        assert_eq!(*buffer, vec![0x00, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05]);

        // test remaining example
        let fattr = FieldAttrModifiers {
            remaining: true,
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        let value = b"\x01\x02\x03\x04\x05".as_ref();
        assert_eq!(value.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 5);
        assert_eq!(*buffer, vec![0x01, 0x02, 0x03, 0x04, 0x05]);

        // test key and split
        let fattr = FieldAttrModifiers {
            key: Some(b"\xff\xfe"),
            split: Some(b"\xfa"),
            linend_value: Some(b"\xff"),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        let value = b"\x01\x02\x03\x04\x05".as_ref();
        assert_eq!(value.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 9);
        assert_eq!(*buffer, vec![0xff, 0xfe, 0xfa, 0x01, 0x02, 0x03, 0x04, 0x05, 0xff]);
    }
}