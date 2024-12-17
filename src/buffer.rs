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
            data,
            position: 0,
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
    fn remain(&self) -> &'_ [u8] {
        &self.data[self.position..]
    }

    #[inline]
    fn advance(&mut self, nbyte: usize) {
        self.position += nbyte;
    }

    fn take_bytes(&mut self, nbyte: usize) -> JResult<&'_ [u8]> {
        let input = &self.data[self.position..];
        let input_len = input.len();
        let nbyte = nbyte.into();

        if input_len < nbyte {
            return Err(make_error(input, self.position, ErrorKind::InvalidByteLength));
        }

        let value = &input[..nbyte];
        self.position += nbyte;

        Ok(value)
    }

    fn take_int(&mut self, byteorder: ByteOrder, nbyte: u8) -> JResult<u128> {
        let input = &self.data[self.position..];
        let input_len = input.len();
        let nbyte = nbyte.into();
        let mut value: u128 = 0;
    
        if input_len < nbyte {
            return Err(make_error(input, self.position, ErrorKind::InvalidByteLength));
        }
    
        match byteorder {
            ByteOrder::Be => {
                for byte in input.iter().take(nbyte) {
                    value = (value << 8) + *byte as u128;
                }
            },
            ByteOrder::Le => {
                for (index, byte) in input.iter().enumerate().take(nbyte) {
                    value += (*byte as u128) << (8 * index);
                }
            }
        }

        self.position += nbyte;
    
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
    fn push<V: AsRef<[u8]>>(&mut self, value: V) {
        self.data.extend_from_slice(value.as_ref());        
    }

    #[inline]
    fn push_char(&mut self, value: char) {
        self.data.push(value as u8);
    }

    #[inline]
    fn push_bytes(&mut self, value: &[u8]) {
        self.data.extend_from_slice(value);
    }

    #[inline]
    fn push_u8(&mut self, value: u8) {
        self.data.push(value);
    }

    #[inline]
    fn push_be_u16(&mut self, value: u16) {
        self.data.extend(value.to_be_bytes());        
    }

    #[inline]
    fn push_le_u16(&mut self, value: u16) {
        self.data.extend(value.to_le_bytes());        
    }

    #[inline]
    fn push_be_u24(&mut self, value: u32) {
        self.data.extend(&value.to_be_bytes()[1..]);        
    }

    #[inline]
    fn push_le_u24(&mut self, value: u32) {
        self.data.extend(&value.to_le_bytes()[..3]);        
    }

    #[inline]
    fn push_be_u32(&mut self, value: u32) {
        self.data.extend(value.to_be_bytes());        
    }

    #[inline]
    fn push_le_u32(&mut self, value: u32) {
        self.data.extend(value.to_le_bytes());        
    }

    #[inline]
    fn push_be_u64(&mut self, value: u64) {
        self.data.extend(value.to_be_bytes());        
    }

    #[inline]
    fn push_le_u64(&mut self, value: u64) {
        self.data.extend(value.to_le_bytes());        
    }

    #[inline]
    fn push_be_u128(&mut self, value: u128) {
        self.data.extend(value.to_be_bytes());        
    }

    #[inline]
    fn push_le_u128(&mut self, value: u128) {
        self.data.extend(value.to_le_bytes());        
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

        let nbyte = std::cmp::min(buf.len(), data_len - self.position);
        buf[..nbyte].copy_from_slice(&self.data[self.position..self.position + nbyte]);
        self.position += nbyte;

        Ok(nbyte)
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
        let mut buffer = Buffer::new(vec![0x01]);
        buffer.push_u8(0x02);
        buffer.push_u16(0x03);
        buffer.push_u24(0x04);
        buffer.push_u32(0x05);
        buffer.push_u64(0x06);
        buffer.push_u128(0x07);
        buffer.push_char('1');
        buffer.push("23");
        buffer.push("45".to_string());
        buffer.push([4, 5]);
        buffer.push(vec![6, 7]);
        buffer.push(&[8, 9]);

        assert_eq!(*buffer, [
            0x01,
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