use crate::{
    traits::{BufRead, BufWrite},
    errors::{JResult, make_error, ErrorKind},
};


impl BufRead for &'_ [u8] {
    fn get_position(&self) -> usize {
        0
    }
    fn advance(&mut self, nbytes: usize) {
        *self = &self[nbytes..]
    }

    fn remaining(&self) -> &'_ [u8] {
        &self
    }

    fn take_bytes(&mut self, nbytes: usize) -> JResult<&'_ [u8]> {
        let value = match self.get(..nbytes) {
            Some(value) => value,
            None => return Err(make_error(self.remaining(), 0, ErrorKind::InvalidByteLength)),
        };

        Ok(value)
    }
}