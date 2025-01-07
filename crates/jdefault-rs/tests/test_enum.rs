use jdefault_derive::Jdefault;


#[derive(Debug, PartialEq, Eq, Jdefault)]
pub enum EnumExample {
    Value,
    #[jd(default)]
    Value2 {
        #[jd(default=3)]
        value1: u8,
        value2: u8,
    }
}


#[test]
fn test_enum_example() {
    let value = EnumExample::default();
    assert_eq!(value, EnumExample::Value2 { value1: 3, value2: 0 });
}


#[derive(Debug, PartialEq, Eq, Jdefault)]
pub enum EnumExample2 {
    #[jd(default)]
    Value,
    Value2 {
        value1: u8,
        value2: u8,
    }
}


#[test]
fn test_enum_example2() {
    let value = EnumExample2::default();
    assert_eq!(value, EnumExample2::Value);
}


#[derive(Debug, PartialEq, Eq, Jdefault)]
pub enum EnumExample3 {
    #[jd(default)]
    Value(#[jd(default=12)]u8),
    Value2 {
        value1: u8,
        value2: u8,
    }
}


#[test]
fn test_enum_example3() {
    let value = EnumExample3::default();
    assert_eq!(value, EnumExample3::Value(12));
}


#[derive(Debug, PartialEq, Eq, Jdefault)]
pub enum EnumExample4 {
    #[jd(default)]
    Value(#[jd(default=12)]u8, u8),
    Value2 {
        value1: u8,
        value2: u8,
    }
}


#[test]
fn test_enum_example4() {
    let value = EnumExample4::default();
    assert_eq!(value, EnumExample4::Value(12, 0));
}


#[derive(Debug, PartialEq, Eq, Jdefault)]
pub enum EnumExample5 {
    #[jd(default="(12, 0)")]
    Value((u8, u8)),
    Value2 {
        value1: u8,
        value2: u8,
    }
}


#[test]
fn test_enum_example5() {
    let value = EnumExample5::default();
    assert_eq!(value, EnumExample5::Value((12, 0)));
}
