#![doc = include_str!("../README.md")]

#[macro_use]
pub mod error;
pub mod crypto_api;

#[cfg(feature = "crypto_api")]
pub use crate::crypto_api::OsRandom;
use crate::error::Error;

/// Fills the given slice with crpytographically secure random bytes
pub fn to_slice(slice: &mut [u8]) -> Result<(), Error> {
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
pub fn to_array<const LEN: usize>() -> Result<[u8; LEN], Error> {
    let mut array = [0; LEN];
    crate::to_slice(&mut array)?;
    Ok(array)
}

/// Creates a `len`-sized vec filled with crpytographically secure random bytes
pub fn to_vec(len: usize) -> Result<Vec<u8>, Error> {
    let mut vec = vec![0; len];
    crate::to_slice(&mut vec)?;
    Ok(vec)
}
