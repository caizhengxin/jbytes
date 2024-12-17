use crate::{
    ByteOrder,
    errors::JResult,
};


pub trait BufRead {
    fn remain(&self) -> &'_ [u8];

    fn advance(&mut self, nbyte: usize);

    fn take_bytes(&mut self, nbyte: usize) -> JResult<&'_ [u8]>;

    fn take_int(&mut self, byteorder: ByteOrder, nbyte: u8) -> JResult<u128>;

    #[inline]
    fn take_be_int(&mut self, nbyte: u8) -> JResult<u128> {
        self.take_int(ByteOrder::Be, nbyte)
    }

    #[inline]
    fn take_le_int(&mut self, nbyte: u8) -> JResult<u128> {
        self.take_int(ByteOrder::Le, nbyte)
    }

    #[inline]
    fn take_u8(&mut self) -> JResult<u8> {
        let value = self.take_be_int(1)?;
        Ok(value as u8)
    }

    #[inline]
    fn take_be_u8(&mut self) -> JResult<u8> {
        self.take_u8()
    }

    #[inline]
    fn take_le_u8(&mut self) -> JResult<u8> {
        self.take_u8()
    }

    #[inline]
    fn take_be_u16(&mut self) -> JResult<u16> {
        let value = self.take_be_int(2)?;
        Ok(value as u16)
    }

    #[inline]
    fn take_le_u16(&mut self) -> JResult<u16> {
        let value = self.take_le_int(2)?;
        Ok(value as u16)
    }

    #[inline]
    fn take_be_u24(&mut self) -> JResult<u32> {
        let value = self.take_be_int(3)?;
        Ok(value as u32)
    }

    #[inline]
    fn take_le_u24(&mut self) -> JResult<u32> {
        let value = self.take_le_int(3)?;
        Ok(value as u32)
    }

    #[inline]
    fn take_be_u32(&mut self) -> JResult<u32> {
        let value = self.take_be_int(4)?;
        Ok(value as u32)
    }

    #[inline]
    fn take_le_u32(&mut self) -> JResult<u32> {
        let value = self.take_le_int(4)?;
        Ok(value as u32)
    }

    #[inline]
    fn take_be_u64(&mut self) -> JResult<u64> {
        let value = self.take_be_int(8)?;
        Ok(value as u64)
    }

    #[inline]
    fn take_le_u64(&mut self) -> JResult<u64> {
        let value = self.take_le_int(8)?;
        Ok(value as u64)
    }

    #[inline]
    fn take_be_u128(&mut self) -> JResult<u128> {
        let value = self.take_be_int(16)?;
        Ok(value as u128)
    }

    #[inline]
    fn take_le_u128(&mut self) -> JResult<u128> {
        let value = self.take_le_int(16)?;
        Ok(value as u128)
    }
}


pub trait BufWrite {
    fn push<V: AsRef<[u8]>>(&mut self, value: V);

    #[inline]
    fn push_bytes(&mut self, value: &[u8]) {
        self.push(value)
    }

    #[inline]
    fn push_char(&mut self, value: char) {
        self.push([value as u8]);
    }

    #[inline]
    fn push_u8(&mut self, value: u8) {
        self.push([value])
    }

    #[inline]
    fn push_be_u8(&mut self, value: u8) {
        self.push_u8(value)
    }

    #[inline]
    fn push_le_u8(&mut self, value: u8) {
        self.push_u8(value)
    }

    #[inline]
    fn push_ne_u8(&mut self, value: u8) {
        self.push_u8(value)
    }

    #[inline]
    fn push_u16(&mut self, value: u16) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_be_u16(&mut self, value: u16) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_le_u16(&mut self, value: u16) {
        self.push(value.to_le_bytes())
    }

    #[inline]
    fn push_ne_u16(&mut self, value: u16) {
        self.push(value.to_ne_bytes())
    }

    #[inline]
    fn push_u24(&mut self, value: u32) {
        self.push_be_u24(value);
    }

    #[inline]
    fn push_be_u24(&mut self, value: u32) {
        self.push(&value.to_be_bytes()[1..])
    }

    #[inline]
    fn push_le_u24(&mut self, value: u32) {
        self.push(&value.to_le_bytes()[..3])
    }

    #[inline]
    fn push_ne_u24(&mut self, value: u32) {
        if cfg!(target_endian = "big") {
            self.push_be_u24(value)
        } else {
            self.push_le_u24(value)
        }
    }

    #[inline]
    fn push_u32(&mut self, value: u32) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_be_u32(&mut self, value: u32) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_le_u32(&mut self, value: u32) {
        self.push(value.to_le_bytes())
    }

    #[inline]
    fn push_ne_u32(&mut self, value: u32) {
        self.push(value.to_ne_bytes())
    }

    #[inline]
    fn push_u64(&mut self, value: u64) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_be_u64(&mut self, value: u64) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_le_u64(&mut self, value: u64) {
        self.push(value.to_le_bytes())
    }

    #[inline]
    fn push_ne_u64(&mut self, value: u64) {
        self.push(value.to_ne_bytes())
    }

    #[inline]
    fn push_u128(&mut self, value: u128) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_be_u128(&mut self, value: u128) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_le_u128(&mut self, value: u128) {
        self.push(value.to_le_bytes())
    }

    #[inline]
    fn push_ne_u128(&mut self, value: u128) {
        self.push(value.to_ne_bytes())
    }

    #[inline]
    fn push_f32(&mut self, value: f32) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_be_f32(&mut self, value: f32) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_le_f32(&mut self, value: f32) {
        self.push(value.to_le_bytes())
    }

    #[inline]
    fn push_ne_f32(&mut self, value: f32) {
        self.push(value.to_ne_bytes())
    }

    #[inline]
    fn push_f64(&mut self, value: f64) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_be_f64(&mut self, value: f64) {
        self.push(value.to_be_bytes())
    }

    #[inline]
    fn push_le_f64(&mut self, value: f64) {
        self.push(value.to_le_bytes())
    }

    #[inline]
    fn push_ne_f64(&mut self, value: f64) {
        self.push(value.to_ne_bytes())
    }
}