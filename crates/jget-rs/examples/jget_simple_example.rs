use jget_derive::Jget;


#[derive(Debug, Jget)]
pub enum SimpleEnumExample {
    Read {
        #[jget(get_option)]
        value: u8,
    },
    Write {
        #[jget(get_option)]
        value: u8,
        #[jget(get_option)]
        data: u16,
    }
}


fn main() {
    let value = SimpleEnumExample::Read { value: 1 };
    assert_eq!(value.get_value(), Some(1));
    assert_eq!(value.get_data(), None);

    let value = SimpleEnumExample::Write { value: 1, data: 2 };
    assert_eq!(value.get_value(), Some(1));
    assert_eq!(value.get_data(), Some(2));
}
