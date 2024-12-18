use core::mem;
use crate::{
    // ByteOrder,
    errors::{JResult, make_error, ErrorKind},
};


macro_rules! macro_take_bytes {
    ($this:ident, $typ:tt::$func:tt) => {{
        const SIZE: usize = core::mem::size_of::<$typ>();

        return Ok($typ::$func(unsafe { *($this.take_bytes(SIZE)? as *const _ as *const [_; SIZE]) }));
    }};
    ($this:ident, $typ:tt::from_be_bytes, $nbytes:expr) => {{
        const SIZE: usize = core::mem::size_of::<$typ>();

        let mut buf = [0; SIZE];
        let slice_at = match SIZE.checked_sub($nbytes) {
            Some(slice_at) => slice_at,
            None => return Err(make_error($this.remaining(), $this.get_position(), ErrorKind::InvalidByteLength)),
        };
        buf[slice_at..].copy_from_slice($this.take_bytes($nbytes)?);

        return Ok($typ::from_be_bytes(buf));
    }};
    ($this:ident, $typ:tt::from_le_bytes, $nbytes:expr) => {{
        const SIZE: usize = core::mem::size_of::<$typ>();

        let mut buf = [0; SIZE];

        buf[..$nbytes].copy_from_slice($this.take_bytes($nbytes)?);

        return Ok($typ::from_le_bytes(buf));
    }};
}


pub trait BufRead {
    /// Get the internal cursor of the `self`.
    fn get_position_mut(&mut self) -> &mut usize;

    /// Get the internal cursor of the `self`.
    fn get_position(&self) -> usize;

    /// Reset the internal cursor of the `self`.
    #[inline]
    fn reset_position(&mut self) {
        *self.get_position_mut() = 0;
    }

    /// Set the internal cursor of the `self`.
    #[inline]
    fn set_position(&mut self, position: usize) {
        *self.get_position_mut() = position;
    }

    /// Advance the internal cursor of the `self`.
    fn advance(&mut self, nbytes: usize);

    /// Returns the n-bytes between the current position and the end of the buffer.
    fn remaining(&self) -> &'_ [u8];

    /// Returns the number of bytes between the current position and the end of the buffer.
    #[inline]
    fn remaining_len(&self) -> usize {
        self.remaining().len()
    }

    /// Reads n-byte data from `self`.
    fn take_bytes(&mut self, nbytes: usize) -> JResult<&'_ [u8]>;

    /// Reads an unsigned 8 bit integer from `self`.
    #[inline]
    fn take_u8(&mut self) -> JResult<u8> {
        Ok(self.take_bytes(1)?[0])
    }

    /// Reads an unsigned 8 bit integer from `self`, exactly like the `[take_u8]` function.
    #[inline]
    fn take_be_u8(&mut self) -> JResult<u8> {
        self.take_u8()
    }

    /// Reads an unsigned 8 bit integer from `self`, exactly like the `[take_u8]` function.
    #[inline]
    fn take_le_u8(&mut self) -> JResult<u8> {
        self.take_u8()
    }

    /// Reads an unsigned 8 bit integer from `self`, exactly like the `[take_u8]` function.
    #[inline]
    fn take_ne_u8(&mut self) -> JResult<u8> {
        self.take_u8()
    }

    /// Reads a signed 8 bit integer from `self`.
    #[inline]
    fn take_i8(&mut self) -> JResult<i8> {
        Ok(self.take_bytes(1)?[0] as i8)
    }

    /// Reads a signed 8 bit integer from `self`, exactly like the `[take_i8]` function.
    #[inline]
    fn take_be_i8(&mut self) -> JResult<i8> {
        self.take_i8()
    }

    /// Reads a signed 8 bit integer from `self`, exactly like the `[take_i8]` function.
    #[inline]
    fn take_le_i8(&mut self) -> JResult<i8> {
        self.take_i8()
    }

    /// Reads a signed 8 bit integer from `self`, exactly like the `[take_i8]` function.
    #[inline]
    fn take_ne_i8(&mut self) -> JResult<i8> {
        self.take_i8()
    }

