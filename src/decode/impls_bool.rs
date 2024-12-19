impl crate::ByteDecode for bool {
    fn decode<'da, 'db, T: crate::BufRead>(input: &'da mut T, _cattr: Option<&'db crate::ContainerAttrModifiers>, _fattr: Option<&'db crate::FieldAttrModifiers>) -> crate::JResult<'da, Self>
    where 
        Self: Sized
    {
        input.take_bool()        
    }
}