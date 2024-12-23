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
            fn encode_inner<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
            {
                let ($($t,)*) = self;
                let mut nbytes_count = 0;

                $(
                    nbytes_count += $t.encode_inner(input, cattr, fattr)?;
                )*

                Ok(nbytes_count)
            }
        }


        #[allow(non_camel_case_types)]
        impl<$($t: BorrowByteEncode,)+> BorrowByteEncode for ($($t,)+)
        {
            fn encode_inner<T: BufWriteMut>(&self, input: &mut T, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>) -> JResult<usize>
            {
                let ($($t,)*) = self;
                let mut nbytes_count = 0;

                $(
                    nbytes_count += $t.encode_inner(input, cattr, fattr)?;
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