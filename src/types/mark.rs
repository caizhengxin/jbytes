use core::marker::PhantomData;


pub struct Mark<'a>(PhantomData<&'a ()>);


impl<'a> Default for Mark<'a> {
    fn default() -> Self {
        Mark(PhantomData)
    }
}