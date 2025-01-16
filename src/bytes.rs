use core::{
    ops::Deref,
    cell::Cell,
};
use crate::BufRead;


/// This is a Bytes<T> type for including byte stream data.
/// 
/// # Example
/// 
/// ```
/// use jbytes::prelude::*;
///
///
/// fn main() {
///     let bytes = Bytes::new(b"\x01\x02\x03");
///     assert_eq!(bytes.take_be_u16(), Ok(0x0102));
///     assert_eq!(bytes.take_be_u16().is_err(), true);
/// }
/// ```
#[derive(Debug)]
pub struct Bytes<T> {
    data: T,
    position: Cell<usize>,
}


impl<T> Bytes<T> {
    /// Constructs a new Bytes.
    #[inline]
    pub fn new(data: T) -> Self {
        Self { data, position: Cell::new(0) }
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
    fn set_position(&self, position: usize) {
        self.position.set(position);
    }

    #[inline]
    fn reset_position(&self) {
        self.position.set(0)
    }

    #[inline]
    fn advance(&self, nbytes: usize) {
        self.position.set(self.position.get() + nbytes)
    }
}


pub trait ToBytes {
    type Target: ?Sized;

    /// Returns a Bytes<T> type.
    fn to_bytes(&self) -> Bytes<&Self::Target>;
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
        assert_eq!(buffer.remaining_len(), 0);
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