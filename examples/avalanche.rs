use rand::Rng;
use speck_cipher::speck_cbc_encrypt;
use std::fs::File;
use std::io::Write;

fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x ^ y).count_ones() as usize)
        .sum()
}

fn benchmark_avalanche_effect(num_iterations: usize, max_flipped_bits: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut avalanche_effects = vec![0.0; max_flipped_bits + 1];

    for _ in 0..num_iterations {
        let plaintext: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        let key: [u8; 32] = rng.gen();
        let iv: [u8; 16] = rng.gen();

        let ciphertext = speck_cbc_encrypt(&key, &iv, &plaintext);

        for flipped_bits in 0..=max_flipped_bits {
            let mut modified_plaintext = plaintext.clone();

            for _ in 0..flipped_bits {
                let bit_index = rng.gen_range(0..plaintext.len() * 8);
                let byte_index = bit_index / 8;
                let bit_mask = 1 << (bit_index % 8);

                modified_plaintext[byte_index] ^= bit_mask;
            }

            let modified_ciphertext = speck_cbc_encrypt(&key, &iv, &modified_plaintext);
            let distance = hamming_distance(&ciphertext, &modified_ciphertext);
            avalanche_effects[flipped_bits] += distance as f64 / (ciphertext.len() * 8) as f64;
        }
    }

    avalanche_effects
        .iter()
        .map(|&x| x / num_iterations as f64)
        .collect()
}

fn main() {
    let num_iterations = 10000;
    let max_flipped_bits = 10;
    let avalanche_effects = benchmark_avalanche_effect(num_iterations, max_flipped_bits);

    let mut data_file = File::create("examples/avalanche_data.txt").unwrap();
    for (flipped_bits, effect) in avalanche_effects.iter().enumerate() {
        writeln!(data_file, "{} {}", flipped_bits, effect).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        let a = [0b10101010, 0b11110000];
        let b = [0b01010101, 0b00001111];
        assert_eq!(hamming_distance(&a, &b), 16);
    }

    #[test]
    fn test_benchmark_avalanche_effect() {
        let num_iterations = 10;
        let max_flipped_bits = 2;
        let avalanche_effects = benchmark_avalanche_effect(num_iterations, max_flipped_bits);

        assert_eq!(avalanche_effects.len(), max_flipped_bits + 1);
        for &effect in &avalanche_effects {
            assert!(effect >= 0.0 && effect <= 1.0);
        }
    }
}
