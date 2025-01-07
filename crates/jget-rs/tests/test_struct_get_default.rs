use jget_derive::Jget;


#[derive(Debug, Jget)]
#[jget(get_default="length:u16,size:u32")]
pub struct JgetDefaultExample {
}


#[test]
fn test_jget_struct_example() {
    let value = JgetDefaultExample { };
    assert_eq!(value.get_length(), None);
    assert_eq!(value.get_size(), None);
}
