use jget_derive::Jget;


#[derive(Debug, Default, Jget)]
pub struct JgetStringExample {
    #[jget(get)]
    pub value1: String,
    #[jget(get="value2:String")]
    pub value2: String,
    #[jget(get_option="value3:String")]
    pub value3: String,
    #[jget(get="value4:&String")]
    pub value4: String,
    #[jget(get_option="value5:&String")]
    pub value5: String,
}


#[test]
fn test_jget_string_example() {
    let value = JgetStringExample::default();

    assert_eq!(value.get_value1(), "");
    assert_eq!(value.get_value2(), "");
    assert_eq!(value.get_value3(), Some("".to_string()));
    assert_eq!(value.get_value4(), &"");
    assert_eq!(value.get_value5(), Some(&"".to_string()));
}
