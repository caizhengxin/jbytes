use crate::{
    JResult, BufWrite,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    // get_byteorder,
};
use super::push_count_and_try_count;


impl<T: ByteEncode> ByteEncode for Vec<T> {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        let data_len = self.len();
        let mut r_nbytes = 0;

        r_nbytes += push_count_and_try_count(buffer, cattr, fattr, data_len)?;

        for value in self {
            r_nbytes += value.encode_inner(buffer, cattr, fattr)?;
        }

        Ok(r_nbytes)
    }
}


impl<T: BorrowByteEncode> BorrowByteEncode for Vec<T> {
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        let data_len = self.len();
        let mut r_nbytes = 0;

        r_nbytes += push_count_and_try_count(buffer, cattr, fattr, data_len)?;

        for value in self {
            r_nbytes += value.encode_inner(buffer, cattr, fattr)?;
        }

        Ok(r_nbytes)
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