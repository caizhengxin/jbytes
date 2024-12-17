#[cfg(feature = "std")]
use std::io::{self, Read, Write, Seek, SeekFrom};
use core::ops::Deref;
use crate::{
    ByteOrder, BufRead, BufWrite,
    errors::{JResult, make_error, ErrorKind},
};


#[derive(Debug)]
pub struct Buffer {
    data: Vec<u8>,
    position: usize,
}


impl Buffer {
    #[inline]
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            position: 0,
            data,
        }
    }

    #[inline]
    pub fn reset_position(&mut self) {
        self.position = 0;
    }
}


impl Deref for Buffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}


impl BufRead for Buffer {
    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn remain(&self) -> &'_ [u8] {
        &self.data[self.position..]
    }

    #[inline]
    fn remaining_data(&self) -> &'_ [u8] {
        &self.data[self.position..]
    }

    #[inline]
    fn remaining_len(&self) -> usize {
        self.data.len() - self.position
    }

    #[inline]
    fn current_position(&self) -> usize {
        self.position
    }

    #[inline]
    fn set_position(&mut self, position: usize) -> JResult<()> {
        if position > self.data.len() {
            return Err(make_error(self.remaining_data(), self.current_position(), ErrorKind::InvalidPosition(position)));
        }

        self.position = position;
        Ok(())
    }

    #[inline]
    fn advance(&mut self, nbytes: usize) {
        self.position += nbytes;
    }

    fn take_bytes(&mut self, nbytes: usize) -> JResult<&'_ [u8]> {
        let value = match self.data.get(self.position..self.position + nbytes) {
            Some(value) => value,
            None => return Err(make_error(self.remaining_data(), self.position, ErrorKind::InvalidByteLength)),
        };

        self.position += nbytes;

        Ok(value)
    }

    fn take_int(&mut self, byteorder: ByteOrder, nbytes: u8) -> JResult<u128> {
        let input = &self.data[self.position..];
        let input_len = input.len();
        let nbytes = nbytes.into();
        let mut value: u128 = 0;
    
        if input_len < nbytes {
            return Err(make_error(input, self.position, ErrorKind::InvalidByteLength));
        }
    
        match byteorder {
            ByteOrder::Be => {
                for byte in input.iter().take(nbytes) {
                    value = (value << 8) + *byte as u128;
                }
            },
            ByteOrder::Le => {
                for (index, byte) in input.iter().enumerate().take(nbytes) {
                    value += (*byte as u128) << (8 * index);
                }
            }
        }

        self.position += nbytes;
    
        Ok(value)
    }

    fn take_u8(&mut self) -> JResult<u8> {
        let input = &self.data[self.position..];

        if input.is_empty() {
            return Err(make_error(input, self.position, ErrorKind::InvalidByteLength));
        }

        let value = self.data[self.position];
        self.position += 1;

        Ok(value)
    }
}


impl BufWrite for Buffer {
    #[inline]
    fn push<V: AsRef<[u8]>>(&mut self, value: V) -> JResult<usize> {
        let data = value.as_ref();
        let data_len = data.len();

        if data_len > self.remaining_len() {
            self.data.resize(self.position + data_len, 0);
        }

        self.data[self.position..self.position + data_len].clone_from_slice(data);
        self.position += data_len;

        Ok(data_len)
    }
}


#[cfg(feature = "std")]
impl Read for Buffer
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let data_len = self.data.len();

        if self.position >= data_len {
            return Ok(0);
        }

        let nbytes = std::cmp::min(buf.len(), data_len - self.position);
        buf[..nbytes].copy_from_slice(&self.data[self.position..self.position + nbytes]);
        self.position += nbytes;

        Ok(nbytes)
    }
}


#[cfg(feature = "std")]
impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let buf_len = buf.len();

        if self.position + buf_len > self.data.len() {
            self.data.resize(self.position + buf_len, 0);
        }

        self.data[self.position..self.position + buf_len].copy_from_slice(buf);
        self.position += buf_len;

        Ok(buf_len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}


