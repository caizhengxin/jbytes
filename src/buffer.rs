#[cfg(feature = "std")]
use std::io::{self, Read, Write, Seek, SeekFrom};
use crate::{Take, ByteOrder};
use crate::errors::{JResult, make_error, ErrorKind};


#[derive(Debug)]
pub struct Buffer {
    data: Vec<u8>,
    position: usize,
}


impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            position: 0,
        }
    }

    pub fn remain(&self) -> &'_ [u8] {
        &self.data[self.position..]
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position
    }

    pub fn offset(&mut self, offset: isize) {
        self.position = (self.position as isize - offset) as usize;
    }
}


impl Take for Buffer {
    fn take(&mut self, nbyte: usize) -> JResult<&'_ [u8]> {
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

    #[test]
    fn test_buffer_vec() {
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
}