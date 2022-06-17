// Include `stdlib.h` for `size_t`
#include <stdlib.h>
#include <stdint.h>
#include <sys/random.h>


/// Get a series of cryptographically secure random bytes
///
/// \param buf The buffer to write the bytes to
/// \param len The amount of bytes to write
/// \return 0 on success or 1 in case of an error
uint8_t crypto_api_osrandom_native(uint8_t* buf, size_t len) {
	// Get the random bytes
	if (getrandom(buf, len, 0) != len) {
		return 1;
	}
	
	// Return success
	return 0;
}
