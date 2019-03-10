/// Links the security framework and returns the implementation
fn macos_ios_secrandomcopybytes() -> Option<&'static str> {
	println!("cargo:rustc-link-lib=framework=Security");
	Some("USE_SECRANDOMCOPYBYTES")
}


/// Checks if the current glibc version supports `getrandom`
#[cfg(target_os = "linux")]
fn linux_check_getrandom() -> Option<&'static str> {
	// Get libc version
	use std::{ u32, ffi::CStr, os::raw::c_char, str::FromStr };
	extern "C" {
		// const char *gnu_get_libc_version(void);
		fn gnu_get_libc_version() -> *const c_char;
	}
	let v: Vec<u8> = unsafe{ CStr::from_ptr(gnu_get_libc_version()) }.to_str().unwrap()
		.split(".").map(|s| u8::from_str(s).unwrap()).collect();
	
	// Validate version
	match (v[0], v[1]) {
		(2...255, 25...255) => Some("USE_GETRANDOM"),
		_ => Some("USE_DEV_URANDOM")
	}
}


fn main() {
	// Determine which secure random number generator to use
	#[allow(unused_assignments)]
	let mut secure_random = None;
	
	#[cfg(target_os = "macos")] { secure_random = macos_ios_secrandomcopybytes() }
	#[cfg(target_os = "ios")] { secure_random = macos_ios_secrandomcopybytes() }
	#[cfg(target_os = "freebsd")] { secure_random = Some("USE_ARC4RANDOM") }
	#[cfg(target_os = "openbsd")] { secure_random = Some("USE_ARC4RANDOM") }
	#[cfg(target_os = "netbsd")] { secure_random = Some("USE_ARC4RANDOM") }
	#[cfg(target_os = "windows")] { secure_random = Some("USE_CRYPTGENRANDOM") }
	#[cfg(target_os = "linux")] { secure_random = linux_check_getrandom() }
	
	// Check if we have a secure random number generator
	let secure_random = match secure_random {
		Some(secure_random) => secure_random,
		None => panic!("No secure random number generator known for your target platform")
	};
	
	// Compile and link the library
	cc::Build::new()
		.file("helpers/helpers.c")
		.define(secure_random, None)
		.warnings_into_errors(true)
		.compile("helpers");
	println!("cargo:rustc-link-lib=static=helpers");
}