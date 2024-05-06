use crypto_wallet::bip39::mnemonic::*;
use rand::{thread_rng, Rng};
use speck_cipher::speck_cbc_encrypt;
use statrs::distribution::{ChiSquared, ContinuousCDF};

const NUM_SAMPLES: usize = 10000;
const NUM_BINS: usize = 256;

fn main() {
    // Generate a random key and IV
    let key = generate_random_key();
    let iv = generate_random_iv();

    // Generate random plaintext samples
    let plaintexts: Vec<Vec<u8>> = (0..NUM_SAMPLES)
        .map(|_| generate_random_mnemonic_phrase().as_bytes().to_vec())
        .collect();

    // Encrypt the plaintexts using Speck cipher in CBC mode
    let ciphertexts: Vec<Vec<u8>> = plaintexts
        .iter()
        .map(|plaintext| speck_cbc_encrypt(&key, &iv, plaintext))
        .collect();

    // Perform uniformity analysis
    let chi_square = perform_uniformity_analysis(&ciphertexts);

    // Calculate the critical value for the Chi-square test
    let degrees_of_freedom = NUM_BINS - 1;
    let significance_level = 0.05;
    let critical_value = ChiSquared::new(degrees_of_freedom as f64)
        .unwrap()
        .inverse_cdf(1.0 - significance_level);

    // Print the results
    println!("Chi-square statistic: {}", chi_square);
    println!("Critical value: {}", critical_value);

    if chi_square < critical_value {
        println!("The ciphertext is uniformly distributed.");
    } else {
        println!("The ciphertext deviates from a uniform distribution.");
    }
}

fn generate_random_key() -> [u8; 32] {
    let mut rng = thread_rng();
    let mut key = [0u8; 32];
    rng.fill(&mut key);
    key
}

fn generate_random_iv() -> [u8; 16] {
    let mut rng = thread_rng();
    let mut iv = [0u8; 16];
    rng.fill(&mut iv);
    iv
}

fn generate_random_mnemonic_phrase() -> String {
    Mnemonic::random(12).unwrap().to_phrase()
}

fn perform_uniformity_analysis(ciphertexts: &[Vec<u8>]) -> f64 {
    let mut observed_frequencies = vec![0; NUM_BINS];

    for ciphertext in ciphertexts {
        for &byte in ciphertext {
            observed_frequencies[byte as usize] += 1;
        }
    }

    let expected_frequency = (NUM_SAMPLES * ciphertexts[0].len()) as f64 / NUM_BINS as f64;
    let chi_square: f64 = observed_frequencies
        .iter()
        .map(|&observed| {
            let diff = observed as f64 - expected_frequency;
            diff * diff / expected_frequency
        })
        .sum();

    chi_square
}
