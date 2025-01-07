use jget_derive::Jget;


#[derive(Debug, PartialEq, Eq, Jget)]
pub struct SimpleStructExample {
    pub value: u16,
}


#[derive(Debug, PartialEq, Eq, Jget)]
pub struct SimpleStructExample2 {
    pub value: u16,
    pub data: u16,
}


#[derive(Debug, PartialEq, Eq, Jget)]
pub enum SimpleEnumExample {
    #[jget(get="value:u16")]
    V1(SimpleStructExample),
    #[jget(get="value:u16", get_option="data:Option<u16>")]
    V2(SimpleStructExample2),
}


#[test]
fn test_enum_struct() {
    let value = SimpleEnumExample::V2(SimpleStructExample2 { value: 1, data: 2 });
    assert_eq!(value.get_value(), 1);
    assert_eq!(value.get_data(), Some(2));
}