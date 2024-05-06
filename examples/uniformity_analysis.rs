use crypto_wallet::bip39::mnemonic::*;
use rand::{thread_rng, Rng};
use speck_cipher::speck_cbc_encrypt;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use std::fs::File;
use std::io::Write;

const NUM_SAMPLES: usize = 10000;
const NUM_BINS: usize = 256;
const NUM_KEYS: usize = 100;

fn main() {
    let iv = generate_random_iv();

    let mut data = Vec::new();

    for _ in 0..NUM_KEYS {
        let key = generate_random_key();
        let mut chi_square_sum = 0.0;

        for _ in 0..NUM_SAMPLES {
            let mnemonic_phrase = generate_random_mnemonic_phrase();
            let ciphertext = speck_cbc_encrypt(&key, &iv, mnemonic_phrase.as_bytes());
            let chi_square = perform_uniformity_analysis(&ciphertext);
            chi_square_sum += chi_square;
        }

        let avg_chi_square = chi_square_sum / NUM_SAMPLES as f64;
        data.push(avg_chi_square);
    }

    let degrees_of_freedom = NUM_BINS - 1;
    let significance_level = 0.05;
    let critical_value = ChiSquared::new(degrees_of_freedom as f64)
        .unwrap()
        .inverse_cdf(1.0 - significance_level);

    let uniform_count = data
        .iter()
        .filter(|&&chi_square| chi_square < critical_value)
        .count();
    let uniform_percentage = (uniform_count as f64 / NUM_KEYS as f64) * 100.0;

    println!(
        "Percentage of keys with uniformly distributed ciphertext: {:.2}%",
        uniform_percentage
    );

    export_data_to_file(&data, "data/uniformity_data.txt");
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

fn perform_uniformity_analysis(ciphertext: &[u8]) -> f64 {
    let mut observed_frequencies = vec![0; NUM_BINS];

    for &byte in ciphertext {
        observed_frequencies[byte as usize] += 1;
    }

    let expected_frequency = ciphertext.len() as f64 / NUM_BINS as f64;
    let chi_square: f64 = observed_frequencies
        .iter()
        .map(|&observed| {
            let diff = observed as f64 - expected_frequency;
            diff * diff / expected_frequency
        })
        .sum();

    chi_square
}

fn export_data_to_file(data: &[f64], filename: &str) {
    let mut file = File::create(filename).expect("Failed to create file");
    for &value in data {
        writeln!(file, "{}", value).expect("Failed to write to file");
    }
}
