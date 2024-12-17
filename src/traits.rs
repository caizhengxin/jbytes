use core::mem;
use crate::{
    ByteOrder,
    errors::{JResult, make_error, ErrorKind},
};


pub trait BufRead {
    fn len(&self) -> usize;

    fn remain(&self) -> &'_ [u8];

    fn remaining_data(&self) -> &'_ [u8];

    #[inline]
    fn remaining_len(&self) -> usize {
        self.remaining_data().len()
    }

    fn current_position(&self) -> usize;

    fn set_position(&mut self, position: usize) -> JResult<()>;

    fn advance(&mut self, nbytes: usize);

    fn take_bytes(&mut self, nbytes: usize) -> JResult<&'_ [u8]>;

    fn take_int(&mut self, byteorder: ByteOrder, nbytes: u8) -> JResult<u128>;

    #[inline]
    fn take_be_int(&mut self, nbytes: u8) -> JResult<u128> {
        self.take_int(ByteOrder::Be, nbytes)
    }

    #[inline]
    fn take_le_int(&mut self, nbytes: u8) -> JResult<u128> {
        self.take_int(ByteOrder::Le, nbytes)
    }

    #[inline]
    fn take_u8(&mut self) -> JResult<u8> {
        let value = self.take_be_int(1)?;
        Ok(value as u8)
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


pub trait BufWrite: BufRead {
    /// Writes `AsRef<[u8]>` to `self`, eg: &[u8]/&str/String/array/vec, etc.
    fn push<V: AsRef<[u8]>>(&mut self, value: V) -> JResult<usize>;

    /// Writes bytes(&[u8]) to `self`.
    #[inline]
    fn push_bytes(&mut self, value: &[u8]) -> JResult<usize> {
        self.push(value)
    }

    /// Writes a char to `self`.
    #[inline]
    fn push_char(&mut self, value: char) -> JResult<usize> {
        self.push([value as u8])
    }

    /// Writes an unsigned 8 bit integer to `self`.
    #[inline]
    fn push_u8(&mut self, value: u8) -> JResult<usize> {
        self.push([value])
    }

    /// Writes an unsigned 8 bit integer to `self`, exactly like the `[push_u8]` function.
    #[inline]
    fn push_be_u8(&mut self, value: u8) -> JResult<usize> {
        self.push_u8(value)
    }

    /// Writes an unsigned 8 bit integer to `self`, exactly like the `[push_u8]` function.
    #[inline]
    fn push_le_u8(&mut self, value: u8) -> JResult<usize> {
        self.push_u8(value)
    }

    /// Writes an unsigned 8 bit integer to `self`, exactly like the `[push_u8]` function.
    #[inline]
    fn push_ne_u8(&mut self, value: u8) -> JResult<usize> {
        self.push_u8(value)
    }

    /// Writes a signed 8 bit integer to `self`.
    #[inline]
    fn push_i8(&mut self, value: i8) -> JResult<usize> {
        self.push([value as u8])
    }

    /// Writes a signed 8 bit integer to `self`, exactly like the `[push_i8]` function.
    #[inline]
    fn push_be_i8(&mut self, value: i8) -> JResult<usize> {
        self.push_i8(value)
    }

    /// Writes a signed 8 bit integer to `self`, exactly like the `[push_i8]` function.
    #[inline]
    fn push_le_i8(&mut self, value: i8) -> JResult<usize> {
        self.push_i8(value)
    }

    /// Writes a signed 8 bit integer to `self`, exactly like the `[push_i8]` function.
    #[inline]
    fn push_ne_i8(&mut self, value: i8) -> JResult<usize> {
        self.push_i8(value)
    }

    /// Writes an unsigned 16 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_u16(&mut self, value: u16) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned 16 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_u16(&mut self, value: u16) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned 16 bit integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_u16(&mut self, value: u16) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes an unsigned 16 bit integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_u16(&mut self, value: u16) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes a signed 16 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_i16(&mut self, value: i16) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed 16 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_i16(&mut self, value: i16) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed 16 bit integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_i16(&mut self, value: i16) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes a signed 16 bit integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_i16(&mut self, value: i16) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes an unsigned 24 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_u24(&mut self, value: u32) -> JResult<usize> {
        self.push(&value.to_be_bytes()[1..])
    }

    /// Writes an unsigned 24 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_u24(&mut self, value: u32) -> JResult<usize> {
        self.push(&value.to_be_bytes()[1..])
    }

