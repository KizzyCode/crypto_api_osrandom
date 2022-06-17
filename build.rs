use cc::Build;
use std::env::consts::OS;

/// Checks if the current glibc version supports `getrandom`
fn linux_has_getrandom() -> bool {
    #[cfg(target_os = "linux")]
    {
        // Get libc version
        use std::{ffi::CStr, os::raw::c_char, str::FromStr};

        // glibc bindings
        extern "C" {
            // const char *gnu_get_libc_version(void);
            fn gnu_get_libc_version() -> *const c_char;
        }

        // Parse the version
        let version_ptr = unsafe { gnu_get_libc_version() };
        let version_str = unsafe { CStr::from_ptr(version_ptr) }
            .to_str()
            .expect("glibc version is not a valid string?!");
        let version: Vec<_> = version_str
            .split('.')
            .map(|s| u8::from_str(s).expect("Invalid glibc version?!"))
            .collect();

        // Ensure glibc-version is >= 2.2
        match version.as_slice() {
            [2..=255, 25..=255, ..] => return true,
            _ => return false,
        }
    }

    // Otherwise, return false
    #[allow(unreachable_code)]
    false
}

/// Select the random number generator
fn select_random() -> &'static str {
    match OS {
        "macos" | "ios" => {
            println!("cargo:rustc-link-lib=framework=Security");
            "librandom/secrandomcopybytes.c"
        }
        "freebsd" | "openbsd" | "netbsd" => "librandom/arc4random.c",
        "windows" => "librandom/cryptgenrandom.c",
        "linux" if linux_has_getrandom() => "librandom/getrandom.c",
        "linux" => "librandom/urandom.c",
        os => panic!("Unsupported target OS: {os}"),
    }
}

fn main() {
    // Compile and link the library
    Build::new()
        .file(select_random())
        .warnings_into_errors(true)
        .compile("helpers");
}
