#[cfg(not(feature = "std"))]
pub use thiserror_no_std::Error as ThisError;
#[cfg(feature = "std")]
pub use thiserror::Error as ThisError;
use crate::std::fmt;


pub type JResult<O, E = Error> = Result<O, E>;


#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub position: usize,
    pub code: ErrorKind,
}


impl Error {
    pub fn new(position: usize, kind: ErrorKind) -> Self {
        Self { position, code: kind }
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {     
           write!(f, "[ERROR]: {}, position: {}", self.code, self.position)
    }
}


pub trait ParseError {
    fn from_error_kind(position: usize, kind: ErrorKind) -> Self;
}


impl ParseError for Error {
    fn from_error_kind(position: usize, kind: ErrorKind) -> Self {
        Error::new(position, kind)
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
pub fn make_error<E: ParseError>(position: usize, kind: ErrorKind) -> E {
    E::from_error_kind(position, kind)
}