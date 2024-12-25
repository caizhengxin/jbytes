use crate::{
    JResult, BufWriteMut,
    BorrowByteEncode,
    ContainerAttrModifiers, FieldAttrModifiers,
    get_byteorder,
};


#[inline]
pub(super) fn encode_inner<B: BufWriteMut>(buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                            fattr: Option<&FieldAttrModifiers>, data: &[u8]) -> JResult<usize> {
    let mut r_nbytes = 0;
    let mut linend_value = None;

    // key and split
    if let Some(fr) = fattr {
        if let Some(length) = fr.length {
            r_nbytes += buffer.push_u8(length as u8)?;
        }
        else if let Some(byte_count) = fr.byte_count {
            r_nbytes += buffer.push_byteorder_uint(data.len(), byte_count, get_byteorder(cattr, fattr))?;
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
    fn encode_inner<B: BufWriteMut>(&self, buffer: &mut B, cattr: Option<&ContainerAttrModifiers>,
                                                                  fattr: Option<&FieldAttrModifiers>) -> JResult<usize> {
        encode_inner(buffer, cattr, fattr, self)
    }
}