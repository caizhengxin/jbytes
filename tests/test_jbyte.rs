use jbytes::{ByteDecode, BufRead};
use jbytes_derive::ByteDecode;


#[derive(Debug, ByteDecode)]
pub struct Example {
    pub a: u16,
}


#[test]
fn test_jbyte() {
    // println!("{:?}", Example { a:1 }.hello_world());
    let value = 0x01_u128;
    println!(">>> {:?}", (value as u16).to_be_bytes());
    println!(">>> {:?}", (value as u16).to_le_bytes());
    println!(">>> {:?}", (value as u16).to_ne_bytes());
    println!(">>> {:?}", core::mem::size_of::<usize>());

    let mut value = vec![0x01_u8];
    value.resize(5, 0);
    println!("{:?}", value[2..4].clone_from_slice(&[0x01, 0x02]));
    let n = 0x01_u64;
    println!(">>> {:?}", core::mem::size_of_val(&n).checked_sub(10));
    // let mut buf = [0; 1024];
    // buf.copy_from_slice(src)

    let mut value = std::collections::VecDeque::new();
    value.push_back(1_u8);
    value.push_back(2_u8);
    println!("{:?}", value.as_slices());
    value.drain(..1);
    println!("{:?}", value);
}
