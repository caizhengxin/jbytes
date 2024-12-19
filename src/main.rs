use jbytes::{BufRead, Buffer};


fn main() {
    let now = std::time::Instant::now();
    for _i in 0..1000000 {
        let mut buffer = Buffer::new(vec![0x00, 0x01, 0x02]);
        buffer.take_bytes(3).unwrap();
    }
    println!("time: {:?}", now.elapsed());

    // let now = std::time::Instant::now();
    // for _i in 0..1000000 {
    //     let mut buffer = jbyte::Buffer::new(vec![0x00, 0x01, 0x02]);
    //     buffer.take_u16().unwrap();
    // }
    // println!("time: {:?}", now.elapsed());
}