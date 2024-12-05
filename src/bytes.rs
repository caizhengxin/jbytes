use crate::take::Take;
use crate::byteorder::ByteOrder;
use crate::errors::{JResult, make_error, ErrorKind};


#[derive(Debug)]
pub struct Buffer<'a> {
    data: &'a [u8],
    position: usize,
}


impl<'a> Buffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    pub fn remain(&self) -> &'a [u8] {
        &self.data[self.position..]
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position
    }
}


// impl<T: ExactSizeIterator> Read for Buffer<T> {
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         if self.position >= self.data.len() {
//             return Ok(0);
//         }

//         let bytes_to_read = std::cmp::min(buf.len(), self.data.len() - self.position);
//         buf[..bytes_to_read].copy_from_slice(&self.data[self.position..self.position + bytes_to_read]);
//         self.position += bytes_to_read;

//         Ok(bytes_to_read)
//     }
// }


impl<'a> Take for Buffer<'a>
{
    fn take_int(&mut self, byteorder: ByteOrder, nbyte: u8) -> JResult<u128> {
        let input = &self.data[self.position..];
        let input_len = self.data.len();
        let nbyte = nbyte.into();
        let mut value: u128 = 0;
    
        if input_len < nbyte {
            return Err(make_error(self.data, self.position, ErrorKind::InvalidByteLength));
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

    #[inline]
    fn take_be_int(&mut self, nbyte: u8) -> JResult<u128> {
        self.take_int(ByteOrder::Be, nbyte)
    }

    #[inline]
    fn take_le_int(&mut self, nbyte: u8) -> JResult<u128> {
        self.take_int(ByteOrder::Le, nbyte)
    }

    fn take_u8(&mut self) -> JResult<u8> {
        if self.data.is_empty() {
            return Err(make_error(self.data, self.position, ErrorKind::InvalidByteLength));
        }

        let value = self.data[self.position];
        self.position += 1;

        Ok(value)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_int() {
        let mut buffer = Buffer::new(&[0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(0x0102, buffer.take_be_int(2).unwrap());
        assert_eq!(0x0403, buffer.take_le_int(2).unwrap());
        assert_eq!([0x05], buffer.remain());
    }

    #[test]
    fn test_take() {
        let mut buffer = Buffer::new(&[0x01, 0x02, 0x03]);
        let value = buffer.take_u8().unwrap();
        assert_eq!(value, 0x01);
        let value = buffer.take_u8().unwrap();
        assert_eq!(value, 0x02);
        assert_eq!(buffer.position, 2);
    }
}