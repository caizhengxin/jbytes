use core::{
    ops::Deref,
    cell::Cell,
};
use crate::BufRead;


#[derive(Debug)]
pub struct Bytes<T> {
    data: T,
    position: Cell<usize>,
}


impl<T> Bytes<T> {
    #[inline]
    pub fn new(data: T) -> Self {
        Self { data, position: Cell::new(0) }
    }

    /// Reset the internal cursor of the `self`.
    #[inline]
    pub fn reset_position(&mut self) {
        self.position = Cell::new(0);
    }

    /// Set the internal cursor of the `self`.
    #[inline]
    pub fn set_position(&mut self, position: usize) {
        self.position = Cell::new(position);
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
        self.position.get()
    }

    #[inline]
    fn get_data(&self) -> &'_ [u8] {
        self.data.as_ref()
    }

    #[inline]
    fn advance(&self, nbytes: usize) {
        self.position.set(self.position.get() + nbytes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_take_u8() {
        let buffer = Bytes::new(&[0x01, 0x02, 0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x01);
        assert_eq!(buffer.take_u8().unwrap(), 0x02);
        assert_eq!(buffer.remaining(), [0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x03);
        assert_eq!(buffer.remaining(), []);
        assert_eq!(buffer.get_position(), 3);
        assert_eq!(buffer.take_u8().is_err(), true);
    }

    #[test]
    fn test_bytes_take() {
        let buffer = Bytes::new([0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.take_bytes(2).unwrap(), &[0x01, 0x02]);
        assert_eq!(buffer.take_bytes(2).unwrap(), &[0x03, 0x04]);
        assert_eq!(buffer.remaining(), &[0x05]);
        assert_eq!(buffer.take_bytes(2).is_err(), true);
        assert_eq!(buffer.take_bytes(1).unwrap(), &[0x05]);
        assert_eq!(buffer.get_position(), 5);
    }
}