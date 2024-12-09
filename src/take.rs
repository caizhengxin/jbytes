use crate::errors::JResult;
use crate::byteorder::ByteOrder;


pub trait Take {
    fn take(&mut self, nbyte: usize) -> JResult<&'_ [u8]>;

    fn take_int(&mut self, byteorder: ByteOrder, nbyte: u8) -> JResult<u128>;

    fn take_be_int(&mut self, nbyte: u8) -> JResult<u128>;

    fn take_le_int(&mut self, nbyte: u8) -> JResult<u128>;

    fn take_u8(&mut self) -> JResult<u8>;

    fn take_be_u8(&mut self) -> JResult<u8>;

    fn take_le_u8(&mut self) -> JResult<u8>;

    fn take_be_u16(&mut self) -> JResult<u16>;

    fn take_le_u16(&mut self) -> JResult<u16>;

    fn take_be_u24(&mut self) -> JResult<u32>;

    fn take_le_u24(&mut self) -> JResult<u32>;

    fn take_be_u32(&mut self) -> JResult<u32>;

    fn take_le_u32(&mut self) -> JResult<u32>;

    fn take_be_u64(&mut self) -> JResult<u64>;

    fn take_le_u64(&mut self) -> JResult<u64>;

    fn take_be_u128(&mut self) -> JResult<u128>;

    fn take_le_u128(&mut self) -> JResult<u128>;
}
