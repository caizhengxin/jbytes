use core::mem;
use memchr::memmem;
use aho_corasick::AhoCorasick;
use crate::{
    ByteOrder,
    errors::{JResult, make_error, ErrorKind},
};


#[macro_export]
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
            None => return Err(make_error($this.get_position(), ErrorKind::InvalidByteLength)),
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


pub trait BufReadMut {
    /// Get the internal cursor of the `self`.
    fn get_position(&self) -> usize;

    /// Get the internal data of the `self`.
    fn get_data(&self) -> &'_ [u8];

    /// Advance the internal cursor of the `self`.
    fn advance(&mut self, nbytes: usize);

    /// Returns the n-bytes between the current position and the end of the buffer.
    #[inline]
    fn remaining(&self) -> &'_ [u8] {
        &self.get_data().get(self.get_position()..).unwrap_or(&[])
    }

    /// Returns the number of bytes between the current position and the end of the buffer.
    #[inline]
    fn remaining_len(&self) -> usize {
        self.remaining().len()
    }

    /// Reads n-byte data to arrary from `self`.
    #[inline]
    fn copy_to_slice(&mut self, dst: &mut [u8]) -> JResult<()> {
        let value = match self.remaining().get(..dst.len()) {
            Some(value) => value,
            None => return Err(make_error(self.get_position(), ErrorKind::InvalidByteLength)),
        };

        dst.copy_from_slice(value);
        self.advance(dst.len());

        Ok(())
    }

    /// Reads n-byte data from `self`.
    #[inline]
    fn take_array<const N: usize>(&mut self) -> JResult<[u8; N]> {
        let mut array = [0_u8; N];

        self.copy_to_slice(&mut array)?;

        Ok(array)
    }

    /// Reads n-byte data from `self`.
    #[inline]
    fn take_bytes(&mut self, nbytes: usize) -> JResult<&'_ [u8]> {
        if self.remaining_len() < nbytes {
            return Err(make_error(self.get_position(), ErrorKind::InvalidByteLength));
        }

        self.advance(nbytes);

        let position = self.get_position();

        Ok(&self.get_data()[position - nbytes..position])
    }

    /// Reads a bool from `self`.
    #[inline]
    fn take_bool(&mut self) -> JResult<bool> {
        Ok(self.take_bytes(1)?[0] != 0)
    }

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

    /// Reads an unsigned 8 bit integer from `self`, exactly like the `[take_u8]` function.
    #[inline]
    fn take_byteorder_u8(&mut self, _byteorder: ByteOrder) -> JResult<u8> {
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

    /// Reads a signed 8 bit integer from `self`, exactly like the `[take_i8]` function.
    #[inline]
    fn take_byteorder_i8(&mut self, _byteorder: ByteOrder) -> JResult<i8> {
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

    /// Reads an unsigned 16 bit integer from `self`.
    #[inline]
    fn take_byteorder_u16(&mut self, byteorder: ByteOrder) -> JResult<u16> {
        match byteorder {
            ByteOrder::Be => self.take_be_u16(),
            ByteOrder::Le => self.take_le_u16(),
        }
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

    /// Reads a signed 16 bit integer from `self`.
    #[inline]
    fn take_byteorder_i16(&mut self, byteorder: ByteOrder) -> JResult<i16> {
        match byteorder {
            ByteOrder::Be => self.take_be_i16(),
            ByteOrder::Le => self.take_le_i16(),
        }
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

    /// Reads an unsigned 24 bit integer from `self`.
    #[inline]
    fn take_byteorder_u24(&mut self, byteorder: ByteOrder) -> JResult<u32> {
        match byteorder {
            ByteOrder::Be => self.take_be_u24(),
            ByteOrder::Le => self.take_le_u24(),
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

    /// Reads an unsigned 32 bit integer from `self`.
    #[inline]
    fn take_byteorder_u32(&mut self, byteorder: ByteOrder) -> JResult<u32> {
        match byteorder {
            ByteOrder::Be => self.take_be_u32(),
            ByteOrder::Le => self.take_le_u32(),
        }
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

    /// Reads a signed 32 bit integer from `self`.
    #[inline]
    fn take_byteorder_i32(&mut self, byteorder: ByteOrder) -> JResult<i32> {
        match byteorder {
            ByteOrder::Be => self.take_be_i32(),
            ByteOrder::Le => self.take_le_i32(),
        }
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

    /// Reads an unsigned 64 bit integer from `self`.
    #[inline]
    fn take_byteorder_u64(&mut self, byteorder: ByteOrder) -> JResult<u64> {
        match byteorder {
            ByteOrder::Be => self.take_be_u64(),
            ByteOrder::Le => self.take_le_u64(),
        }
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

    /// Reads a signed 64 bit integer from `self`.
    #[inline]
    fn take_byteorder_i64(&mut self, byteorder: ByteOrder) -> JResult<i64> {
        match byteorder {
            ByteOrder::Be => self.take_be_i64(),
            ByteOrder::Le => self.take_le_i64(),
        }
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

    /// Reads an unsigned 128 bit integer from `self`.
    #[inline]
    fn take_byteorder_u128(&mut self, byteorder: ByteOrder) -> JResult<u128> {
        match byteorder {
            ByteOrder::Be => self.take_be_u128(),
            ByteOrder::Le => self.take_le_u128(),
        }
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

    /// Reads a signed 128 bit integer from `self`.
    #[inline]
    fn take_byteorder_i128(&mut self, byteorder: ByteOrder) -> JResult<i128> {
        match byteorder {
            ByteOrder::Be => self.take_be_i128(),
            ByteOrder::Le => self.take_le_i128(),
        }
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

    /// Reads an unsigned size bit integer from `self`.
    #[inline]
    fn take_byteorder_usize(&mut self, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.take_be_usize(),
            ByteOrder::Le => self.take_le_usize(),
        }
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

    /// Reads a signed size bit integer from `self`.
    #[inline]
    fn take_byteorder_isize(&mut self, byteorder: ByteOrder) -> JResult<isize> {
        match byteorder {
            ByteOrder::Be => self.take_be_isize(),
            ByteOrder::Le => self.take_le_isize(),
        }
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

    /// Reads an unsigned n-byte integer from `self`.
    #[inline]
    fn take_byteorder_uint(&mut self, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.take_be_usize(),
            ByteOrder::Le => self.take_le_usize(),
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

    /// Reads a signed n-byte integer from `self`.
    #[inline]
    fn take_byteorder_int(&mut self, byteorder: ByteOrder) -> JResult<isize> {
        match byteorder {
            ByteOrder::Be => self.take_be_isize(),
            ByteOrder::Le => self.take_le_isize(),
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

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self`.
    #[inline]
    fn take_byteorder_f32(&mut self, byteorder: ByteOrder) -> JResult<f32> {
        match byteorder {
            ByteOrder::Be => self.take_be_f32(),
            ByteOrder::Le => self.take_le_f32(),
        }
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

    /// Reads an IEEE754 single-precision (8 bytes) floating point number from `self`.
    #[inline]
    fn take_byteorder_f64(&mut self, byteorder: ByteOrder) -> JResult<f64> {
        match byteorder {
            ByteOrder::Be => self.take_be_f64(),
            ByteOrder::Le => self.take_le_f64(),
        }
    }

    // Finds a substring in a byte stream
    #[inline]
    fn find_subsequence<I: AsRef<[u8]>>(&mut self, needle: I) -> JResult<&[u8]> {
        let position = self.get_position();

        if let Some(subposition) = memmem::find(self.remaining(), needle.as_ref()) {
            self.advance(subposition + needle.as_ref().len());
            return Ok(&self.get_data()[position..position + subposition]);
        }

        Err(make_error(position, ErrorKind::Fail))
    }

    // Finds a substring in a byte stream
    #[inline]
    fn find_subsequence_needle<I: AsRef<[u8]>>(&mut self, needle: I, include_needle: bool) -> JResult<&[u8]> {
        let position = self.get_position();

        if let Some(subposition) = memmem::find(self.remaining(), needle.as_ref()) {
            let subposition = if include_needle { subposition + needle.as_ref().len() } else { subposition };
            self.advance(subposition);
            return Ok(&self.get_data()[position..position + subposition]);
        }

        Err(make_error(position, ErrorKind::Fail))
    }

    // Finds a substring in a byte stream
    #[inline]
    fn find_subsequences<I, P>(&mut self, needle: I) -> JResult<&[u8]>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<[u8]>,
    {
        let position = self.get_position();

        if let Ok(ac) = AhoCorasick::new(needle) {
            if let Some(mt) = ac.find(self.remaining()) {
                self.advance(mt.end());
                return Ok(&self.get_data()[position..position + mt.start()]);
            }
        }

        Err(make_error(position, ErrorKind::Fail))
    }

    // Finds a substring in a byte stream, include needle
    #[inline]
    fn find_subsequences_needle<P, I>(&mut self, needle: I, include_needle: bool) -> JResult<&[u8]>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<[u8]>,
    {
        let position = self.get_position();

        if let Ok(ac) = AhoCorasick::new(needle) {
            if let Some(mt) = ac.find(self.remaining()) {
                let subposition = if include_needle { mt.end()} else { mt.start() };
                self.advance(subposition);
                return Ok(&self.get_data()[position..position + subposition]);
            }
        }

        Err(make_error(position, ErrorKind::Fail))
    }
}


pub trait BufWriteMut: BufReadMut {
    /// Returns the n-bytes between the current position and the end of the buffer.
    fn remaining_mut(&mut self) -> &'_ mut [u8];

    /// Update memory size.
    fn resize(&mut self, nbytes: usize) -> usize;

    /// Writes `AsRef<[u8]>` to `self`, eg: &[u8]/&str/String/array/vec, etc.
    fn push<V: AsRef<[u8]>>(&mut self, value: V) -> JResult<usize> {
        let data = value.as_ref();
        let data_len = data.len();

        if data_len > self.remaining_len() && self.resize(data_len) == 0 {
            return Err(make_error(self.get_position(), ErrorKind::PushFail));
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

    /// Writes a bool to `self`.
    #[inline]
    fn push_bool(&mut self, value: bool) -> JResult<usize> {
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

    /// Writes an unsigned 8 bit integer to `self`, exactly like the `[push_u8]` function.
    #[inline]
    fn push_byteorder_u8(&mut self, value: u8, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_u8(value),
            ByteOrder::Le => self.push_le_u8(value),
        }
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

    /// Writes a signed 8 bit integer to `self`.
    #[inline]
    fn push_byteorder_i8(&mut self, value: i8, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_i8(value),
            ByteOrder::Le => self.push_le_i8(value),
        }
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

    /// Writes an unsigned 16 bit integer to `self`.
    #[inline]
    fn push_byteorder_u16(&mut self, value: u16, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_u16(value),
            ByteOrder::Le => self.push_le_u16(value),
        }
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

    /// Writes a signed 16 bit integer to `self`.
    #[inline]
    fn push_byteorder_i16(&mut self, value: i16, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_i16(value),
            ByteOrder::Le => self.push_le_i16(value),
        }
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

    /// Writes an unsigned 24 bit integer to `self`.
    #[inline]
    fn push_byteorder_u24(&mut self, value: u32, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_u24(value),
            ByteOrder::Le => self.push_le_u24(value),
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

    /// Writes an unsigned 32 bit integer to `self`.
    #[inline]
    fn push_byteorder_u32(&mut self, value: u32, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_u32(value),
            ByteOrder::Le => self.push_le_u32(value),
        }
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

    /// Writes a signed 32 bit integer to `self`.
    #[inline]
    fn push_byteorder_i32(&mut self, value: i32, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_i32(value),
            ByteOrder::Le => self.push_le_i32(value),
        }
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

    /// Writes an unsigned 64 bit integer to `self`.
    #[inline]
    fn push_byteorder_u64(&mut self, value: u64, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_u64(value),
            ByteOrder::Le => self.push_le_u64(value),
        }
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

    /// Writes a signed 64 bit integer to `self`.
    #[inline]
    fn push_byteorder_i64(&mut self, value: i64, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_i64(value),
            ByteOrder::Le => self.push_le_i64(value),
        }
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

    /// Writes an unsigned 128 bit integer to `self`.
    #[inline]
    fn push_byteorder_u128(&mut self, value: u128, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_u128(value),
            ByteOrder::Le => self.push_le_u128(value),
        }
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

    /// Writes a signed 128 bit integer to `self`.
    #[inline]
    fn push_byteorder_i128(&mut self, value: i128, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_i128(value),
            ByteOrder::Le => self.push_le_i128(value),
        }
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

    /// Writes an unsigned usize integer to `self`.
    #[inline]
    fn push_byteorder_usize(&mut self, value: usize, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_usize(value),
            ByteOrder::Le => self.push_le_usize(value),
        }
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

    /// Writes a signed usize integer to `self`.
    #[inline]
    fn push_byteorder_isize(&mut self, value: isize, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_isize(value),
            ByteOrder::Le => self.push_le_isize(value),
        }
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
            None => return Err(make_error(self.get_position(), ErrorKind::InvalidByteLength)),
        };

        self.push(&value.to_be_bytes()[start..])
    }

    /// Writes an unsigned n-byte integer to `self` in little-endian byte order.
    #[inline]
    fn push_le_uint(&mut self, value: usize, nbytes: usize) -> JResult<usize> {
        let slice = value.to_le_bytes();
        let slice = match slice.get(..nbytes) {
            Some(slice) => slice,
            None => return Err(make_error(self.get_position(), ErrorKind::InvalidByteLength)),
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

    /// Writes an unsigned n-byte integer to `self`.
    #[inline]
    fn push_byteorder_uint(&mut self, value: usize, nbytes: usize, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_uint(value, nbytes),
            ByteOrder::Le => self.push_le_uint(value, nbytes),
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

    /// Writes a signed n-byte integer to `self`.
    #[inline]
    fn push_byteorder_int(&mut self, value: isize, nbytes: usize, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_int(value, nbytes),
            ByteOrder::Le => self.push_le_int(value, nbytes),
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

    /// Writes an IEEE754 double-precision (4 bytes) floating point number to `self`.
    #[inline]
    fn push_byteorder_f32(&mut self, value: f32, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_f32(value),
            ByteOrder::Le => self.push_le_f32(value),
        }
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

    /// Writes an IEEE754 double-precision (8 bytes) floating point number to `self`.
    #[inline]
    fn push_byteorder_f64(&mut self, value: f64, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.push_be_f64(value),
            ByteOrder::Le => self.push_le_f64(value),
        }
    }
}