// Include `stdlib.h` for `size_t`
#include <stdlib.h>
#include <stdint.h>
#include <windows.h>
#include <Wincrypt.h>


/// Get a series of cryptographically secure random bytes
///
/// \param buf The buffer to write the bytes to
/// \param len The amount of bytes to write
/// \return 0 on success or 1 in case of an error
uint8_t crypto_api_osrandom_native(uint8_t* buf, size_t len) {
    // Acquire context
    HCRYPTPROV rng;
    if (CryptAcquireContext(&rng, NULL, NULL, PROV_RSA_FULL, CRYPT_SILENT) == 0) {
        return 1;
    }
    
    // Generate random data
    if (CryptGenRandom(rng, (DWORD)len, (BYTE*)buf) == 0) {
        CryptReleaseContext(rng, 0);
        return 1;
    }

    // Close context and return success
    CryptReleaseContext(rng, 0);
    return 0;
}
