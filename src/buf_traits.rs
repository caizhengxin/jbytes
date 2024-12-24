use memchr::memmem;
use aho_corasick::AhoCorasick;
use crate::{
    JResult, ErrorKind, make_error,
    BufWriteMut, ByteOrder,
    macro_take_bytes,
};


pub trait BufRead {
    /// Get the internal cursor of the `self`.
    fn get_position(&self) -> usize;

    /// Get the internal data of the `self`.
    fn get_data(&self) -> &'_ [u8];

    /// Advance the internal cursor of the `self`.
    fn advance(&self, nbytes: usize);

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
    fn copy_to_slice(&self, dst: &mut [u8]) -> JResult<()> {
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
    fn take_array<const N: usize>(&self) -> JResult<[u8; N]> {
        let mut array = [0_u8; N];

        self.copy_to_slice(&mut array)?;

        Ok(array)
    }

    /// Reads n-byte data from `self`.
    #[inline]
    fn take_bytes(&self, nbytes: usize) -> JResult<&'_ [u8]> {
        if self.remaining_len() < nbytes {
            return Err(make_error(self.get_position(), ErrorKind::InvalidByteLength));
        }

        self.advance(nbytes);

        let position = self.get_position();

        Ok(&self.get_data()[position - nbytes..position])
    }

    /// Reads a bool from `self`.
    #[inline]
    fn take_bool(&self) -> JResult<bool> {
        Ok(self.take_bytes(1)?[0] != 0)
    }

    /// Reads an unsigned 8 bit integer from `self`.
    #[inline]
    fn take_u8(&self) -> JResult<u8> {
        Ok(self.take_bytes(1)?[0])
    }

    /// Reads an unsigned 8 bit integer from `self`, exactly like the `[take_u8]` function.
    #[inline]
    fn take_be_u8(&self) -> JResult<u8> {
        self.take_u8()
    }

    /// Reads an unsigned 8 bit integer from `self`, exactly like the `[take_u8]` function.
    #[inline]
    fn take_le_u8(&self) -> JResult<u8> {
        self.take_u8()
    }

    /// Reads an unsigned 8 bit integer from `self`, exactly like the `[take_u8]` function.
    #[inline]
    fn take_ne_u8(&self) -> JResult<u8> {
        self.take_u8()
    }

    /// Reads an unsigned 8 bit integer from `self`, exactly like the `[take_u8]` function.
    #[inline]
    fn take_byteorder_u8(&self, _byteorder: ByteOrder) -> JResult<u8> {
        self.take_u8()
    }

    /// Reads a signed 8 bit integer from `self`.
    #[inline]
    fn take_i8(&self) -> JResult<i8> {
        Ok(self.take_bytes(1)?[0] as i8)
    }

    /// Reads a signed 8 bit integer from `self`, exactly like the `[take_i8]` function.
    #[inline]
    fn take_be_i8(&self) -> JResult<i8> {
        self.take_i8()
    }

    /// Reads a signed 8 bit integer from `self`, exactly like the `[take_i8]` function.
    #[inline]
    fn take_le_i8(&self) -> JResult<i8> {
        self.take_i8()
    }

    /// Reads a signed 8 bit integer from `self`, exactly like the `[take_i8]` function.
    #[inline]
    fn take_ne_i8(&self) -> JResult<i8> {
        self.take_i8()
    }

    /// Reads a signed 8 bit integer from `self`, exactly like the `[take_i8]` function.
    #[inline]
    fn take_byteorder_i8(&self, _byteorder: ByteOrder) -> JResult<i8> {
        self.take_i8()
    }

