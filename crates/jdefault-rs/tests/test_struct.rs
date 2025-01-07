use jdefault_derive::Jdefault;


#[derive(Debug, PartialEq, Eq, Jdefault)]
pub struct StructExample<'a> {
    pub value1: u16,
    #[jd(default=18)]
    pub value2: u16,
    #[jd(default="\"jankincai\".to_string()")]
    pub value3: String,
    #[jd(default="\"jankincai\"")]
    pub value4: &'a str,
    #[jd(default=b"\x00\x01\x02")]
    pub value5: &'a [u8],
    pub body: StructExampleBody,
}


#[derive(Debug, PartialEq, Eq, Jdefault)]
pub struct StructExampleBody {
    #[jd(default=1)]
    pub value: u16,
}


#[test]
fn test_jdefault() {
    let value = StructExample::default();

    assert_eq!(value, StructExample {
        value1: 0,
        value2: 18,
        value3: "jankincai".to_string(),
        value4: "jankincai",
        value5: b"\x00\x01\x02",
        body: StructExampleBody {
            value: 1,
        }
    });
}