    /// Reads an unsigned 16 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u16(&mut self) -> JResult<u16> {
        macro_take_bytes!(self, u16::from_be_bytes);
    }

    /// Reads an unsigned 16 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u16(&mut self) -> JResult<u16> {
        macro_take_bytes!(self, u16::from_be_bytes);
    }

    /// Reads an unsigned 16 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u16(&mut self) -> JResult<u16> {
        macro_take_bytes!(self, u16::from_le_bytes);
    }

    /// Reads an unsigned 16 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u16(&mut self) -> JResult<u16> {
        macro_take_bytes!(self, u16::from_ne_bytes);
    }

    /// Reads a signed 16 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_i16(&mut self) -> JResult<i16> {
        macro_take_bytes!(self, i16::from_be_bytes);
    }

    /// Reads a signed 16 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_i16(&mut self) -> JResult<i16> {
        macro_take_bytes!(self, i16::from_be_bytes);
    }

    /// Reads a signed 16 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_i16(&mut self) -> JResult<i16> {
        macro_take_bytes!(self, i16::from_le_bytes);
    }

    /// Reads a signed 16 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_i16(&mut self) -> JResult<i16> {
        macro_take_bytes!(self, i16::from_ne_bytes);
    }

    /// Reads an unsigned 24 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u24(&mut self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_be_bytes, 3);
    }

    /// Reads an unsigned 24 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u24(&mut self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_be_bytes, 3);
    }

    /// Reads an unsigned 24 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u24(&mut self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_le_bytes, 3);
    }

    /// Reads an unsigned 24 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u24(&mut self) -> JResult<u32> {
        if cfg!(target_endian = "big") {
            self.take_be_u24()
        } else {
            self.take_le_u24()
        }
    }

    /// Reads an unsigned 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u32(&mut self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_be_bytes);
    }

    /// Reads an unsigned 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u32(&mut self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_be_bytes);
    }

    /// Reads an unsigned 32 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u32(&mut self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_le_bytes);
    }

    /// Reads an unsigned 32 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u32(&mut self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_ne_bytes);
    }

    /// Reads a signed 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_i32(&mut self) -> JResult<i32> {
        macro_take_bytes!(self, i32::from_be_bytes);
    }

    /// Reads a signed 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_i32(&mut self) -> JResult<i32> {
        macro_take_bytes!(self, i32::from_be_bytes);
    }

    /// Reads a signed 32 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_i32(&mut self) -> JResult<i32> {
        macro_take_bytes!(self, i32::from_le_bytes);
    }

    /// Reads a signed 32 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_i32(&mut self) -> JResult<i32> {
        macro_take_bytes!(self, i32::from_ne_bytes);
    }

    /// Reads an unsigned 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u64(&mut self) -> JResult<u64> {
        macro_take_bytes!(self, u64::from_be_bytes);
    }

    /// Reads an unsigned 64 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u64(&mut self) -> JResult<u64> {
        macro_take_bytes!(self, u64::from_be_bytes);
    }

    /// Reads an unsigned 64 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u64(&mut self) -> JResult<u64> {
        macro_take_bytes!(self, u64::from_le_bytes);
    }

    /// Reads an unsigned 64 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u64(&mut self) -> JResult<u64> {
        macro_take_bytes!(self, u64::from_ne_bytes);
    }

    /// Reads a signed 64 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_i64(&mut self) -> JResult<i64> {
        macro_take_bytes!(self, i64::from_be_bytes);
    }

    /// Reads a signed 64 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_i64(&mut self) -> JResult<i64> {
        macro_take_bytes!(self, i64::from_be_bytes);
    }

    /// Reads a signed 64 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_i64(&mut self) -> JResult<i64> {
        macro_take_bytes!(self, i64::from_le_bytes);
    }

    /// Reads a signed 64 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_i64(&mut self) -> JResult<i64> {
        macro_take_bytes!(self, i64::from_ne_bytes);
    }

    /// Reads an unsigned 128 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u128(&mut self) -> JResult<u128> {
        macro_take_bytes!(self, u128::from_be_bytes);
    }

    /// Reads an unsigned 128 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u128(&mut self) -> JResult<u128> {
        macro_take_bytes!(self, u128::from_be_bytes);
    }

    /// Reads an unsigned 128 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u128(&mut self) -> JResult<u128> {
        macro_take_bytes!(self, u128::from_le_bytes);
    }

    /// Reads an unsigned 128 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u128(&mut self) -> JResult<u128> {
        macro_take_bytes!(self, u128::from_ne_bytes);
    }

    /// Reads a signed 128 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_i128(&mut self) -> JResult<i128> {
        macro_take_bytes!(self, i128::from_be_bytes);
    }

    /// Reads a signed 128 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_i128(&mut self) -> JResult<i128> {
        macro_take_bytes!(self, i128::from_be_bytes);
    }

    /// Reads a signed 128 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_i128(&mut self) -> JResult<i128> {
        macro_take_bytes!(self, i128::from_le_bytes);
    }

    /// Reads a signed 128 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_i128(&mut self) -> JResult<i128> {
        macro_take_bytes!(self, i128::from_ne_bytes);
    }

    /// Reads an unsigned size integer from `self` in big-endian byte order.
    #[inline]
    fn take_usize(&mut self) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_be_bytes);
    }

    /// Reads an unsigned size integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_usize(&mut self) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_be_bytes);
    }

    /// Reads an unsigned size integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_usize(&mut self) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_le_bytes);
    }

    /// Reads an unsigned size integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_usize(&mut self) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_ne_bytes);
    }

    /// Reads a signed size integer from `self` in big-endian byte order.
    #[inline]
    fn take_isize(&mut self) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_be_bytes);
    }

    /// Reads a signed size integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_isize(&mut self) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_be_bytes);
    }

    /// Reads a signed size integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_isize(&mut self) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_le_bytes);
    }

    /// Reads a signed size integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_isize(&mut self) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_ne_bytes);
    }

    /// Reads an unsigned n-byte integer from `self` in big-endian byte order.
    #[inline]
    fn take_uint(&mut self, nbytes: usize) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_be_bytes, nbytes);
    }

    /// Reads an unsigned n-byte integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_uint(&mut self, nbytes: usize) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_be_bytes, nbytes);
    }

    /// Reads an unsigned n-byte integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_uint(&mut self, nbytes: usize) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_le_bytes, nbytes);
    }

    /// Reads an unsigned n-byte integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_uint(&mut self, nbytes: usize) -> JResult<usize> {
        if cfg!(target_endian = "big") {
            self.take_be_uint(nbytes)
        } else {
            self.take_le_uint(nbytes)
        }
    }

    /// Reads a signed n-byte integer from `self` in big-endian byte order.
    #[inline]
    fn take_int(&mut self, nbytes: usize) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_be_bytes, nbytes);
    }

    /// Reads a signed n-byte integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_int(&mut self, nbytes: usize) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_be_bytes, nbytes);
    }

    /// Reads a signed n-byte integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_int(&mut self, nbytes: usize) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_le_bytes, nbytes);
    }

    /// Reads a signed n-byte integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_int(&mut self, nbytes: usize) -> JResult<isize> {
        if cfg!(target_endian = "big") {
            self.take_be_int(nbytes)
        } else {
            self.take_le_int(nbytes)
        }
    }

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self` in big-endian byte order.
    #[inline]
    fn take_f32(&mut self) -> JResult<f32> {
        macro_take_bytes!(self, f32::from_be_bytes);
    }

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self` in big-endian byte order.
    #[inline]
    fn take_be_f32(&mut self) -> JResult<f32> {
        macro_take_bytes!(self, f32::from_be_bytes);
    }

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self` in little-endian byte order.
    #[inline]
    fn take_le_f32(&mut self) -> JResult<f32> {
        macro_take_bytes!(self, f32::from_le_bytes);
    }

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self` in native-endian byte order.
    #[inline]
    fn take_ne_f32(&mut self) -> JResult<f32> {
        macro_take_bytes!(self, f32::from_ne_bytes);
    }

    /// Reads an IEEE754 double-precision (8 bytes) floating point number from `self` in big-endian byte order.
    #[inline]
    fn take_f64(&mut self) -> JResult<f64> {
        macro_take_bytes!(self, f64::from_be_bytes);
    }

    /// Reads an IEEE754 double-precision (8 bytes) floating point number from `self` in big-endian byte order.
    #[inline]
    fn take_be_f64(&mut self) -> JResult<f64> {
        macro_take_bytes!(self, f64::from_be_bytes);
    }

    /// Reads an IEEE754 double-precision (8 bytes) floating point number from `self` in little-endian byte order.
    #[inline]
    fn take_le_f64(&mut self) -> JResult<f64> {
        macro_take_bytes!(self, f64::from_le_bytes);
    }

    /// Reads an IEEE754 double-precision (8 bytes) floating point number from `self` in native-endian byte order.
    #[inline]
    fn take_ne_f64(&mut self) -> JResult<f64> {
        macro_take_bytes!(self, f64::from_ne_bytes);
    }
}


