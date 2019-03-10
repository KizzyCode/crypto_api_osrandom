use crypto_api_osrandom::OsRandom;


const TEST_SIZES: &[usize] = &[1 * 1024 * 1024, 4 * 1024 * 1024, (4 * 1024 * 1024) + 15];
const TEST_ITERATIONS: usize = 8;


/// A simple function to test the uniform distribution of byte values to catch an all-zero array or
/// similar results of an API misuse.
///
/// _Important: This function *does not* attempt to test the randomness of the OS' CSPRNG_
fn test_uniform_dist(buf: &[u8]) {
	let mut dist = vec![0f64; 256];
	buf.iter().for_each(|b| dist[*b as usize] += 1.0);
	
	let est_avg = (buf.len() as f64) / 256.0;
	let (est_avg_min, est_avg_max) = (est_avg * 0.9, est_avg * 1.1);
	dist.iter().for_each(|d| {
		assert!(*d > est_avg_min, "{} is not > {}", *d, est_avg_min);
		assert!(*d < est_avg_max, "{} is not < {}", *d, est_avg_max);
	});
}


#[test]
fn test() {
	for _ in 0..TEST_ITERATIONS {
		for size in TEST_SIZES.iter() {
			let mut buf = vec![0; *size];
			OsRandom::secure_rng().random(&mut buf).unwrap();
			test_uniform_dist(&buf)
		}
	}
}


#[test] #[should_panic]
fn test_test_uniform_dist() {
	test_uniform_dist(&vec![0; (4 * 1024 * 1024) + 15]);
}