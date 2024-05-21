#[derive(Clone)]
pub struct Speck32_64 {
    k: [u16; 22],
}

impl Speck32_64 {
    #[inline]
    fn from_be_bytes(bytes: &[u8]) -> u16 {
        let mut tmp = [0u8; size_of::<u16>()];
        let offset = size_of::<u16>() - 2;
        tmp[offset..].copy_from_slice(bytes);
        <u16>::from_be_bytes(tmp)
    }
    #[inline]
    fn to_be_bytes(word: u16) -> [u8; 2] {
        let tmp = word.to_be_bytes();
        let offset = size_of::<u16>() - 2;
        tmp[offset..].try_into().unwrap()
    }
    #[inline]
    fn rotate_right(x: u16, pos: u16) -> u16 {
        (x >> pos) | (x << (16 - pos))
    }
    #[inline]
    fn rotate_left(x: u16, pos: u16) -> u16 {
        (x << pos) | (x >> (16 - pos))
    }
    #[inline]
    fn round_function(k: u16, mut x: u16, mut y: u16) -> (u16, u16) {
        x = Speck32_64::rotate_right(x, 7);
        x = <u16>::wrapping_add(x, y) & 0xFFFF;
        x = (x ^ k) & 0xFFFF;
        y = Speck32_64::rotate_left(y, 2);
        y = (y ^ x) & 0xFFFF;
        (x, y)
    }
    #[inline]
    fn inverse_round_function(k: u16, mut x: u16, mut y: u16) -> (u16, u16) {
        y = (y ^ x) & 0xFFFF;
        y = Speck32_64::rotate_right(y, 2);
        x = (x ^ k) & 0xFFFF;
        x = <u16>::wrapping_sub(x, y) & 0xFFFF;
        x = Speck32_64::rotate_left(x, 7);
        (x, y)
    }

    pub fn new(key: &[u8; 8]) -> Self {
        let mut k = [0; 22];
        let mut l = [0; 24];
        k[0] = Speck32_64::from_be_bytes(&key[6..8]);
        for i in 0..3 {
            l[i] = Speck32_64::from_be_bytes(&key[(2 - i) * (2)..(3 - i) * (2)]);
        }
        for i in 0..21 {
            let res = Speck32_64::round_function(i.try_into().unwrap(), l[i], k[i]);
            l[i + 3] = res.0;
            k[i + 1] = res.1;
        }
        Self { k }
    }

    pub fn encrypt(&self, block: &mut [u8; 4]) {
        let mut x = Self::from_be_bytes(&b[0..2]);
        let mut y = Self::from_be_bytes(&b[2..4]);

        for i in 0..22 {
            let (x_prev, y_prev) = Self::round_function(self.k[i], x, y);
            x = x_prev;
            y = y_prev;
        }
        block[0..2].copy_from_slice(&Self::to_be_bytes(x));
        block[2..4].copy_from_slice(&Self::to_be_bytes(y));
    }

    pub fn decrypt(self, block: &mut [u8; 16]) {
        let mut x = Self::from_be_bytes(&b[0..(2)]);
        let mut y = Self::from_be_bytes(&b[(2)..2 * (2)]);
        for i in (0..22).rev() {
            let res = Speck32_64::inverse_round_function(cipher.k[i], x, y);
            x = res.0;
            y = res.1;
        }
        block[0..2].copy_from_slice(&Self::to_be_bytes(x));
        block[2..4].copy_from_slice(&Self::to_be_bytes(y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speck32_64() {
        let key = hex!("1918111009080100");
        let plaintext = hex!("6574694c");
        let ciphertext = hex!("a86842f2");

        let cipher = Speck32_64::new(&key);

        let mut block = plaintext;
        cipher.encrypt(&mut block);
        assert_eq!(block, ciphertext);

        cipher.decrypt(&mut block);
        assert_eq!(block, plaintext);
    }
}
