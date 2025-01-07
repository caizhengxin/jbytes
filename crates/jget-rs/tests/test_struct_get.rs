use jget_derive::Jget;


#[derive(Debug, Default, Jget)]
pub struct JgetExample {
    #[jget(get)]
    pub value1: u16,
    #[jget(get="value2:u32")]
    pub value2: u16,
    #[jget(get_option="value3:u16")]
    pub value3: u16,
    #[jget(get="value4:u32")]
    pub value: u16,
}


#[test]
fn test_jget_example() {
    let value = JgetExample::default();

    assert_eq!(value.get_value1(), 0 as u16);
    assert_eq!(value.get_value2(), 0 as u32);
    assert_eq!(value.get_value3(), Some(0));
    assert_eq!(value.get_value4(), 0 as u32);
}