pub trait BufWrite: BufRead {
    /// Returns the n-bytes between the current position and the end of the buffer.
    fn remaining_mut(&mut self) -> &'_ mut [u8];

    fn resize(&mut self, nbytes: usize) -> usize;

    /// Writes `AsRef<[u8]>` to `self`, eg: &[u8]/&str/String/array/vec, etc.
    fn push<V: AsRef<[u8]>>(&mut self, value: V) -> JResult<usize> {
        let data = value.as_ref();
        let data_len = data.len();

        if data_len > self.remaining_len() {
            if self.resize(data_len) == 0 {
                return Err(make_error(self.remaining(), self.get_position(), ErrorKind::PushFail));
            }
        }

        self.remaining_mut()[..data_len].clone_from_slice(data);
        self.advance(data_len);

        Ok(data_len)
    }

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
    fn push_uint(&mut self, value: usize, nbytes: usize) -> JResult<usize> {
        self.push_be_uint(value, nbytes)
    }

    /// Writes an unsigned n-byte integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_uint(&mut self, value: usize, nbytes: usize) -> JResult<usize> {
        let start = match mem::size_of_val(&value).checked_sub(nbytes) {
            Some(start) => start,
            None => return Err(make_error(self.remaining(), self.get_position(), ErrorKind::InvalidByteLength)),
        };

