use jget_derive::Jget;


#[derive(Debug, Default, Jget)]
pub struct JgetVecExample {
    #[jget(get)]
    pub value1: Vec<u16>,
    #[jget(get="value2:Vec<u16>")]
    pub value2: Vec<u16>,
    #[jget(get_option="value3:Vec<u16>")]
    pub value3: Vec<u16>,
    #[jget(get="value4:&Vec<u16>")]
    pub value4: Vec<u16>,
    #[jget(get_option="value5:&Vec<u16>")]
    pub value5: Vec<u16>,
    #[jget(get="value6:Vec<u32>")]
    pub value6: Vec<u16>,
    #[jget(get_option="value7:Vec<u32>")]
    pub value7: Vec<u16>,
    #[jget(get_option="value8:Vec<u32>")]
    pub value8: Option<Vec<u16>>,
    #[jget(get_option="value9:&Vec<u16>")]
    pub value9: Option<Vec<u16>>,
    #[jget(get_option="value10:Vec<u16>")]
    pub value10: Option<Vec<u16>>,
}


#[test]
fn test_jget_vec_example() {
    let value = JgetVecExample::default();

    assert_eq!(value.get_value1(), vec![]);
    assert_eq!(value.get_value2(), vec![]);
    assert_eq!(value.get_value3(), Some(vec![]));
    assert_eq!(value.get_value4(), &vec![]);
    assert_eq!(value.get_value5(), Some(&vec![]));
}


#[derive(Debug, Default, Jget)]
pub struct JgetVecExample2 {
    #[jget(get="value:Vec<u16>")]
    pub value: u8,
}


#[test]
fn test_jget_vec_example2() {
    let value = JgetVecExample2::default();

    assert_eq!(value.get_value(), vec![0]);
}
