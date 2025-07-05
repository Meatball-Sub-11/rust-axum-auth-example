// src/sha2_manual.rs
// A manual implementation of SHA-256 for educational purposes.
// DO NOT USE IN PRODUCTION.

// --- Core SHA-256 Functions ---

// Performs a right-rotate operation on a 32-bit integer.
fn rotr(x: u32, n: u8) -> u32 {
    (x >> n) | (x << (32 - n))
}

// SHA-256 logical function `ch` (Choose)
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

// SHA-256 logical function `maj` (Majority)
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

// SHA-256 logical function `sigma0`
fn sigma0(x: u32) -> u32 {
    rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
}

// SHA-256 logical function `sigma1`
fn sigma1(x: u32) -> u32 {
    rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
}

// SHA-256 logical function `gamma0`
fn gamma0(x: u32) -> u32 {
    rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3)
}

// SHA-256 logical function `gamma1`
fn gamma1(x: u32) -> u32 {
    rotr(x, 17) ^ rotr(x, 19) ^ (x >> 10)
}

// --- Main Hashing Function ---

/// Hashes an input string using a manual SHA-256 implementation.
pub fn digest(input: &str) -> String {
    let input_bytes = input.as_bytes();

    // --- SHA-256 Constants ---
    // K constants (first 32 bits of the fractional parts of the cube roots of the first 64 primes)
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    // Initial hash values (H constants)
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    // --- Padding ---
    let original_len_bits = (input_bytes.len() as u64) * 8;
    let mut padded_data = input_bytes.to_vec();
    padded_data.push(0x80); // Append '1' bit

    // Append '0' bits until length is 64 bytes short of a multiple of 64
    while (padded_data.len() % 64) != 56 {
        padded_data.push(0x00);
    }

    // Append original length in bits as a 64-bit big-endian integer
    padded_data.extend_from_slice(&original_len_bits.to_be_bytes());

    // --- Process Chunks ---
    for chunk in padded_data.chunks_exact(64) {
        let mut w = [0u32; 64];
        // Copy chunk into first 16 words of message schedule
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }

        // Extend the first 16 words into the remaining 48 words
        for i in 16..64 {
            w[i] = gamma1(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(gamma0(w[i - 15]))
                .wrapping_add(w[i - 16]);
        }

        // --- Compression Loop ---
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut h_ = h[7];

        for i in 0..64 {
            let t1 = h_
                .wrapping_add(sigma1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let t2 = sigma0(a).wrapping_add(maj(a, b, c));
            h_ = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        // Update hash values
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(h_);
    }

    // --- Final Hash ---
    let mut final_hash = String::new();
    for val in h {
        final_hash.push_str(&format!("{val:08x}"));
    }
    final_hash
}

// --- Unit Test Module ---
#[cfg(test)]
mod tests {
    // Import the `digest` function from the file above
    use super::digest;

    #[test]
    fn test_digest_with_known_value() {
        // A known input string
        let input = "hello world";

        // The known, correct SHA-256 hash for "hello world"
        let expected_hash = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";

        // Run our digest function
        let actual_hash = digest(input);

        // Assert that the actual result matches the expected result
        assert_eq!(actual_hash, expected_hash);
    }
}
