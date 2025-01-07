use jget_derive::Jget;


#[derive(Debug, Jget)]
#[jget(get_default="length:u16,size:u32")]
pub enum EnumExample {
    Read {
        value: u8,
    },
    Write {
        value: u8,
        data: u16,
    }
}


#[test]
fn test_jget_enum_example() {
    let value = EnumExample::Read { value: 1 };
    assert_eq!(value.get_length(), None);
    assert_eq!(value.get_size(), None);
}