#[cfg(not(feature = "std"))]
use thiserror_no_std::Error as ThisError;
#[cfg(feature = "std")]
use thiserror::Error as ThisError;
use crate::std::fmt;


pub type JResult<'a, O, E = Error<&'a [u8]>> = Result<O, E>;


#[derive(Debug, PartialEq, Eq)]
pub struct Error<I> {
    pub current_offset: usize,
    pub input: I,
    pub code: ErrorKind,
}


impl<I> Error<I> {
    pub fn new(input: I, offset: usize, kind: ErrorKind) -> Self {
        Self { input, current_offset: offset, code: kind }
    }
}


impl<I> fmt::Display for Error<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {     
           write!(f, "[ERROR]: {}, current_offset: {}, remaining_input: {:?}", self.code, self.current_offset, self.input)
    }
}


pub trait ParseError<I> {
    fn from_error_kind(input: I, offset: usize, kind: ErrorKind) -> Self;
}


impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, offset: usize, kind: ErrorKind) -> Self {
        Error::new(input, offset, kind)
    }
}


#[derive(Debug, PartialEq, Eq, ThisError)]
pub enum ErrorKind {
    #[error("invalid byte length")]
    InvalidByteLength,
    #[error("find subsequence failure")]
    SubSequence,
    #[error("parse byte failure")]
    Fail,
    #[error("invalid position ({0})")]
    InvalidPosition(usize),
    #[error("invalid buffer memory")]
    PushFail,
}


#[inline]
pub fn make_error<I, E: ParseError<I>>(input: I, offset: usize, kind: ErrorKind) -> E {
    E::from_error_kind(input, offset, kind)
}


#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn test_error() {
        let error = Error::new([0x01_u8, 0x02, 0x03], 10, super::ErrorKind::Fail);
        println!("{error:?}");
        println!("{error}");
        println!("{}", error.to_string());
    }
}