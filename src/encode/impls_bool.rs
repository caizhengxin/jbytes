impl crate::ByteEncode for bool {
    fn encode<T: crate::BufWrite>(&self, input: &mut T, _cattr: Option<&crate::ContainerAttrModifiers>, _fattr: Option<&crate::FieldAttrModifiers>) -> crate::JResult<usize> {
        input.push_bool(*self)
    }
}