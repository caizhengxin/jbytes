use std::collections::HashMap;
use core::hash::Hash;
use core::cmp::Eq;
use crate::{
    JResult, BufWrite,
    ContainerAttrModifiers, FieldAttrModifiers,
};
use super::push_count_and_try_count;


impl<K, V> crate::ByteEncode for HashMap<K, V>
where
    K: crate::ByteEncode + Hash + Eq,
    V: crate::ByteEncode + Hash + Eq,
{
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        let mut k_fattr = None;
        let mut v_fattr = None;

        if let Some(fr) = fattr {
            if fr.split.is_some() {
                k_fattr = Some(FieldAttrModifiers {
                    linend_value: fr.split,
                    ..Default::default()
                });
            }

            if fr.linend_value.is_some() {
                v_fattr = Some(FieldAttrModifiers {
                    linend_value: fr.linend_value,
                    ..Default::default()
                });    
            }
            else if fr.linend {
                v_fattr = Some(FieldAttrModifiers {
                    linend: true,
                    ..Default::default()
                });    
            }
        }
        let k_fattr = k_fattr.as_ref().or(fattr);
        let v_fattr = v_fattr.as_ref().or(fattr);

        let mut r_nbytes = 0;

        r_nbytes += push_count_and_try_count(buffer, cattr, fattr, self.len())?;

        for (key, value) in self {
            r_nbytes += key.encode_inner(buffer, cattr, k_fattr)?;
            r_nbytes += value.encode_inner(buffer, cattr, v_fattr)?;
        }

        Ok(r_nbytes)
    }
}


impl<K, V> crate::BorrowByteEncode for HashMap<K, V>
where
    K: crate::BorrowByteEncode + Hash + Eq,
    V: crate::BorrowByteEncode + Hash + Eq,
{
    #[inline]
    fn encode_inner<B: BufWrite>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        let mut k_fattr = None;
        let mut v_fattr = None;

        if let Some(fr) = fattr {
            if fr.split.is_some() {
                k_fattr = Some(FieldAttrModifiers {
                    linend_value: fr.split,
                    ..Default::default()
                });
            }

            if fr.linend_value.is_some() {
                v_fattr = Some(FieldAttrModifiers {
                    linend_value: fr.linend_value,
                    ..Default::default()
                });    
            }
            else if fr.linend {
                v_fattr = Some(FieldAttrModifiers {
                    linend: true,
                    ..Default::default()
                });    
            }
        }
        let k_fattr = k_fattr.as_ref().or(fattr);
        let v_fattr = v_fattr.as_ref().or(fattr);

        let mut r_nbytes = 0;

        r_nbytes += push_count_and_try_count(buffer, cattr, fattr, self.len())?;

        for (key, value) in self {
            r_nbytes += key.encode_inner(buffer, cattr, k_fattr)?;
            r_nbytes += value.encode_inner(buffer, cattr, v_fattr)?;
        }

        Ok(r_nbytes)
    }
}


#[cfg(test)]
mod tests {
    use crate::{Buffer, BorrowByteEncode};
    use super::*;

    #[test]
    fn test_encode_hashmap_int_and_int() {
        // test default example
        let value = HashMap::from([
            (0x0001_u16, 0x0002_u16),
            (0x0003, 0x0004),
            (0x0005, 0x0006),
        ]);
        let mut buffer = Buffer::new();
        assert_eq!(value.encode(&mut buffer).unwrap(), 13);

        // test count modifier
        let fattr = FieldAttrModifiers {
            count: Some(3),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(value.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 12);

        // test try_count modifier
        let fattr = FieldAttrModifiers {
            try_count: Some(10),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(value.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 12);

        // test byte_count modifier
        let fattr = FieldAttrModifiers {
            byte_count_outside: Some(2),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(value.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 14);
    }

    #[test]
    fn test_encode_hashmap_str_and_str() {
        let value = HashMap::from([
            ("K1", "V1"),
            ("K2", "V2"),
            ("K3", "V3"),
        ]);
        // let data = b"K1: V1\r\nK2: V2\r\nK3: V3\r\n";
        let fattr = FieldAttrModifiers {
            try_count: Some(50),
            split: Some(b": "),
            linend_value: Some(b"\r\n"),
            ..Default::default()
        };
        let mut buffer = Buffer::new();
        assert_eq!(value.encode_inner(&mut buffer, None, Some(&fattr)).unwrap(), 24);
    }
}