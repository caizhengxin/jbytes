use crate::{
    JResult, BufWriteMut,
    ByteEncode, BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    // ByteOrder, get_byteorder,
};


macro_rules! impls_tuple {
    ($($t:ident),+) => {
        #[allow(non_camel_case_types)]
        impl<$($t: ByteEncode,)+> ByteEncode for ($($t,)+)
        {
            #[inline]
            fn encode_inner<T: BufWriteMut>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
            {
                let ($($t,)*) = self;
                let mut nbytes_count = 0;

                $(
                    nbytes_count += $t.encode_inner(buffer, cattr, fattr)?;
                )*

                Ok(nbytes_count)
            }
        }


        #[allow(non_camel_case_types)]
        impl<$($t: BorrowByteEncode,)+> BorrowByteEncode for ($($t,)+)
        {
            #[inline]
            fn encode_inner<T: BufWriteMut>(&self, buffer: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
            {
                let ($($t,)*) = self;
                let mut nbytes_count = 0;

                $(
                    nbytes_count += $t.encode_inner(buffer, cattr, fattr)?;
                )*

                Ok(nbytes_count)
            }
        }
    };

    () => {
        impls_tuple!(t1);
        impls_tuple!(t1, t2);
        impls_tuple!(t1, t2, t3);
        impls_tuple!(t1, t2, t3, t4);
        impls_tuple!(t1, t2, t3, t4, t5);
        impls_tuple!(t1, t2, t3, t4, t5, t6);
        impls_tuple!(t1, t2, t3, t4, t5, t6, t7);
        impls_tuple!(t1, t2, t3, t4, t5, t6, t7, t8);
        impls_tuple!(t1, t2, t3, t4, t5, t6, t7, t8, t9);
    };
}


impls_tuple!();


#[cfg(test)]
mod tests {
    use crate::{
        Buffer, BorrowByteEncode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_encode_tuple() {
        let mut buffer = Buffer::new();
        let value: (u8, u8, u16) = (0x00, 0x01, 0x0002);
        assert_eq!(value.encode(&mut buffer).unwrap(), 4);
        assert_eq!(*buffer, vec![0x00, 0x01, 0x00, 0x02]);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        let value: (u16, u16) = (0x0001, 0x0002);
        assert_eq!(value.encode_inner(&mut buffer, Some(&cattr), None).unwrap(), 4);
        assert_eq!(*buffer, vec![0x01, 0x00, 0x02, 0x00]);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        let value: (u16, u16) = (0x0001, 0x0002);
        assert_eq!(value.encode_inner(&mut buffer, Some(&cattr), Some(&fattr)).unwrap(), 4);
        assert_eq!(*buffer, vec![0x00, 0x01, 0x00, 0x02]);
    }
}