//! Implements the `crypto_api` type
#![cfg(feature = "crypto_api")]

use std::error;

/// An interface to the operating system's secure random number generator
pub struct OsRandom;
impl OsRandom {
    /// Creates a new `OsRandom` instance
    pub const fn new() -> OsRandom {
        Self
    }
}
impl OsRandom {
    /// Creates a `SecureRng` instance with the OS' RNG as underlying implementation
    pub fn secure_rng() -> Box<dyn crypto_api::rng::SecureRng> {
        Box::new(Self)
    }
}
impl crypto_api::rng::SecureRng for OsRandom {
    fn random(&mut self, buf: &mut [u8]) -> Result<(), Box<dyn error::Error + 'static>> {
        crate::to_slice(buf)?;
        Ok(())
    }
}
