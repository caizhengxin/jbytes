use jget_derive::Jget;


#[derive(Debug, Default, Jget)]
pub struct JgetAttrExample {
    #[jget(get="data.value:u16,data.jkc:u16")]
    pub data: JgetDataExample,
}


#[derive(Debug, Default, Jget)]
pub struct JgetDataExample {
    pub value: u16,
    pub jkc: u16,
}


#[test]
fn test_jget_attr_example() {
    let value = JgetAttrExample::default();
    assert_eq!(value.get_value(), 0);
    assert_eq!(value.get_jkc(), 0);
}


#[derive(Debug, Default, Jget)]
pub struct JgetAttrExample2 {
    #[jget(get_option="data.value:u16,data.jkc:u16")]
    pub data: JgetDataExample,
}


#[test]
fn test_jget_attr_example2() {
    let value = JgetAttrExample2::default();
    assert_eq!(value.get_value(), Some(0));
    assert_eq!(value.get_jkc(), Some(0));
}
