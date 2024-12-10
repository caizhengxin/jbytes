use crate::errors::JResult;
use crate::byteorder::ByteOrder;


pub trait Take {
    fn take(&mut self, nbyte: usize) -> JResult<&'_ [u8]>;

    fn take_int(&mut self, byteorder: ByteOrder, nbyte: u8) -> JResult<u128>;

    #[inline]
    fn take_be_int(&mut self, nbyte: u8) -> JResult<u128> {
        self.take_int(ByteOrder::Be, nbyte)
    }

    #[inline]
    fn take_le_int(&mut self, nbyte: u8) -> JResult<u128> {
        self.take_int(ByteOrder::Le, nbyte)
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
