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
    // use crate::{decode::ByteDecode, FieldAttrModifiers, ByteOrder};

    // #[test]
    // fn test_tuple_decode() {
    //     let (input, (v1, v2)): (&[u8], (u16, u16)) = ByteDecode::decode(&[0x00, 0x01, 0x00, 0x02], None, None).unwrap();
    //     assert_eq!(input.is_empty(), true);
    //     assert_eq!(v1, 1);
    //     assert_eq!(v2, 2);

    //     let fattr = FieldAttrModifiers { byteorder: Some(ByteOrder::Le), ..Default::default() };
    //     let (input, (v1, v2)): (&[u8], (u16, u16)) = ByteDecode::decode(&[0x01, 0x00, 0x02, 0x00], None, Some(&fattr)).unwrap();
    //     assert_eq!(input.is_empty(), true);
    //     assert_eq!(v1, 1);
    //     assert_eq!(v2, 2);

    //     let (input, (v1, v2)): (&[u8], (u8, u16)) = ByteDecode::decode(&[0x01, 0x00, 0x02], None, None).unwrap();
    //     assert_eq!(input.is_empty(), true);
    //     assert_eq!(v1, 1);
    //     assert_eq!(v2, 2);
    // }
}