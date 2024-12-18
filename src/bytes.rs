use core::ops::Deref;
use crate::{
    BufRead,
    errors::{JResult, make_error, ErrorKind},
};


#[derive(Debug)]
pub struct Bytes<T> {
    data: T,
    position: usize,
}


impl<T> Bytes<T> {
    #[inline]
    pub fn new(data: T) -> Self {
        Self { data, position: 0 }
    }
}


impl<T> Deref for Bytes<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}


impl<T> BufRead for Bytes<T>
where
    T: AsRef<[u8]>,
{
    #[inline]
    fn get_position(&self) -> usize {
        self.position
    }

    #[inline]
    fn get_position_mut(&mut self) -> &mut usize {
        &mut self.position
    }

    #[inline]
    fn advance(&mut self, nbytes: usize) {
        self.position += nbytes;
    }

    #[inline]
    fn remaining(&self) -> &'_ [u8] {
        self.data.as_ref().get(self.position..).unwrap_or(&[])
    }

    fn take_bytes(&mut self, nbytes: usize) -> JResult<&'_ [u8]> {
        let value = match self.data.as_ref().get(self.position..self.position + nbytes) {
            Some(value) => value,
            None => return Err(make_error(self.remaining(), self.position, ErrorKind::InvalidByteLength)),
        };

        self.position += nbytes;

        Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_take_u8() {
        let mut buffer = Bytes::new(&[0x01, 0x02, 0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x01);
        assert_eq!(buffer.take_u8().unwrap(), 0x02);
        assert_eq!(buffer.remaining(), [0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x03);
        assert_eq!(buffer.remaining(), []);
        assert_eq!(buffer.position, 3);
        assert_eq!(buffer.take_u8().is_err(), true);
    }

    #[test]
    fn test_bytes_take() {
        let mut buffer = Bytes::new([0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.take_bytes(2).unwrap(), &[0x01, 0x02]);
        assert_eq!(buffer.take_bytes(2).unwrap(), &[0x03, 0x04]);
        assert_eq!(buffer.remaining(), &[0x05]);
        assert_eq!(buffer.take_bytes(2).is_err(), true);
        assert_eq!(buffer.take_bytes(1).unwrap(), &[0x05]);
        assert_eq!(buffer.position, 5);
    }
}