use jget_derive::Jget;


#[derive(Debug, PartialEq, Eq, Jget)]
pub struct SimpleStructExample {
    pub value: u16,
}


#[derive(Debug, PartialEq, Eq, Jget)]
pub struct SimpleStructExample2 {
    pub value: u16,
    pub data: u16,
    #[jget(get)]
    pub other: String,
}


#[derive(Debug, PartialEq, Eq, Jget)]
pub enum SimpleEnumExample {
    #[jget(get_option="value:Option<u16>")]
    V1(SimpleStructExample),
    #[jget(get_option="value:Option<u16>, data:Option<u16>, get_other():Option<String>")]
    V2(SimpleStructExample2),
}


fn main() {
    let value = SimpleEnumExample::V2(SimpleStructExample2 { value: 1, data: 2, other: "123".to_string() });
    assert_eq!(value.get_value(), Some(1));
    assert_eq!(value.get_data(), Some(2));
    assert_eq!(value.get_other(), Some("123".to_string()));

    let value = SimpleEnumExample::V1(SimpleStructExample { value: 1 });
    assert_eq!(value.get_value(), Some(1));
}