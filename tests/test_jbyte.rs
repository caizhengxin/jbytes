use jbyte::ByteDecode;
use jbyte_derive::ByteDecode;


#[derive(Debug, ByteDecode)]
pub struct Example {
    pub a: u16,
}


#[test]
fn test_jbyte() {
    // println!("{:?}", Example { a:1 }.hello_world());
}
