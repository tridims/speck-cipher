use std::fmt;
use std::mem::size_of;

#[derive(Clone)]
pub struct Speck128_128 {
    round_keys: [u64; 32],
}

impl Speck128_128 {
    #[inline]
    fn from_be_bytes(bytes: &[u8]) -> u64 {
        let mut tmp = [0u8; size_of::<u64>()];
        let offset = size_of::<u64>() - 64 / 8;
        tmp[offset..].copy_from_slice(bytes);
        u64::from_be_bytes(tmp)
    }

    #[inline]
    fn to_be_bytes(word: u64) -> [u8; 64 / 8] {
        let tmp = word.to_be_bytes();
        let offset = size_of::<u64>() - 64 / 8;
        tmp[offset..].try_into().unwrap()
    }

    #[inline]
    fn rotate_right(x: u64, pos: u64) -> u64 {
        (x >> pos) | (x << (64 - pos))
    }

    #[inline]
    fn rotate_left(x: u64, pos: u64) -> u64 {
        (x << pos) | (x >> (64 - pos))
    }

    #[inline]
    fn round_function(key: u64, mut x: u64, mut y: u64) -> (u64, u64) {
        x = Self::rotate_right(x, 8);
        x = x.wrapping_add(y);
        x ^= key;
        y = Self::rotate_left(y, 3);
        y ^= x;
        (x, y)
    }

    #[inline]
    fn inverse_round_function(key: u64, mut x: u64, mut y: u64) -> (u64, u64) {
        y ^= x;
        y = Self::rotate_right(y, 3);
        x ^= key;
        x = x.wrapping_sub(y);
        x = Self::rotate_left(x, 8);
        (x, y)
    }

    pub fn new(key: &[u8; 128 / 8]) -> Self {
        let mut round_keys = [0; 32];
        let mut l = [0; 2 - 1 + 32 - 1];

        round_keys[0] = Self::from_be_bytes(&key[(2 - 1) * (64 / 8)..(2) * (64 / 8)]);
        for i in 0..2 - 1 {
            l[i] = Self::from_be_bytes(&key[(2 - 2 - i) * (64 / 8)..(2 - 1 - i) * (64 / 8)]);
        }

        for i in 0..(32 - 1) {
            let (l_next, k_next) = Self::round_function(i as u64, l[i], round_keys[i]);
            l[i + 2 - 1] = l_next;
            round_keys[i + 1] = k_next;
        }

        Self { round_keys }
    }

    pub fn encrypt(&self, block: &mut [u8; 128 / 8]) {
        let mut x = Self::from_be_bytes(&block[0..(64 / 8)]);
        let mut y = Self::from_be_bytes(&block[(64 / 8)..]);

        for i in 0..32 {
            let (x_next, y_next) = Self::round_function(self.round_keys[i], x, y);
            x = x_next;
            y = y_next;
        }

        block[0..(64 / 8)].copy_from_slice(&Self::to_be_bytes(x));
        block[(64 / 8)..].copy_from_slice(&Self::to_be_bytes(y));
    }

    pub fn decrypt(&self, block: &mut [u8; 128 / 8]) {
        let mut x = Self::from_be_bytes(&block[0..(64 / 8)]);
        let mut y = Self::from_be_bytes(&block[(64 / 8)..]);

        for i in (0..32).rev() {
            let (x_prev, y_prev) = Self::inverse_round_function(self.round_keys[i], x, y);
            x = x_prev;
            y = y_prev;
        }

        block[0..(64 / 8)].copy_from_slice(&Self::to_be_bytes(x));
        block[(64 / 8)..].copy_from_slice(&Self::to_be_bytes(y));
    }
}

impl fmt::Debug for Speck128_128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Speck128_128 { .. }")
    }
}