    /// Writes an unsigned 24 bit integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_u24(&mut self, value: u32) -> JResult<usize> {
        self.push(&value.to_le_bytes()[..3])
    }

    /// Writes an unsigned 24 bit integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_u24(&mut self, value: u32) -> JResult<usize> {
        if cfg!(target_endian = "big") {
            self.push_be_u24(value)
        } else {
            self.push_le_u24(value)
        }
    }

    /// Writes an unsigned 32 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_u32(&mut self, value: u32) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned 32 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_u32(&mut self, value: u32) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned 32 bit integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_u32(&mut self, value: u32) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes an unsigned 32 bit integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_u32(&mut self, value: u32) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes a signed 32 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_i32(&mut self, value: i32) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed 32 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_i32(&mut self, value: i32) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed 32 bit integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_i32(&mut self, value: i32) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes a signed 32 bit integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_i32(&mut self, value: i32) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes an unsigned 64 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_u64(&mut self, value: u64) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned 64 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_u64(&mut self, value: u64) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned 64 bit integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_u64(&mut self, value: u64) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes an unsigned 64 bit integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_u64(&mut self, value: u64) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes a signed 64 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_i64(&mut self, value: i64) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed 64 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_i64(&mut self, value: i64) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed 64 bit integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_i64(&mut self, value: i64) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes a signed 64 bit integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_i64(&mut self, value: i64) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes an unsigned 128 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_u128(&mut self, value: u128) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned 128 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_u128(&mut self, value: u128) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned 128 bit integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_u128(&mut self, value: u128) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes an unsigned 128 bit integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_u128(&mut self, value: u128) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes a signed 128 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_i128(&mut self, value: i128) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed 128 bit integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_i128(&mut self, value: i128) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed 128 bit integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_i128(&mut self, value: i128) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes a signed 128 bit integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_i128(&mut self, value: i128) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes an unsigned size integer to `self` in big-endian byte order.
    #[inline]
    fn push_usize(&mut self, value: usize) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned usize integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_usize(&mut self, value: usize) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an unsigned usize integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_usize(&mut self, value: usize) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes an unsigned usize integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_usize(&mut self, value: usize) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes a signed size integer to `self` in big-endian byte order.
    #[inline]
    fn push_isize(&mut self, value: isize) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed usize integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_isize(&mut self, value: isize) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes a signed usize integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_isize(&mut self, value: isize) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes a signed usize integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_isize(&mut self, value: isize) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes an unsigned n-byte integer to `self` in big-endian byte order.
    #[inline]
    fn push_uint(&mut self, value: u64, nbytes: usize) -> JResult<usize> {
        self.push_be_uint(value, nbytes)
    }

    /// Writes an unsigned n-byte integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_uint(&mut self, value: u64, nbytes: usize) -> JResult<usize> {
        let start = match mem::size_of_val(&value).checked_sub(nbytes) {
            Some(start) => start,
            None => return Err(make_error(self.remain(), self.current_position(), ErrorKind::InvalidByteLength)),
        };

        self.push(&value.to_be_bytes()[start..])
    }

    /// Writes an unsigned n-byte integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_uint(&mut self, value: u64, nbytes: usize) -> JResult<usize> {
        let slice = value.to_le_bytes();
        let slice = match slice.get(..nbytes) {
            Some(slice) => slice,
            None => return Err(make_error(self.remain(), self.current_position(), ErrorKind::InvalidByteLength)),
        };

        self.push(slice)
    }

    /// Writes an unsigned n-byte integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_uint(&mut self, value: u64, nbytes: usize) -> JResult<usize> {
        if cfg!(target_endian = "big") {
            self.push_be_uint(value, nbytes)
        } else {
            self.push_be_uint(value, nbytes)
        }
    }

    /// Writes a signed n-byte integer to `self` in big-endian byte order.
    #[inline]
    fn push_int(&mut self, value: i64, nbytes: usize) -> JResult<usize> {
        self.push_uint(value as u64, nbytes)
    }

    /// Writes a signed n-byte integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_int(&mut self, value: i64, nbytes: usize) -> JResult<usize> {
        self.push_be_uint(value as u64, nbytes)
    }

    /// Writes a signed n-byte integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_int(&mut self, value: i64, nbytes: usize) -> JResult<usize> {
        self.push_le_uint(value as u64, nbytes)
    }

    /// Writes a signed n-byte integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_int(&mut self, value: i64, nbytes: usize) -> JResult<usize> {
        if cfg!(target_endian = "big") {
            self.push_be_int(value, nbytes)
        } else {
            self.push_be_int(value, nbytes)
        }
    }

    /// Writes an IEEE754 signle-precision (4 bytes) floating point number to `self` in big-endian byte order.
    #[inline]
    fn push_f32(&mut self, value: f32) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an IEEE754 signle-precision (4 bytes) floating point number to `self` in big-endian byte order.
    #[inline]
    fn push_be_f32(&mut self, value: f32) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an IEEE754 signle-precision (4 bytes) floating point number to `self` in little-endian byte order.
    #[inline]
    fn push_le_f32(&mut self, value: f32) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes an IEEE754 signle-precision (4 bytes) floating point number to `self` in native-endian byte order.
    #[inline]
    fn push_ne_f32(&mut self, value: f32) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }

    /// Writes an IEEE754 double-precision (8 bytes) floating point number to `self` in big-endian byte order.
    #[inline]
    fn push_f64(&mut self, value: f64) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an IEEE754 double-precision (8 bytes) floating point number to `self` in big-endian byte order.
    #[inline]
    fn push_be_f64(&mut self, value: f64) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an IEEE754 double-precision (8 bytes) floating point number to `self` in little-endian byte order.
    #[inline]
    fn push_le_f64(&mut self, value: f64) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes an IEEE754 double-precision (8 bytes) floating point number to `self` in native-endian byte order.
    #[inline]
    fn push_ne_f64(&mut self, value: f64) -> JResult<usize> {
        self.push(value.to_ne_bytes())
    }
}