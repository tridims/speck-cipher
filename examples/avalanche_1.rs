use rand::Rng;
use speck_cipher::{speck_cbc_encrypt, test_utils::hamming_distance};
use std::fs::File;
use std::io::Write;

// Testing Avalanche Effect on Flipping Bits at random positions in the plaintext and key
// each result of the bit flipped ciphertext is compared to the original ciphertext

// Testing the avalanche effect when flipping bits in the plaintext
fn benchmark_avalanche_effect_on_plaintext(
    num_iterations: usize,
    max_flipped_bits: usize,
) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut avalanche_effects = vec![0.0; max_flipped_bits + 1];

    for _ in 0..num_iterations {
        let plaintext: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        let key: [u8; 32] = rng.gen();
        let iv: [u8; 16] = rng.gen();

        let ciphertext = speck_cbc_encrypt(&key, &iv, &plaintext);

        for flipped_bits in 1..=max_flipped_bits {
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
        .skip(1)
        .map(|&x| x / num_iterations as f64)
        .collect()
}

fn benchmark_avalanche_effect_on_key(num_iterations: usize, max_flipped_bits: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut avalanche_effects = vec![0.0; max_flipped_bits + 1];

    for _ in 0..num_iterations {
        let plaintext: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        let key: [u8; 32] = rng.gen();
        let iv: [u8; 16] = rng.gen();

        let ciphertext = speck_cbc_encrypt(&key, &iv, &plaintext);

        for flipped_bits in 1..=max_flipped_bits {
            let mut modified_key = key;

            for _ in 0..flipped_bits {
                let bit_index = rng.gen_range(0..modified_key.len() * 8);
                let byte_index = bit_index / 8;
                let bit_mask = 1 << (bit_index % 8);

                modified_key[byte_index] ^= bit_mask;
            }

            let modified_ciphertext = speck_cbc_encrypt(&modified_key, &iv, &plaintext);
            let distance = hamming_distance(&ciphertext, &modified_ciphertext);
            avalanche_effects[flipped_bits] += distance as f64 / (ciphertext.len() * 8) as f64;
        }
    }

    avalanche_effects
        .iter()
        .skip(1)
        .map(|&x| x / num_iterations as f64)
        .collect()
}

fn main() {
    let num_iterations = 10000;
    let max_flipped_bits = 10;
    let avalanche_effects_on_plaintext =
        benchmark_avalanche_effect_on_plaintext(num_iterations, max_flipped_bits);
    let avalanche_effects_on_key =
        benchmark_avalanche_effect_on_key(num_iterations, max_flipped_bits);

    let mut data_file_pl = File::create("data/avalanche/test_1/data_plaintext.txt").unwrap();
    let mut data_file_key = File::create("data/avalanche/test_1/data_key.txt").unwrap();

    for (flipped_bits, effect) in avalanche_effects_on_plaintext.iter().enumerate() {
        writeln!(data_file_pl, "{} {}", flipped_bits, effect).unwrap();
    }
    for (flipped_bits, effect) in avalanche_effects_on_key.iter().enumerate() {
        writeln!(data_file_key, "{} {}", flipped_bits, effect).unwrap();
    }

    println!("Avalanche effect data has been written to data/avalanche/test_1");
}
