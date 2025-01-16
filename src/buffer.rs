#[cfg(feature = "std")]
use std::io::{self, Read, Write, Seek, SeekFrom};
use core::{
    ops::Deref,
    cell::Cell,
};
use crate::std::*;
use crate::{
    BufRead, BufWrite,
    bytes::{Bytes, ToBytes},
    // errors::{JResult, make_error, ErrorKind},
};


/// This is a Buffer type based on the Vec<u8> implementation for storing byte stream data.
/// 
/// # Example
/// 
/// ```
/// use jbytes::prelude::*;
///
///
/// fn buffer_example(buffer: &mut Buffer) -> JResult<()>  {
///     buffer.push_be_u16(1)?;
///     buffer.push(b"\x01\x02\x03")?;
///
///     Ok(())
/// }
///
///
/// fn main() {
///     let mut buffer = Buffer::new();
///     if buffer_example(&mut buffer).is_ok() {
///         assert_eq!(*buffer, b"\x00\x01\x01\x02\x03");
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    data: Vec<u8>,
    position: Cell<usize>,
}


impl Buffer {
    /// Constructs a new Buffer.
    #[inline]
    pub fn new() -> Self {
        Self {
            position: Cell::new(0),
            data: Vec::new(),
        }
    }
}


impl ToBytes for Buffer {
    type Target = [u8];

    fn to_bytes(&self) -> Bytes<&Self::Target> {
        Bytes::new(&(*self.data)[..])
    }
}


impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}


impl From<Vec<u8>> for Buffer {
    fn from(value: Vec<u8>) -> Self {
        Self { data: value, position: Cell::new(0) }
    }
}


impl Deref for Buffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}


impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &(*self.data)[..]
    }
}


impl BufRead for Buffer {
    #[inline]
    fn get_position(&self) -> usize {
        self.position.get()
    }

    #[inline]
    fn get_data(&self) -> &'_ [u8] {
        &self.data
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


impl BufWrite for Buffer {
    #[inline]
    fn remaining_mut(&mut self) -> &'_ mut [u8] {
        &mut self.data[self.position.get()..]
    }

    #[inline]
    fn resize(&mut self, nbytes: usize) -> usize {
        let nbytes = self.position.get() + nbytes;

        self.data.resize(nbytes, 0);

        nbytes
    }
}


#[cfg(feature = "std")]
impl Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let data_len = self.data.len();
        let position = self.position.get();

        if position >= data_len {
            return Ok(0);
        }

        let nbytes = std::cmp::min(buf.len(), data_len - position);
        buf[..nbytes].copy_from_slice(&self.data[position..position + nbytes]);
        self.position.set(position + nbytes);

        Ok(nbytes)
    }
}


#[cfg(feature = "std")]
impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let buf_len = buf.len();
        let position = self.position.get();

        if position + buf_len > self.data.len() {
            self.data.resize(position + buf_len, 0);
        }

        self.data[position..position + buf_len].copy_from_slice(buf);
        self.position.set(position + buf_len);

        Ok(buf_len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}


#[cfg(feature = "std")]
impl Seek for Buffer {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let position = self.position.get();

        let new_position = match pos {
            SeekFrom::Start(offset) => offset as isize,
            SeekFrom::End(offset) => self.data.len() as isize + offset as isize,
            SeekFrom::Current(offset) => position as isize + offset as isize,
        };

