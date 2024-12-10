use crate::{
    take::Take,
    byteorder::ByteOrder,
    errors::{JResult, make_error, ErrorKind},
};


#[derive(Debug)]
pub struct Bytes<T> {
    data: T,
    position: usize,
}


impl<T> Bytes<T>
where
    T: AsRef<[u8]>,
{
    pub fn new(data: T) -> Self {
        Self { data, position: 0 }
    }

    pub fn remain(&self) -> &'_ [u8] {
        &self.data.as_ref()[self.position..]
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

    // pub fn push_u8(&mut self, value: u8) {
    //     let ss = self.data.push(value);
    // }
}


impl<T> Take for Bytes<T>
where
    T: AsRef<[u8]>,
{
    fn take(&mut self, nbyte: usize) -> JResult<&'_ [u8]> {
        let data = self.data.as_ref();
        let input = &data[self.position..];
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
        let data = self.data.as_ref();
        let input = &data[self.position..];
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
        let data = self.data.as_ref();
        let input = &data[self.position..];

        if input.is_empty() {
            return Err(make_error(input, self.position, ErrorKind::InvalidByteLength));
        }

        let value = data[self.position];
        self.position += 1;

        Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_int() {
        let value = [0x01, 0x02, 0x03, 0x04, 0x05];
        let mut buffer = Bytes::new(&value);
        assert_eq!(buffer.take_be_int(2).unwrap(), 0x0102);
        assert_eq!(buffer.take_le_int(2).unwrap(), 0x0403);
        assert_eq!(buffer.remain(), [0x05]);
        assert_eq!(buffer.take_le_int(2), Err(make_error(&value[4..], 4, ErrorKind::InvalidByteLength)));
        assert_eq!(buffer.take_le_int(1).unwrap(), 0x05);
        assert_eq!(buffer.position, 5);
    }

    #[test]
    fn test_take_u8() {
        let mut buffer = Bytes::new(&[0x01, 0x02, 0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x01);
        assert_eq!(buffer.take_u8().unwrap(), 0x02);
        assert_eq!(buffer.remain(), [0x03]);
        assert_eq!(buffer.take_u8().unwrap(), 0x03);
        assert_eq!(buffer.remain(), []);
        assert_eq!(buffer.position, 3);
        assert_eq!(buffer.take_u8().is_err(), true);
    }

    #[test]
    fn test_take_string_type() {
        let mut buffer = Bytes::new("abcde");
        assert_eq!(buffer.take_be_int(2).unwrap(), 0x6162);
        assert_eq!(buffer.take_le_int(2).unwrap(), 0x6463);
        assert_eq!(buffer.remain(), &[0x65]);
        assert_eq!(buffer.take_le_int(2).is_err(), true);
        assert_eq!(buffer.take_le_int(1).unwrap(), 0x65);
        assert_eq!(buffer.position, 5);
    }

    #[test]
    fn test_take() {
        let mut buffer = Bytes::new([0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(buffer.take(2).unwrap(), &[0x01, 0x02]);
        assert_eq!(buffer.take(2).unwrap(), &[0x03, 0x04]);
        assert_eq!(buffer.remain(), &[0x05]);
        assert_eq!(buffer.take(2).is_err(), true);
        assert_eq!(buffer.take(1).unwrap(), &[0x05]);
        assert_eq!(buffer.position, 5);
    }
}