use crate::{
    BufRead,
    errors::{JResult, make_error, ErrorKind},
};


impl BufRead for &'_ [u8] {
    #[inline]
    fn get_position(&self) -> usize {
        0
    }

    #[inline]
    fn get_data(&self) -> &'_ [u8] {
        self
    }

    #[inline]
    fn advance(&mut self, nbytes: usize) {
        *self = &self[nbytes..]
    }

    #[inline]
    fn remaining(&self) -> &'_ [u8] {
        self
    }

    #[inline]
    fn take_bytes(&mut self, nbytes: usize) -> JResult<&'_ [u8]> {
        let value = match self.get(..nbytes) {
            Some(value) => value,
            None => return Err(make_error(self.get_position(), ErrorKind::InvalidByteLength)),
        };

        self.advance(nbytes);

        Ok(value)
    }
}


#[cfg(feature = "std")]
impl<T: AsRef<[u8]>> BufRead for std::io::Cursor<T> {
    #[inline]
    fn get_position(&self) -> usize {
        self.position() as usize
    }

    #[inline]
    fn get_data(&self) -> &'_ [u8] {
        self.get_ref().as_ref()
    }

    #[inline]
    fn advance(&mut self, nbytes: usize) {
        self.set_position(self.position() + nbytes as u64);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_ref() {
        let buffer = [0x01_u8, 0x02, 0x03, 0x04];
        let mut buffer_ref = buffer.as_ref();
        assert_eq!(buffer_ref.take_u16().unwrap(), 0x0102);
        assert_eq!(buffer, [0x01, 0x02, 0x03, 0x04]);
        assert_eq!(buffer_ref, [0x03, 0x04]);
        assert_eq!(buffer_ref.take_u16().unwrap(), 0x0304);
        assert_eq!(buffer_ref.take_u16().is_err(), true);
    }
}