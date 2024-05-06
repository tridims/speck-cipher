use std::mem::size_of;

#[derive(Clone)]
pub struct Speck128_256 {
    round_keys: [u64; 34],
}

impl Speck128_256 {
    // Perform a right rotation on a 64-bit unsigned integer by a specified number of positions
    #[inline]
    fn rotate_right(x: u64, pos: u64) -> u64 {
        (x >> pos) | (x << (64 - pos))
    }

    // Perform a left rotation on a 64-bit unsigned integer by a specified number of positions
    #[inline]
    fn rotate_left(x: u64, pos: u64) -> u64 {
        (x << pos) | (x >> (64 - pos))
    }

    // Perform the Speck round function on a pair of 64-bit unsigned integers and a round key
    #[inline]
    fn round_function(key: u64, mut x: u64, mut y: u64) -> (u64, u64) {
        // Rotate x to the right by 8 positions
        x = Self::rotate_right(x, 8);
        // Add x and y modulo 2^64
        x = x.wrapping_add(y);
        // XOR x with the round key
        x ^= key;
        // Rotate y to the left by 3 positions
        y = Self::rotate_left(y, 3);
        // XOR y with the updated value of x
        y ^= x;
        // Return the updated values of x and y
        (x, y)
    }

    // Perform the inverse Speck round function on a pair of 64-bit unsigned integers and a round key
    #[inline]
    fn inverse_round_function(key: u64, mut x: u64, mut y: u64) -> (u64, u64) {
        // XOR y with x
        y ^= x;
        // Rotate y to the right by 3 positions
        y = Self::rotate_right(y, 3);
        // XOR x with the round key
        x ^= key;
        // Subtract y from x modulo 2^64
        x = x.wrapping_sub(y);
        // Rotate x to the left by 8 positions
        x = Self::rotate_left(x, 8);
        // Return the updated values of x and y
        (x, y)
    }

    // Initialize a new instance of the Speck128/256 cipher with the given key
    pub fn new(key: &[u8; 32]) -> Self {
        let mut round_keys = [0; 34];
        let mut l = [0; 36];

        // Initialize the first round key with the last 64 bits of the key
        round_keys[0] = from_be_bytes(&key[24..32]);

        // Initialize the l array with the remaining 192 bits of the key
        for i in 0..3 {
            l[i] = from_be_bytes(&key[(2 - i) * 8..(3 - i) * 8]);
        }

        // Generate the remaining round keys using the key schedule
        for i in 0..33 {
            let (l_next, k_next) = Self::round_function(i as u64, l[i], round_keys[i]);
            l[i + 3] = l_next;
            round_keys[i + 1] = k_next;
        }

        // Return a new instance of the Speck128/256 cipher with the computed round keys
        Self { round_keys }
    }

    // Encrypt a 128-bit block using the Speck128/256 cipher
    pub fn encrypt(&self, block: &mut [u8; 16]) {
        // Split the block into two 64-bit halves
        let mut x = from_be_bytes(&block[0..8]);
        let mut y = from_be_bytes(&block[8..]);

        // Apply the Speck round function 34 times, using the precomputed round keys
        for i in 0..34 {
            let (x_next, y_next) = Self::round_function(self.round_keys[i], x, y);
            x = x_next;
            y = y_next;
        }

        // Write the encrypted halves back to the block
        block[0..8].copy_from_slice(&to_be_bytes(x));
        block[8..].copy_from_slice(&to_be_bytes(y));
    }

    // Decrypt a 128-bit block using the Speck128/256 cipher
    pub fn decrypt(&self, block: &mut [u8; 16]) {
        // Split the block into two 64-bit halves
        let mut x = from_be_bytes(&block[0..8]);
        let mut y = from_be_bytes(&block[8..]);

        // Apply the inverse Speck round function 34 times, using the precomputed round keys in reverse order
        for i in (0..34).rev() {
            let (x_prev, y_prev) = Self::inverse_round_function(self.round_keys[i], x, y);
            x = x_prev;
            y = y_prev;
        }

        // Write the decrypted halves back to the block in big-endian order
        block[0..8].copy_from_slice(&to_be_bytes(x));
        block[8..].copy_from_slice(&to_be_bytes(y));
    }
}

// Convert a byte slice into a 64-bit unsigned integer in big-endian order
#[inline]
fn from_be_bytes(bytes: &[u8]) -> u64 {
    let mut tmp = [0u8; size_of::<u64>()];
    let offset = size_of::<u64>() - 8;
    tmp[offset..].copy_from_slice(bytes);
    u64::from_be_bytes(tmp)
}

// Convert a 64-bit unsigned integer into a byte array in big-endian order
#[inline]
fn to_be_bytes(word: u64) -> [u8; 8] {
    let tmp = word.to_be_bytes();
    let offset = size_of::<u64>() - 8;
    tmp[offset..].try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_left() {
        let x: u64 = 1u64;
        let pos: u64 = 1;
        let result = Speck128_256::rotate_left(x, pos);
        let expected: u64 = 2u64;
        assert_eq!(result, expected, "Failed to rotate left correctly");
    }

    #[test]
    fn test_rotate_right() {
        let x: u64 = 2u64;
        let pos: u64 = 1;
        let result = Speck128_256::rotate_right(x, pos);
        let expected: u64 = 1u64;
        assert_eq!(result, expected, "Failed to rotate right correctly");
    }
}
