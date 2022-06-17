// Include `stdlib.h` for `size_t`
#include <stdlib.h>
#include <stdint.h>


/// Get a series of cryptographically secure random bytes
///
/// \param buf The buffer to write the bytes to
/// \param len The amount of bytes to write
/// \return 0 on success or 1 in case of an error
uint8_t crypto_api_osrandom_native(uint8_t* buf, size_t len) {
    arc4random_buf(buf, len);
    return 0;
}
