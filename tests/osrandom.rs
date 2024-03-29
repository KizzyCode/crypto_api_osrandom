use crypto_api_osrandom::error::Error;

/// The amount of iterations over all tests
const TEST_ITERATIONS: usize = 8;
/// The data lengths for a test
const TEST_LENS: &[usize] = &[1024 * 1024, 4 * 1024 * 1024, (4 * 1024 * 1024) + 15];

/// A simple function to test the uniform distribution of byte values to catch an all-zero array or similar results of an
/// API misuse
///
/// _Important: This function *does not* attempt to test the randomness of the OS' CSPRNG_
fn test_uniform_dist(buf: &[u8]) {
    // Count the occurrences of each byte
    let mut occurrences = vec![0f64; 256];
    buf.iter().for_each(|b| occurrences[*b as usize] += 1.0);

    // Compute the estimated average occurrences and the according thresholds
    let est_avg = (buf.len() as f64) / 256.0;
    let (est_avg_min, est_avg_max) = (est_avg * 0.9, est_avg * 1.1);

    // Ensure that the occurrence of each byte is within the estimated average
    occurrences.iter().for_each(|d| {
        assert!(*d > est_avg_min, "{d} is not > {est_avg_min}");
        assert!(*d < est_avg_max, "{d} is not < {est_avg_max}");
    });
}

/// Tests the random number generator
#[test]
fn test() -> Result<(), Error> {
    // Perform multiple tests
    for _ in 0..TEST_ITERATIONS {
        for len in TEST_LENS {
            // Test the random data
            let buf = crypto_api_osrandom::to_vec(*len)?;
            test_uniform_dist(&buf)
        }
    }
    Ok(())
}

/// Test `test_uniform_dist` itself
#[test]
#[should_panic]
fn test_test_uniform_dist() {
    // Generate non-uniform data set
    let mut non_uniform_data = vec![0; (4 * 1024 * 1024) + 15];
    non_uniform_data.extend(vec![7; (4 * 1024 * 1024) + 15]);
    non_uniform_data.extend(vec![255; (4 * 1024 * 1024) + 15]);

    // Test function
    test_uniform_dist(&non_uniform_data);
}
