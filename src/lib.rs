pub use crypto_api;
use crypto_api::rng::SecureRng;
use std::{
    error::Error,
    fmt::{ Display, Formatter, Result as FmtResult }
};


/// This error indicates that something went wrong while calling the OS' RNG
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct OsRandomError;
impl Display for OsRandomError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
impl Error for OsRandomError {}


/// An interface to the operating system's secure random number generator
pub struct OsRandom;
impl OsRandom {
    /// Creates a `SecureRng` instance with the OS' RNG as underlying implementation
    pub fn secure_rng() -> Box<dyn SecureRng> {
        Box::new(Self)
    }
}
impl SecureRng for OsRandom {
    fn random(&mut self, buf: &mut[u8]) -> Result<(), Box<dyn Error + 'static>> {
        // The API of the bridge library
        extern "C" {
            // uint8_t crypto_api_osrandom_secrandom(uint8_t* buf, size_t len);
            fn crypto_api_osrandom_secrandom(buf: *mut u8, len: usize) -> u8;
        }
        
        // Call the bridge library
        Ok(match unsafe{ crypto_api_osrandom_secrandom(buf.as_mut_ptr(), buf.len()) } {
            0 => (),
            _ => Err(OsRandomError)?
        })
    }
}