#[cfg(feature = "std")]
impl Seek for Buffer {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let new_position = match pos {
            SeekFrom::Start(offset) => offset as isize,
            SeekFrom::End(offset) => self.data.len() as isize + offset as isize,
            SeekFrom::Current(offset) => self.position as isize + offset as isize,
        };

        if new_position < 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid seek to a negative position"));
        }

        self.position = new_position as usize;

        Ok(self.position as u64)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_buffer_read_write() {
        let mut buffer = Buffer::new(b"Hello, world!".to_vec());

        // Read Data
        let mut read_buf = [0; 5];
        buffer.read(&mut read_buf).unwrap();
        assert_eq!(&read_buf, b"Hello");
    
        // Seek
        buffer.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(buffer.position, 0);
    
        // Write Data
        buffer.write(b"Rust").unwrap();
        assert_eq!(&buffer.data, b"Rusto, world!");
    
        // Seek
        buffer.seek(SeekFrom::End(-2)).unwrap();
        assert_eq!(buffer.position, buffer.data.len() - 2);
    
        // Write Data
        buffer.write(b"!").unwrap();
        assert_eq!(&buffer.data, b"Rusto, worl!!");
    }

    #[test]
    fn test_take_int() {
        let value = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let mut buffer = Buffer::new(value.clone());
        assert_eq!(buffer.take_be_int(2).unwrap(), 0x0102);
        assert_eq!(buffer.take_le_int(2).unwrap(), 0x0403);
        assert_eq!(buffer.remain(), [0x05]);
        assert_eq!(buffer.take_le_int(2), Err(make_error(&value[4..], 4, ErrorKind::InvalidByteLength)));
        assert_eq!(buffer.take_le_int(1).unwrap(), 0x05);
        assert_eq!(buffer.position, 5);
    }

    #[test]
    fn test_take_u8() {
        let mut buffer = Buffer::new(vec![0x01, 0x02, 0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x01);
        assert_eq!(buffer.take_u8().unwrap(), 0x02);
        assert_eq!(buffer.remain(), [0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x03);
        assert_eq!(buffer.remain(), []);
        assert_eq!(buffer.position, 3);
        assert_eq!(buffer.take_u8().is_err(), true);
    }

    #[test]
    fn test_take_int_with_string_type() {
        let mut buffer = Buffer::new("abcde".as_bytes().to_vec());
        assert_eq!(buffer.take_be_int(2).unwrap(), 0x6162);
        assert_eq!(buffer.take_le_int(2).unwrap(), 0x6463);
        assert_eq!(buffer.remain(), &[0x65]);
        assert_eq!(buffer.take_le_int(2).is_err(), true);
        assert_eq!(buffer.take_le_int(1).unwrap(), 0x65);
        assert_eq!(buffer.position, 5);
    }

    #[test]
    fn test_take_bytes() {
        let mut buffer = Buffer::new(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.take_bytes(2).unwrap(), &[0x01, 0x02]);
        assert_eq!(buffer.take_bytes(2).unwrap(), &[0x03, 0x04]);
        assert_eq!(buffer.remain(), &[0x05]);
        assert_eq!(buffer.take_bytes(2).is_err(), true);
        assert_eq!(buffer.take_bytes(1).unwrap(), &[0x05]);
        assert_eq!(buffer.position, 5);
    }

    #[test]
    fn test_push() {
        let mut buffer = Buffer::new(vec![]);
        buffer.push_u8(0x02).unwrap();
        buffer.push_u16(0x03).unwrap();
        buffer.push_u24(0x04).unwrap();
        buffer.push_u32(0x05).unwrap();
        buffer.push_u64(0x06).unwrap();
        buffer.push_u128(0x07).unwrap();
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
            0x31,
            0x32, 0x33,
            0x34, 0x35,
            0x04, 0x05,
            0x06, 0x07,
            0x08, 0x09,
        ]);
    }
}