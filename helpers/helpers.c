// Include `stdlib.h` for `size_t`
#include <stdlib.h>
#include <stdint.h>


// Function-dependent includes
#if defined(USE_GETRANDOM)
	#include <sys/random.h>
#elif defined(USE_ARC4RANDOM)
	// `stdlib.h` is already included
#elif defined(USE_SECRANDOMCOPYBYTES)
	#include <Security/SecRandom.h>
#elif defined(USE_BCRYPTGENRANDOM)
	#include <bcrypt.h>
#elif defined(USE_DEV_URANDOM)
	#include <stdio.h>
#endif


/// Get a series of cryptographically secure random bytes
///
/// \param buf The buffer to write the bytes to
/// \param len The amount of bytes to write
/// \return 0 on success or 1 in case of an error
uint8_t crypto_api_osrandom_secrandom(uint8_t* buf, size_t len) {
	#if defined(USE_GETRANDOM)
		return getrandom(buf, len, 0) == -1 ? 1 : 0
	#elif defined(USE_ARC4RANDOM)
		arc4random_buf(buf, len);
		return 0;
	#elif defined(USE_SECRANDOMCOPYBYTES)
		return SecRandomCopyBytes(kSecRandomDefault, len, buf) == errSecSuccess ? 0 : 1;
	#elif defined(USE_BCRYPTGENRANDOM)
		NTSTATUS r = BCryptGenRandom(NULL, (PUCHAR)buf, (ULONG)len, BCRYPT_USE_SYSTEM_PREFERRED_RNG);
		return r == STATUS_SUCCESS ? 0 : 1
	#elif defined(USE_DEV_URANDOM)
		FILE* urandom = fopen("/dev/urandom", "r");
		if (urandom == NULL) return 1;
		
		while (len) {
			size_t bytes_read = fread(buf, 1, len, urandom);
			if (bytes_read == 0) {
				fclose(urandom);
				return 1;
			}
			
			buf += bytes_read;
			len -= bytes_read;
		}
		
		fclose(urandom);
		return 0;
	#else
		#error "No secure random number generator selected (this is a build-system error)"
	#endif
}