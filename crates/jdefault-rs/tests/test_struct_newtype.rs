use jdefault_derive::Jdefault;


#[derive(Debug, PartialEq, Eq, Jdefault)]
pub struct SimpleExample(
    #[jd(default=18)]u8,
    #[jd(default=16)]u8
);


#[test]
fn test_jdefault_newtype() {
    let value = SimpleExample::default();
    assert_eq!(value, SimpleExample(18, 16));
}


#[derive(Debug, PartialEq, Eq, Jdefault)]
#[jd(default="(1, 2)")]
pub struct SimpleExample2((u8, u8));


#[test]
fn test_jdefault_newtype2() {
    let value = SimpleExample2::default();
    assert_eq!(value, SimpleExample2((1, 2)));
}