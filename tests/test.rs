
#[test]
fn test() {
    println!("{:?}", core::any::type_name::<String>());
    println!("{:?}", core::any::type_name::<&str>());
    println!("{:?}", core::any::type_name::<&[u8]>());
}