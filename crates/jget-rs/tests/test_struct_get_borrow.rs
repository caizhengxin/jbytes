use jget_derive::Jget;


#[derive(Debug, Default, Jget)]
pub struct JgetBorrowExample<'a> {
    #[jget(get)]
    pub value1: &'a str,
    #[jget(get="value2:&'a str")]
    pub value2: &'a str,
    #[jget(get="value3:String")]
    pub value3: &'a str,

    // option
    #[jget(get_option="value4:&'a str")]
    pub value4: &'a str,
    #[jget(get_option="value5:String")]
    pub value5: &'a str,
}


#[test]
fn test_jget_borrow() {
    let value = JgetBorrowExample {
        value1: "value1",
        value2: "value2",
        value3: "value3",
        value4: "value4",
        value5: "value5",
    };

    assert_eq!(value.get_value1(), "value1");
    assert_eq!(value.get_value2(), "value2");
    assert_eq!(value.get_value3(), "value3".to_string());
    assert_eq!(value.get_value4(), Some("value4"));
    assert_eq!(value.get_value5(), Some("value5".to_string()));
}


#[derive(Debug, Default, Jget)]
pub struct JgetBorrowExample2<'a> {
    #[jget(get_option)]
    pub value1: Option<&'a str>, 
    #[jget(get_option="value2:&'a str")]
    pub value2: Option<&'a str>,
    #[jget(get_option="value3:String")]
    pub value3: Option<&'a str>,
}


#[test]
fn test_jget_borrow2() {
    let value = JgetBorrowExample2 {
        value1: Some("value1"),
        value2: Some("value2"),
        value3: Some("value3"),
    };

    assert_eq!(value.get_value1(), Some(Some("value1")));
    assert_eq!(value.get_value2(), Some("value2"));
    assert_eq!(value.get_value3(), Some("value3".to_string()));
}