        if new_position < 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid seek to a negative position"));
        }

        self.position.set(new_position as usize);

        Ok(position as u64)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_buffer_read_write() {
        let mut buffer = Buffer::from(b"Hello, world!".to_vec());

        // Read Data
        let mut read_buf = [0; 5];
        buffer.read(&mut read_buf).unwrap();
        assert_eq!(&read_buf, b"Hello");
    
        // Seek
        buffer.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(buffer.position.get(), 0);
    
        // Write Data
        buffer.write(b"Rust").unwrap();
        assert_eq!(&buffer.data, b"Rusto, world!");
    
        // Seek
        buffer.seek(SeekFrom::End(-2)).unwrap();
        assert_eq!(buffer.position.get(), buffer.data.len() - 2);
    
        // Write Data
        buffer.write(b"!").unwrap();
        assert_eq!(&buffer.data, b"Rusto, worl!!");
    }

    #[test]
    fn test_buffer_set_position() {
        let mut buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.get_position(), 0);
        buffer.advance(3);
        assert_eq!(buffer.get_position(), 3);
        assert_eq!(buffer.take_u16().unwrap(), 0x0405);
        assert_eq!(buffer.get_position(), 5);
        assert_eq!(buffer.take_u8().is_err(), true);

        buffer.reset_position();
        assert_eq!(buffer.get_position(), 0);
        buffer.set_position(10);
        assert_eq!(buffer.take_u8().is_err(), true);
        buffer.push_u8(0x01).unwrap();
        assert_eq!(buffer.get_position(), 11);
        assert_eq!(buffer.data, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
    }

    #[test]
    fn test_buffer_take_u8() {
        let buffer = Buffer::from(vec![0x01, 0x02, 0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x01);
        assert_eq!(buffer.take_u8().unwrap(), 0x02);
        assert_eq!(buffer.remaining(), [0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x03);
        assert_eq!(buffer.take_u8().is_err(), true);
        assert_eq!(buffer.get_position(), 3);
    }

    #[test]
    fn test_buffer_take_u16() {
        let buffer = Buffer::from(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x04]);
        assert_eq!(buffer.take_u16().unwrap(), 0x0001);
        assert_eq!(buffer.take_be_u16().unwrap(), 0x0002);
        assert_eq!(buffer.take_le_u16().unwrap(), 0x0300);
        assert_eq!(buffer.remaining(), [0x04]);
        assert_eq!(buffer.take_u16().is_err(), true);
        assert_eq!(buffer.take_u8().unwrap(), 0x04);
        assert_eq!(buffer.get_position(), 7);
    }

    #[test]
    fn test_buffer_take_u24() {
        let buffer = Buffer::from(vec![
            0x00, 0x00, 0x01,
            0x00, 0x00, 0x02,
            0x00, 0x00, 0x03,
            0x04
        ]);
        assert_eq!(buffer.take_u24().unwrap(), 0x000001);
        assert_eq!(buffer.take_be_u24().unwrap(), 0x000002);
        assert_eq!(buffer.take_le_u24().unwrap(), 0x030000);
        assert_eq!(buffer.remaining(), [0x04]);
        assert_eq!(buffer.take_u24().is_err(), true);
        assert_eq!(buffer.take_u8().unwrap(), 0x04);
        assert_eq!(buffer.get_position(), 10);
    }

    #[test]
    fn test_buffer_take_u32() {
        let buffer = Buffer::from(vec![
            0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x03,
            0x04
        ]);
        assert_eq!(buffer.take_u32().unwrap(), 0x00000001);
        assert_eq!(buffer.take_be_u32().unwrap(), 0x00000002);
        assert_eq!(buffer.take_le_u32().unwrap(), 0x03000000);
        assert_eq!(buffer.remaining(), [0x04]);
        assert_eq!(buffer.take_u32().is_err(), true);
        assert_eq!(buffer.take_u8().unwrap(), 0x04);
        assert_eq!(buffer.get_position(), 13);
    }

    #[test]
    fn test_buffer_take_u64() {
        let buffer = Buffer::from(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
            0x04
        ]);
        assert_eq!(buffer.take_u64().unwrap(), 0x0000000000000001);
        assert_eq!(buffer.take_be_u64().unwrap(), 0x0000000000000002);
        assert_eq!(buffer.take_le_u64().unwrap(), 0x0300000000000000);
        assert_eq!(buffer.remaining(), [0x04]);
        assert_eq!(buffer.take_u64().is_err(), true);
        assert_eq!(buffer.take_u8().unwrap(), 0x04);
        assert_eq!(buffer.get_position(), buffer.len());
    }

    #[test]
    fn test_buffer_take_u128() {
        let buffer = Buffer::from(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
            0x04
        ]);
        assert_eq!(buffer.take_u128().unwrap(), 0x00000000000000000000000000000001);
        assert_eq!(buffer.take_be_u128().unwrap(), 0x00000000000000000000000000000002);
        assert_eq!(buffer.take_le_u128().unwrap(), 0x03000000000000000000000000000000);
        assert_eq!(buffer.remaining(), [0x04]);
        assert_eq!(buffer.take_u128().is_err(), true);
        assert_eq!(buffer.take_u8().unwrap(), 0x04);
        assert_eq!(buffer.get_position(), buffer.len());
    }

    #[test]
    fn test_buffer_take_uint() {
        let buffer = Buffer::from(vec![
            0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x00, 0x03,
            0x04
        ]);
        assert_eq!(buffer.take_uint(5).unwrap(), 0x0000000001);
        assert_eq!(buffer.take_be_uint(5).unwrap(), 0x0000000002);
        assert_eq!(buffer.take_le_uint(5).unwrap(), 0x0300000000);
        assert_eq!(buffer.remaining(), [0x04]);
        assert_eq!(buffer.take_uint(5).is_err(), true);
        assert_eq!(buffer.take_u8().unwrap(), 0x04);
        assert_eq!(buffer.get_position(), buffer.len());
    }

    #[test]
    fn test_buffer_take_bytes() {
        let buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.take_bytes(2).unwrap(), &[0x01, 0x02]);
        assert_eq!(buffer.take_bytes(2).unwrap(), &[0x03, 0x04]);
        assert_eq!(buffer.remaining(), &[0x05]);
        assert_eq!(buffer.take_bytes(2).is_err(), true);
        assert_eq!(buffer.take_bytes(1).unwrap(), &[0x05]);
        assert_eq!(buffer.position.get(), 5);
    }

    #[test]
    fn test_buffer_take_array() {
        let buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.take_array::<2>().unwrap(), [0x01, 0x02]);
        assert_eq!(buffer.take_array::<2>().unwrap(), [0x03, 0x04]);
        assert_eq!(buffer.remaining(), &[0x05]);
        assert_eq!(buffer.take_array::<2>().is_err(), true);
        assert_eq!(buffer.take_array::<1>().unwrap(), [0x05]);
        assert_eq!(buffer.position.get(), 5);
    }

    #[test]
    fn test_buffer_push() {
        let mut buffer = Buffer::new();
        buffer.push_u8(0x02).unwrap();
        buffer.push_u16(0x03).unwrap();
        buffer.push_u24(0x04).unwrap();
        buffer.push_u32(0x05).unwrap();
        buffer.push_u64(0x06).unwrap();
        buffer.push_u128(0x07).unwrap();
        buffer.push_uint(1, 3).unwrap();
        buffer.push_le_uint(1, 3).unwrap();
        buffer.push_char('1').unwrap();
        buffer.push("23").unwrap();
        buffer.push("45".to_string()).unwrap();
        buffer.push([4, 5]).unwrap();
        buffer.push(vec![6, 7]).unwrap();
        buffer.push(&[8, 9]).unwrap();

        assert_eq!(*buffer, [
            0x02,
            0x00, 0x03,
            0x00, 0x00, 0x04,
            0x00, 0x00, 0x00, 0x05,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07,
            0x00, 0x00, 0x01,
            0x01, 0x00, 0x00,
            0x31,
            0x32, 0x33,
            0x34, 0x35,
            0x04, 0x05,
            0x06, 0x07,
            0x08, 0x09,
        ]);
    }

    #[test]
    fn test_buffer_subsequence() {
        let buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.find_subsequence(&[0x03, 0x04]).unwrap(), &[0x01, 0x02]);
        assert_eq!(buffer.remaining_len(), 1);

        let buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.find_subsequence_needle(&[0x03, 0x04], false).unwrap(), &[0x01, 0x02]);
        assert_eq!(buffer.remaining_len(), 3);

        let buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.find_subsequence_needle(&[0x03, 0x04], true).unwrap(), &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(buffer.remaining_len(), 1);
    }

    #[test]
    fn test_buffer_subsequences() {
        let buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.find_subsequences(&[&[0x03, 0x04], &[0x04, 0x05]]).unwrap(), &[0x01, 0x02]);
        assert_eq!(buffer.remaining_len(), 1);

        let buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.find_subsequences(&[&[0x04, 0x04], &[0x04, 0x05]]).unwrap(), &[0x01, 0x02, 0x03]);
        assert_eq!(buffer.remaining_len(), 0);

        let buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.find_subsequences_needle(&[&[0x03, 0x04], &[0x04, 0x05]], false).unwrap(), &[0x01, 0x02]);
        assert_eq!(buffer.remaining_len(), 3);

        let buffer = Buffer::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.find_subsequences_needle(&[&[0x03, 0x04], &[0x04, 0x05]], true).unwrap(), &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(buffer.remaining_len(), 1);
    }
}