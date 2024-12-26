

struct Hex<T> {
    inner: T,
}


impl<T> Hex<T> {
    pub fn new(t: T) -> Self {
        Self { inner: t }
    }
}


impl<T: AsRef<[u8]>> Hex<T> {

}



impl<T: AsMut<[u8]>> Hex<T> {
    pub fn update(&mut self) {
        let value = self.inner.as_mut();
        value[0] = 1;
    }
}


#[test]
fn test_type_char() {
    let value = Hex::new([0x00, 0x01]);
    let value = Hex::new(vec![0x00, 0x01]);
    let value = Hex::new(&b"\x00\x01"[..]);
}