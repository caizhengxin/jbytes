use jbytes::prelude::*;


fn buffer_example(buffer: &mut Buffer) -> JResult<()>  {
    buffer.push_be_u16(1)?;
    buffer.push(b"\x01\x02\x03")?;

    Ok(())
}


fn main() {
    let mut buffer = Buffer::new();
    if buffer_example(&mut buffer).is_ok() {
        assert_eq!(*buffer, b"\x00\x01\x01\x02\x03");
    }
}