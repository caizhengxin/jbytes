impl crate::ByteEncode for bool {
    fn encode<'da, 'db, 'dc, T: crate::BufWrite>(&'da self, input: &'db mut T, _cattr: Option<&'dc crate::ContainerAttrModifiers>, _fattr: Option<&'dc crate::FieldAttrModifiers>) -> crate::JResult<'db, usize> {
        input.push_bool(*self)
    }
}