[![License](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Travis CI](https://travis-ci.org/KizzyCode/crypto_api_chachapoly.svg?branch=master)](https://travis-ci.org/KizzyCode/crypto_api_chachapoly)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/crypto_api_chachapoly?svg=true)](https://ci.appveyor.com/project/KizzyCode/crypto-api-chachapoly)

# crypto_api_osrandom
Welcome to `crypto_api_osrandom` 🎉


## About
This crate implements access to your operating system's cryptographically secure random number
generator via [`crypto_api`](https://github.com/KizzyCode/crypto_api).


## APIs used
The following native APIs are used:
 - macOS/iOS: `SecRandomCopyBytes` from the security framework
 - FreeBSD/OpenBSD/NetBSD: `arc4random_buf` (which does not use ARC4 anymore but a secure PRF like
   [ChaCha20](https://cr.yp.to/chacha.html))
 - Windows: `BCryptGenRandom` with `BCRYPT_USE_SYSTEM_PREFERRED_RNG`
 - Linux: `getrandom` for glibc versions >= 2.25 or `/dev/urandom` for ancient distributions


## Dependencies
Because this code implements the [`crypto_api`](https://github.com/KizzyCode/crypto_api), it depends
on the `crypto_api`-crate. Otherwise, it's dependency less.