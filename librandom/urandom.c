// Include `stdlib.h` for `size_t`
#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>


/// Get a series of cryptographically secure random bytes
///
/// \param buf The buffer to write the bytes to
/// \param len The amount of bytes to write
/// \return 0 on success or 1 in case of an error
uint8_t crypto_api_osrandom_native(uint8_t* buf, size_t len) {
    // Open /dev/urandom
    FILE* urandom = fopen("/dev/urandom", "r");
    if (urandom == NULL) {
        return 1;
    }
    
    // Read random bytes
    while (len) {
        // Read random bytes
        size_t bytes_read = fread(buf, 1, len, urandom);
        if (bytes_read == 0) {
            fclose(urandom);
            return 1;
        }
        
        // Adjust buffer and remaining length
        buf += bytes_read;
        len -= bytes_read;
    }
    
    // Close /dev/urandom and return success
    fclose(urandom);
    return 0;
}
