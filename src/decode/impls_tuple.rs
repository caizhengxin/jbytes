use crate::{
    JResult, BufRead,
    ByteDecode, BorrowByteDecode,
    ContainerAttrModifiers, FieldAttrModifiers,
};


macro_rules! impls_tuple {
    ($($t:ident),+) => {
        #[allow(non_camel_case_types)]
        impl<$($t: ByteDecode,)+> ByteDecode for ($($t,)+)
        {
            #[inline]
            fn decode_inner<T: BufRead>(input: &T, cattr: Option<&ContainerAttrModifiers>,
                                        fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
                where 
                    Self: Sized
            {
                $(
                    let $t = $t::decode_inner(input, cattr, fattr)?;
                )*
        
                Ok(($($t,)+))
            }
        }


        #[allow(non_camel_case_types)]
        impl<'de, $($t: BorrowByteDecode<'de>,)+> BorrowByteDecode<'de> for ($($t,)+)
        {
            #[inline]
            fn decode_inner<T: BufRead>(input: &'de T, cattr: Option<&ContainerAttrModifiers>,
                                        fattr: Option<&FieldAttrModifiers>) -> JResult<Self>
                where 
                    Self: Sized
            {
                $(
                    let $t = $t::decode_inner(input, cattr, fattr)?;
                )*
        
                Ok(($($t,)+))
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
        Bytes, BufRead, ByteDecode, ByteOrder,
        ContainerAttrModifiers, FieldAttrModifiers,
    };

    #[test]
    fn test_decode_tuple() {
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
        let value: (u8, u8, u16) = ByteDecode::decode(&bytes).unwrap();
        assert_eq!(value, (0x00, 0x01, 0x0002));
        assert_eq!(bytes.remaining_len(), 0);

        let cattr = ContainerAttrModifiers {
            byteorder: Some(ByteOrder::Le),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
        let value: (u16, u16) = ByteDecode::decode_inner(&bytes, Some(&cattr), None).unwrap();
        assert_eq!(value, (0x0100, 0x0200));
        assert_eq!(bytes.remaining_len(), 0);

        let fattr = FieldAttrModifiers {
            byteorder: Some(ByteOrder::Be),
            ..Default::default()
        };
        let bytes = Bytes::new([0x00, 0x01, 0x00, 0x02]);
        let value: (u16, u16) = ByteDecode::decode_inner(&bytes, Some(&cattr), Some(&fattr)).unwrap();
        assert_eq!(value, (0x0001, 0x0002));
        assert_eq!(bytes.remaining_len(), 0);
    }
}