        self.push(&value.to_be_bytes()[start..])
    }

    /// Writes an unsigned n-byte integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_uint(&mut self, value: usize, nbytes: usize) -> JResult<usize> {
        let slice = value.to_le_bytes();
        let slice = match slice.get(..nbytes) {
            Some(slice) => slice,
            None => return Err(make_error(self.remaining(), self.get_position(), ErrorKind::InvalidByteLength)),
        };

        self.push(slice)
    }

    /// Writes an unsigned n-byte integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_uint(&mut self, value: usize, nbytes: usize) -> JResult<usize> {
        if cfg!(target_endian = "big") {
            self.push_be_uint(value, nbytes)
        } else {
            self.push_le_uint(value, nbytes)
        }
    }

    /// Writes a signed n-byte integer to `self` in big-endian byte order.
    #[inline]
    fn push_int(&mut self, value: isize, nbytes: usize) -> JResult<usize> {
        self.push_uint(value as usize, nbytes)
    }

    /// Writes a signed n-byte integer to `self` in big-endian byte order.
    #[inline]
    fn push_be_int(&mut self, value: isize, nbytes: usize) -> JResult<usize> {
        self.push_be_uint(value as usize, nbytes)
    }

    /// Writes a signed n-byte integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_int(&mut self, value: isize, nbytes: usize) -> JResult<usize> {
        self.push_le_uint(value as usize, nbytes)
    }

    /// Writes a signed n-byte integer to `self` in native-endian byte order.
    #[inline]
    fn push_ne_int(&mut self, value: isize, nbytes: usize) -> JResult<usize> {
        if cfg!(target_endian = "big") {
            self.push_be_int(value, nbytes)
        } else {
            self.push_le_int(value, nbytes)
        }
    }

    /// Writes an IEEE754 single-precision (4 bytes) floating point number to `self` in big-endian byte order.
    #[inline]
    fn push_f32(&mut self, value: f32) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an IEEE754 single-precision (4 bytes) floating point number to `self` in big-endian byte order.
    #[inline]
    fn push_be_f32(&mut self, value: f32) -> JResult<usize> {
        self.push(value.to_be_bytes())
    }

    /// Writes an IEEE754 single-precision (4 bytes) floating point number to `self` in little-endian byte order.
    #[inline]
    fn push_le_f32(&mut self, value: f32) -> JResult<usize> {
        self.push(value.to_le_bytes())
    }

    /// Writes an IEEE754 single-precision (4 bytes) floating point number to `self` in native-endian byte order.
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