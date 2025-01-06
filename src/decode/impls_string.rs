use crate::std::*;
use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
    ErrorKind, make_error,
};
use super::impls_bytes::find_subsequence;


impl ByteDecode for String {
    #[inline]
    fn decode_inner<I: BufRead>(input: &I, cattr: Option<&ContainerAttrModifiers>,
                                               fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let value = find_subsequence(input, cattr, fattr)?;

        match str::from_utf8(value) {
            Ok(v) => Ok(v.to_string()),
            Err(_e) => Err(make_error(input.get_position(), ErrorKind::Fail))
        }
    }
}


impl<'de> BorrowByteDecode<'de> for String {
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let value = find_subsequence(input, cattr, fattr)?;

        match str::from_utf8(value) {
            Ok(v) => Ok(v.to_string()),
            Err(_e) => Err(make_error(input.get_position(), ErrorKind::Fail))
        }
    }
}


impl<'de> BorrowByteDecode<'de> for &'de str {
    #[inline]
    fn decode_inner<I: BufRead>(input: &'de I, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
    where 
        Self: Sized
    {
        let value = find_subsequence(input, cattr, fattr)?;

        match str::from_utf8(value) {
            Ok(v) => Ok(v),
            Err(_e) => Err(make_error(input.get_position(), ErrorKind::Fail))
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::std::*;
    use crate::{
        Bytes, BufRead, BorrowByteDecode,
        FieldAttrModifiers,
    };

    #[test]
    fn test_decode_string() {
        // test default modifier example
        let bytes = Bytes::new(b"\x03abc");
        assert_eq!(String::decode(&bytes).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);

        // test default error example
        let bytes = Bytes::new(b"\x05abc");
        assert_eq!(String::decode(&bytes).is_err(), true);
        assert_eq!(bytes.remaining_len(), 3);

        // test length example
        let bytes = Bytes::new(b"abc");
        let fattr = FieldAttrModifiers {
            length: Some(3),
            ..Default::default()
        };
        assert_eq!(String::decode_inner(&bytes, None, Some(&fattr)).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);

        // test byte_count example
        let bytes = Bytes::new(b"\x00\x03abc");
        let fattr = FieldAttrModifiers {
            byte_count: Some(2),
            ..Default::default()
        };
        assert_eq!(String::decode_inner(&bytes, None, Some(&fattr)).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);

        // test linend example
        let bytes = Bytes::new(b"abc\r\n");
        let fattr = FieldAttrModifiers {
            linend_value: Some(b"\r\n"),
            ..Default::default()
        };
        assert_eq!(String::decode_inner(&bytes, None, Some(&fattr)).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);

        // test remaining example
        let bytes = Bytes::new(b"abc");
        let fattr = FieldAttrModifiers {
            remaining: true,
            ..Default::default()
        };
        assert_eq!(String::decode_inner(&bytes, None, Some(&fattr)).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);
    }

    #[test]
    fn test_decode_str() {
        // test default modifier example
        let bytes = Bytes::new(b"\x03abc");
        assert_eq!(<&str>::decode(&bytes).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);

        // test default error example
        let bytes = Bytes::new(b"\x05abc");
        assert_eq!(<&str>::decode(&bytes).is_err(), true);
        assert_eq!(bytes.remaining_len(), 3);

        // test length example
        let bytes = Bytes::new(b"abc");
        let fattr = FieldAttrModifiers {
            length: Some(3),
            ..Default::default()
        };
        assert_eq!(<&str>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);

        // test byte_count example
        let bytes = Bytes::new(b"\x00\x03abc");
        let fattr = FieldAttrModifiers {
            byte_count: Some(2),
            ..Default::default()
        };
        assert_eq!(<&str>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);

        // test linend example
        let bytes = Bytes::new(b"abc\r\n");
        let fattr = FieldAttrModifiers {
            linend_value: Some(b"\r\n"),
            ..Default::default()
        };
        assert_eq!(<&str>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);

        // test remaining example
        let bytes = Bytes::new(b"abc");
        let fattr = FieldAttrModifiers {
            remaining: true,
            ..Default::default()
        };
        assert_eq!(<&str>::decode_inner(&bytes, None, Some(&fattr)).unwrap(), "abc");
        assert_eq!(bytes.remaining_len(), 0);
    }
}