#![doc = include_str!("../README.md")]

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

/// An interface to the operating system's secure random number generator
pub struct OsRandom;
impl OsRandom {
    /// Creates a new `OsRandom` instance
    pub const fn new() -> OsRandom {
        Self
    }

    /// Fills the given slice with crpytographically secure random bytes
    pub fn to_slice(&self, slice: &mut [u8]) -> Result<(), Error> {
        // The API of the bridge library
        extern "C" {
            // uint8_t crypto_api_osrandom(uint8_t* buf, size_t len);
            fn crypto_api_osrandom_native(buf: *mut u8, len: usize) -> u8;
        }

        // Call the native implementation
        let result = unsafe { crypto_api_osrandom_native(slice.as_mut_ptr(), slice.len()) };
        match result {
            0 => Ok(()),
            _ => Err(Error),
        }
    }
    /// Creates a `LEN`-sized array filled with crpytographically secure random bytes
    pub fn to_array<const LEN: usize>(&self) -> Result<[u8; LEN], Error> {
        let mut array = [0; LEN];
        self.to_slice(&mut array)?;
        Ok(array)
    }
    /// Creates a `len`-sized vec filled with crpytographically secure random bytes
    pub fn to_vec(&self, len: usize) -> Result<Vec<u8>, Error> {
        let mut vec = vec![0; len];
        self.to_slice(&mut vec)?;
        Ok(vec)
    }
}
#[cfg(feature = "crypto_api")]
impl OsRandom {
    /// Creates a `SecureRng` instance with the OS' RNG as underlying implementation
    pub fn secure_rng() -> Box<dyn crypto_api::rng::SecureRng> {
        Box::new(Self)
    }
}
#[cfg(feature = "crypto_api")]
impl crypto_api::rng::SecureRng for OsRandom {
    fn random(&mut self, buf: &mut [u8]) -> Result<(), Box<dyn error::Error + 'static>> {
        self.to_slice(buf)?;
        Ok(())
    }
}
