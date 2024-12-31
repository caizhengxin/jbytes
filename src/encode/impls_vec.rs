#![allow(clippy::if_same_then_else)]
use crate::{
    JResult, BufWrite,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
};


impl<T: ByteEncode> ByteEncode for Vec<T> {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        let data_len = self.len();
        let byteorder = get_byteorder(cattr, fattr);
        let mut nbytes = 0;

        if let Some(fr) = fattr {
            if let Some(byte_count) = fr.byte_count_outside {
                nbytes += buffer.push_byteorder_uint(data_len as u64, byte_count, byteorder)?;
            }
            else if fr.count.is_some() { }
            else if fr.try_count.is_some() { }
            else {
                nbytes += buffer.push_u8(data_len as u8)?;
            }
        }
        else {
            nbytes += buffer.push_u8(data_len as u8)?;
        }

        for value in self {
            nbytes += value.encode_inner(buffer, cattr, fattr)?;
        }

        Ok(nbytes)
    }
}


impl<T: BorrowByteEncode> BorrowByteEncode for Vec<T> {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        let data_len = self.len();
        let byteorder = get_byteorder(cattr, fattr);
        let mut nbytes = 0;

        if let Some(fr) = fattr {
            if let Some(byte_count) = fr.byte_count_outside {
                nbytes += buffer.push_byteorder_uint(data_len as u64, byte_count, byteorder)?;
            }
            else if fr.count.is_some() { }
            else if fr.try_count.is_some() { }
            else {
                nbytes += buffer.push_u8(data_len as u8)?;
            }
        }
        else {
            nbytes += buffer.push_u8(data_len as u8)?;
        }

        for value in self {
            nbytes += value.encode_inner(buffer, cattr, fattr)?;
        }

        Ok(nbytes)
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::{
        Buffer, BorrowByteEncode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_encode_vec() {
        // test default example
        let mut buffer = Buffer::new();
        assert_eq!(vec![0x0001_u16, 0x0002].encode(&mut buffer).unwrap(), 5);
        assert_eq!(*buffer, vec![0x02, 0x00, 0x01, 0x00, 0x02]);

        // test little-endian example
        let mut buffer = Buffer::new();
        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        assert_eq!(vec![0x0001_u16, 0x0002].encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 5);
        assert_eq!(*buffer, vec![0x02, 0x01, 0x00, 0x02, 0x00]);

        // test `count` example
        let mut buffer = Buffer::new();
        let fattr = FieldAttrModifiers {
            count: Some(2),
            ..Default::default()
        };
        assert_eq!(vec![0x0001_u16, 0x0002].encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 4);
        assert_eq!(*buffer, vec![0x00, 0x01, 0x00, 0x02]);

        // test `try_count` example
        let mut buffer = Buffer::new();
        let fattr = FieldAttrModifiers {
            try_count: Some(10),
            ..Default::default()
        };
        assert_eq!(vec![0x0001_u16, 0x0002].encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 4);
        assert_eq!(*buffer, vec![0x00, 0x01, 0x00, 0x02]);

        // test `byte_count_outside` example
        let mut buffer = Buffer::new();
        let fattr = FieldAttrModifiers {
            byte_count_outside: Some(2),
            ..Default::default()
        };
        assert_eq!(vec![0x0001_u16, 0x0002].encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 6);
        assert_eq!(*buffer, vec![0x00, 0x02, 0x00, 0x01, 0x00, 0x02]);

        // test `byte_count_outside` little-endian example
        let mut buffer = Buffer::new();
        let fattr = FieldAttrModifiers {
            byte_count_outside: Some(2),
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        assert_eq!(vec![0x0001_u16, 0x0002].encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 6);
        assert_eq!(*buffer, vec![0x02, 0x00, 0x01, 0x00, 0x02, 0x00]);
    }
}