    /// Reads an unsigned 16 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u16(&self) -> JResult<u16> {
        macro_take_bytes!(self, u16::from_be_bytes);
    }

    /// Reads an unsigned 16 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u16(&self) -> JResult<u16> {
        macro_take_bytes!(self, u16::from_be_bytes);
    }

    /// Reads an unsigned 16 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u16(&self) -> JResult<u16> {
        macro_take_bytes!(self, u16::from_le_bytes);
    }

    /// Reads an unsigned 16 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u16(&self) -> JResult<u16> {
        macro_take_bytes!(self, u16::from_ne_bytes);
    }

    /// Reads an unsigned 16 bit integer from `self`.
    #[inline]
    fn take_byteorder_u16(&self, byteorder: ByteOrder) -> JResult<u16> {
        match byteorder {
            ByteOrder::Be => self.take_be_u16(),
            ByteOrder::Le => self.take_le_u16(),
        }
    }

    /// Reads a signed 16 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_i16(&self) -> JResult<i16> {
        macro_take_bytes!(self, i16::from_be_bytes);
    }

    /// Reads a signed 16 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_i16(&self) -> JResult<i16> {
        macro_take_bytes!(self, i16::from_be_bytes);
    }

    /// Reads a signed 16 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_i16(&self) -> JResult<i16> {
        macro_take_bytes!(self, i16::from_le_bytes);
    }

    /// Reads a signed 16 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_i16(&self) -> JResult<i16> {
        macro_take_bytes!(self, i16::from_ne_bytes);
    }

    /// Reads a signed 16 bit integer from `self`.
    #[inline]
    fn take_byteorder_i16(&self, byteorder: ByteOrder) -> JResult<i16> {
        match byteorder {
            ByteOrder::Be => self.take_be_i16(),
            ByteOrder::Le => self.take_le_i16(),
        }
    }

    /// Reads an unsigned 24 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u24(&self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_be_bytes, 3);
    }

    /// Reads an unsigned 24 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u24(&self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_be_bytes, 3);
    }

    /// Reads an unsigned 24 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u24(&self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_le_bytes, 3);
    }

    /// Reads an unsigned 24 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u24(&self) -> JResult<u32> {
        if cfg!(target_endian = "big") {
            self.take_be_u24()
        } else {
            self.take_le_u24()
        }
    }

    /// Reads an unsigned 24 bit integer from `self`.
    #[inline]
    fn take_byteorder_u24(&self, byteorder: ByteOrder) -> JResult<u32> {
        match byteorder {
            ByteOrder::Be => self.take_be_u24(),
            ByteOrder::Le => self.take_le_u24(),
        }
    }

    /// Reads an unsigned 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u32(&self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_be_bytes);
    }

    /// Reads an unsigned 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u32(&self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_be_bytes);
    }

    /// Reads an unsigned 32 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u32(&self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_le_bytes);
    }

    /// Reads an unsigned 32 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u32(&self) -> JResult<u32> {
        macro_take_bytes!(self, u32::from_ne_bytes);
    }

    /// Reads an unsigned 32 bit integer from `self`.
    #[inline]
    fn take_byteorder_u32(&self, byteorder: ByteOrder) -> JResult<u32> {
        match byteorder {
            ByteOrder::Be => self.take_be_u32(),
            ByteOrder::Le => self.take_le_u32(),
        }
    }

    /// Reads a signed 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_i32(&self) -> JResult<i32> {
        macro_take_bytes!(self, i32::from_be_bytes);
    }

    /// Reads a signed 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_i32(&self) -> JResult<i32> {
        macro_take_bytes!(self, i32::from_be_bytes);
    }

    /// Reads a signed 32 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_i32(&self) -> JResult<i32> {
        macro_take_bytes!(self, i32::from_le_bytes);
    }

    /// Reads a signed 32 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_i32(&self) -> JResult<i32> {
        macro_take_bytes!(self, i32::from_ne_bytes);
    }

    /// Reads a signed 32 bit integer from `self`.
    #[inline]
    fn take_byteorder_i32(&self, byteorder: ByteOrder) -> JResult<i32> {
        match byteorder {
            ByteOrder::Be => self.take_be_i32(),
            ByteOrder::Le => self.take_le_i32(),
        }
    }

    /// Reads an unsigned 32 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u64(&self) -> JResult<u64> {
        macro_take_bytes!(self, u64::from_be_bytes);
    }

    /// Reads an unsigned 64 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u64(&self) -> JResult<u64> {
        macro_take_bytes!(self, u64::from_be_bytes);
    }

    /// Reads an unsigned 64 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u64(&self) -> JResult<u64> {
        macro_take_bytes!(self, u64::from_le_bytes);
    }

    /// Reads an unsigned 64 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u64(&self) -> JResult<u64> {
        macro_take_bytes!(self, u64::from_ne_bytes);
    }

    /// Reads an unsigned 64 bit integer from `self`.
    #[inline]
    fn take_byteorder_u64(&self, byteorder: ByteOrder) -> JResult<u64> {
        match byteorder {
            ByteOrder::Be => self.take_be_u64(),
            ByteOrder::Le => self.take_le_u64(),
        }
    }

    /// Reads a signed 64 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_i64(&self) -> JResult<i64> {
        macro_take_bytes!(self, i64::from_be_bytes);
    }

    /// Reads a signed 64 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_i64(&self) -> JResult<i64> {
        macro_take_bytes!(self, i64::from_be_bytes);
    }

    /// Reads a signed 64 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_i64(&self) -> JResult<i64> {
        macro_take_bytes!(self, i64::from_le_bytes);
    }

    /// Reads a signed 64 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_i64(&self) -> JResult<i64> {
        macro_take_bytes!(self, i64::from_ne_bytes);
    }

    /// Reads a signed 64 bit integer from `self`.
    #[inline]
    fn take_byteorder_i64(&self, byteorder: ByteOrder) -> JResult<i64> {
        match byteorder {
            ByteOrder::Be => self.take_be_i64(),
            ByteOrder::Le => self.take_le_i64(),
        }
    }

    /// Reads an unsigned 128 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_u128(&self) -> JResult<u128> {
        macro_take_bytes!(self, u128::from_be_bytes);
    }

    /// Reads an unsigned 128 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_u128(&self) -> JResult<u128> {
        macro_take_bytes!(self, u128::from_be_bytes);
    }

    /// Reads an unsigned 128 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_u128(&self) -> JResult<u128> {
        macro_take_bytes!(self, u128::from_le_bytes);
    }

    /// Reads an unsigned 128 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_u128(&self) -> JResult<u128> {
        macro_take_bytes!(self, u128::from_ne_bytes);
    }

    /// Reads an unsigned 128 bit integer from `self`.
    #[inline]
    fn take_byteorder_u128(&self, byteorder: ByteOrder) -> JResult<u128> {
        match byteorder {
            ByteOrder::Be => self.take_be_u128(),
            ByteOrder::Le => self.take_le_u128(),
        }
    }

    /// Reads a signed 128 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_i128(&self) -> JResult<i128> {
        macro_take_bytes!(self, i128::from_be_bytes);
    }

    /// Reads a signed 128 bit integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_i128(&self) -> JResult<i128> {
        macro_take_bytes!(self, i128::from_be_bytes);
    }

    /// Reads a signed 128 bit integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_i128(&self) -> JResult<i128> {
        macro_take_bytes!(self, i128::from_le_bytes);
    }

    /// Reads a signed 128 bit integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_i128(&self) -> JResult<i128> {
        macro_take_bytes!(self, i128::from_ne_bytes);
    }

    /// Reads a signed 128 bit integer from `self`.
    #[inline]
    fn take_byteorder_i128(&self, byteorder: ByteOrder) -> JResult<i128> {
        match byteorder {
            ByteOrder::Be => self.take_be_i128(),
            ByteOrder::Le => self.take_le_i128(),
        }
    }

    /// Reads an unsigned size integer from `self` in big-endian byte order.
    #[inline]
    fn take_usize(&self) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_be_bytes);
    }

    /// Reads an unsigned size integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_usize(&self) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_be_bytes);
    }

    /// Reads an unsigned size integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_usize(&self) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_le_bytes);
    }

    /// Reads an unsigned size integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_usize(&self) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_ne_bytes);
    }

    /// Reads an unsigned size bit integer from `self`.
    #[inline]
    fn take_byteorder_usize(&self, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.take_be_usize(),
            ByteOrder::Le => self.take_le_usize(),
        }
    }

    /// Reads a signed size integer from `self` in big-endian byte order.
    #[inline]
    fn take_isize(&self) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_be_bytes);
    }

    /// Reads a signed size integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_isize(&self) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_be_bytes);
    }

    /// Reads a signed size integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_isize(&self) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_le_bytes);
    }

    /// Reads a signed size integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_isize(&self) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_ne_bytes);
    }

    /// Reads a signed size bit integer from `self`.
    #[inline]
    fn take_byteorder_isize(&self, byteorder: ByteOrder) -> JResult<isize> {
        match byteorder {
            ByteOrder::Be => self.take_be_isize(),
            ByteOrder::Le => self.take_le_isize(),
        }
    }

    /// Reads an unsigned n-byte integer from `self` in big-endian byte order.
    #[inline]
    fn take_uint(&self, nbytes: usize) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_be_bytes, nbytes);
    }

    /// Reads an unsigned n-byte integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_uint(&self, nbytes: usize) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_be_bytes, nbytes);
    }

    /// Reads an unsigned n-byte integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_uint(&self, nbytes: usize) -> JResult<usize> {
        macro_take_bytes!(self, usize::from_le_bytes, nbytes);
    }

    /// Reads an unsigned n-byte integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_uint(&self, nbytes: usize) -> JResult<usize> {
        if cfg!(target_endian = "big") {
            self.take_be_uint(nbytes)
        } else {
            self.take_le_uint(nbytes)
        }
    }

    /// Reads an unsigned n-byte integer from `self`.
    #[inline]
    fn take_byteorder_uint(&self, nbytes: usize, byteorder: ByteOrder) -> JResult<usize> {
        match byteorder {
            ByteOrder::Be => self.take_be_uint(nbytes),
            ByteOrder::Le => self.take_le_uint(nbytes),
        }
    }

    /// Reads a signed n-byte integer from `self` in big-endian byte order.
    #[inline]
    fn take_int(&self, nbytes: usize) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_be_bytes, nbytes);
    }

    /// Reads a signed n-byte integer from `self` in big-endian byte order.
    #[inline]
    fn take_be_int(&self, nbytes: usize) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_be_bytes, nbytes);
    }

    /// Reads a signed n-byte integer from `self` in little-endian byte order.
    #[inline]
    fn take_le_int(&self, nbytes: usize) -> JResult<isize> {
        macro_take_bytes!(self, isize::from_le_bytes, nbytes);
    }

    /// Reads a signed n-byte integer from `self` in native-endian byte order.
    #[inline]
    fn take_ne_int(&self, nbytes: usize) -> JResult<isize> {
        if cfg!(target_endian = "big") {
            self.take_be_int(nbytes)
        } else {
            self.take_le_int(nbytes)
        }
    }

    /// Reads a signed n-byte integer from `self`.
    #[inline]
    fn take_byteorder_int(&self, nbytes: usize, byteorder: ByteOrder) -> JResult<isize> {
        match byteorder {
            ByteOrder::Be => self.take_be_int(nbytes),
            ByteOrder::Le => self.take_le_int(nbytes),
        }
    }

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self` in big-endian byte order.
    #[inline]
    fn take_f32(&self) -> JResult<f32> {
        macro_take_bytes!(self, f32::from_be_bytes);
    }

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self` in big-endian byte order.
    #[inline]
    fn take_be_f32(&self) -> JResult<f32> {
        macro_take_bytes!(self, f32::from_be_bytes);
    }

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self` in little-endian byte order.
    #[inline]
    fn take_le_f32(&self) -> JResult<f32> {
        macro_take_bytes!(self, f32::from_le_bytes);
    }

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self` in native-endian byte order.
    #[inline]
    fn take_ne_f32(&self) -> JResult<f32> {
        macro_take_bytes!(self, f32::from_ne_bytes);
    }

    /// Reads an IEEE754 single-precision (4 bytes) floating point number from `self`.
    #[inline]
    fn take_byteorder_f32(&self, byteorder: ByteOrder) -> JResult<f32> {
        match byteorder {
            ByteOrder::Be => self.take_be_f32(),
            ByteOrder::Le => self.take_le_f32(),
        }
    }

    /// Reads an IEEE754 double-precision (8 bytes) floating point number from `self` in big-endian byte order.
    #[inline]
    fn take_f64(&self) -> JResult<f64> {
        macro_take_bytes!(self, f64::from_be_bytes);
    }

    /// Reads an IEEE754 double-precision (8 bytes) floating point number from `self` in big-endian byte order.
    #[inline]
    fn take_be_f64(&self) -> JResult<f64> {
        macro_take_bytes!(self, f64::from_be_bytes);
    }

    /// Reads an IEEE754 double-precision (8 bytes) floating point number from `self` in little-endian byte order.
    #[inline]
    fn take_le_f64(&self) -> JResult<f64> {
        macro_take_bytes!(self, f64::from_le_bytes);
    }

    /// Reads an IEEE754 double-precision (8 bytes) floating point number from `self` in native-endian byte order.
    #[inline]
    fn take_ne_f64(&self) -> JResult<f64> {
        macro_take_bytes!(self, f64::from_ne_bytes);
    }

    /// Reads an IEEE754 single-precision (8 bytes) floating point number from `self`.
    #[inline]
    fn take_byteorder_f64(&self, byteorder: ByteOrder) -> JResult<f64> {
        match byteorder {
            ByteOrder::Be => self.take_be_f64(),
            ByteOrder::Le => self.take_le_f64(),
        }
    }

    // Finds a substring in a byte stream
    #[inline]
    fn find_subsequence<I: AsRef<[u8]>>(&self, needle: I) -> JResult<&[u8]> {
        let position = self.get_position();

        if let Some(subposition) = memmem::find(self.remaining(), needle.as_ref()) {
            self.advance(subposition + needle.as_ref().len());
            return Ok(&self.get_data()[position..position + subposition]);
        }

        Err(make_error(position, ErrorKind::Fail))
    }

    // Finds a substring in a byte stream
    #[inline]
    fn find_subsequence_needle<I: AsRef<[u8]>>(&self, needle: I, include_needle: bool) -> JResult<&[u8]> {
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
    fn find_subsequences<I, P>(&self, needle: I) -> JResult<&[u8]>
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
    fn find_subsequences_needle<I, P>(&self, needle: I, include_needle: bool) -> JResult<&[u8]>
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


pub trait BufWrite: BufWriteMut {
}