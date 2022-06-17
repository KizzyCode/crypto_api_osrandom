//! Implements the crate's error type

use std::{
    error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// An error that indicates that something went wrong while calling the OS' RNG
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error;
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Severe RNG error")
    }
}
impl error::Error for Error {
    /* No members to implement */
}
