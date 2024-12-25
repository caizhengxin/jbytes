use crate::{
    JResult, BufRead,
    BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
};


#[inline]
pub(super) fn find_subsequence<'da, 'db, T: BufRead>(input: &'da T, cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8]> {
    let value;

    if let Some(fr) = fattr {
        // // skip key
        // if let Some(key) = fr.key {
        //     _ = input.find_subsequence_needle(key, true)?;
        // }

        // // skip split
        // if let Some(split) = fr.split {
        //     _ = input.find_subsequence_needle(split, true)?;
        // }

        if let Some(length) = fr.length {
            value = input.take_bytes(length)?;
        }
        else if let Some(byte_count) = fr.byte_count {
            value = input.take_bytes(input.take_byteorder_uint(byte_count, get_byteorder(cattr, fattr))? as usize)?;
        }
        else if fr.linend {
            value = input.find_subsequences(&["\r\n", "\n", "\x00"])?;
        }
        else if let Some(linend_value) = fr.linend_value {
            value = input.find_subsequence(linend_value)?;
        }
        else if fr.remaining {
            value = input.take_bytes(input.remaining_len())?;
        }
        else {
            value = input.take_bytes(input.take_u8()? as usize)?;
        }
    }
    else {
        value = input.take_bytes(input.take_u8()? as usize)?;
    }

    Ok(value)
}


impl<'de> BorrowByteDecode<'de> for &'de [u8] {
    #[inline]
    fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        find_subsequence(input, cattr, fattr)
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::{
        Bytes, BufRead, BorrowByteDecode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_decode_bytes() {
        // test default modifier example
        let bytes = Bytes::new([0x02, 0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(<&[u8]>::decode(&bytes).unwrap(), &[0x01, 0x02]);
        assert_eq!(bytes.remaining_len(), 3);

        // test default error example
        let bytes = Bytes::new([0x06, 0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(<&[u8]>::decode(&bytes).is_err(), true);
        assert_eq!(bytes.remaining_len(), 5);

        // test length example
        let bytes = Bytes::new([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00]);
        let fattr = FieldAttrModifiers {
            length: Some(5),
            ..Default::default()
        };
        assert_eq!(<&[u8]>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), &[0x00, 0x01, 0x02, 0x03, 0x04]);
        assert_eq!(bytes.remaining_len(), 2);

        // test byte_count example
        let bytes = Bytes::new([0x00, 0x04, 0x02, 0x03, 0x04, 0x05, 0x00]);
        let fattr = FieldAttrModifiers {
            byte_count: Some(2),
            ..Default::default()
        };
        assert_eq!(<&[u8]>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), &[0x02, 0x03, 0x04, 0x05]);
        assert_eq!(bytes.remaining_len(), 1);

        // test remaining example
        let bytes = Bytes::new([0x00, 0x04, 0x02]);
        let fattr = FieldAttrModifiers {
            remaining: true,
            ..Default::default()
        };
        assert_eq!(<&[u8]>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), &[0x00, 0x04, 0x02]);
        assert_eq!(bytes.remaining_len(), 0);

    //     // test key and split
    //     let bytes = Bytes::new([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00]);
    //     let fattr = FieldAttrModifiers {
    //         key: Some(&[0x00, 0x01]),
    //         split: Some(&[0x02]),
    //         linend: true,
    //         ..Default::default()
    //     };
    //     assert_eq!(<&[u8]>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), &[0x03, 0x04, 0x05]);
    //     assert_eq!(bytes.remaining_len(), 0);

    //     // test key and split
    //     let bytes = Bytes::new([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00]);
    //     let fattr = FieldAttrModifiers {
    //         key: Some(&[0x00, 0x01]),
    //         split: Some(&[0x03]),
    //         linend: true,
    //         ..Default::default()
    //     };
    //     assert_eq!(<&[u8]>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), &[0x04, 0x05]);
    //     assert_eq!(bytes.remaining_len(), 0);

    //     // test key and split
    //     let bytes = Bytes::new([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00]);
    //     let fattr = FieldAttrModifiers {
    //         key: Some(&[0x00, 0x01]),
    //         split: Some(&[0x02]),
    //         linend_value: Some(&[0x00]),
    //         ..Default::default()
    //     };
    //     assert_eq!(<&[u8]>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), &[0x03, 0x04, 0x05]);
    //     assert_eq!(bytes.remaining_len(), 0);
    }
}