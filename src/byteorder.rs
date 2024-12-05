use crate::std::*;
use jkcenum::JkcEnum;
// use jkcenum::errors::FromStrParseError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, JkcEnum)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ByteOrder {
    #[cfg_attr(feature = "serde", serde(rename="BE", alias=">", alias="0"))]
    #[jenum(rename="BE", alias=">", alias="0")]
    #[default]
    Be,
    #[cfg_attr(feature = "serde", serde(rename="LE", alias="<", alias="1"))]
    #[jenum(rename="LE", alias="<", alias="1")]
    Le,
}
