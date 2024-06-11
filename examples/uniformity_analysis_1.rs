use crypto_wallet::bip39::mnemonic::*;
use rand::{thread_rng, Rng};
use speck_cipher::{
    speck_cbc_encrypt,
    test_utils::{export_to_file, perform_uniformity_analysis},
};
use statrs::distribution::{ChiSquared, ContinuousCDF};

// ===============================
// Single Key, Multiple Samples
// ===============================

const NUM_SAMPLES: usize = 5000;
const NUM_BINS: usize = 256;

fn main() {
    // Generate a random key and IV
    let mut rng = thread_rng();
    let key: [u8; 32] = rng.gen();

    // Generate random plaintext samples
    let plaintexts: Vec<Vec<u8>> = (0..NUM_SAMPLES)
        .map(|_| {
            Mnemonic::random(24)
                .unwrap()
                .to_phrase()
                .as_bytes()
                .to_vec()
        })
        .collect();

    let chi_squares: Vec<f64> = plaintexts
        .iter()
        .map(|plaintext| {
            let iv = rng.gen();
            let ciphertext = speck_cbc_encrypt(&key, &iv, plaintext);
            perform_uniformity_analysis(ciphertext.as_slice(), NUM_BINS)
        })
        .collect();

    let avg_chi_square = chi_squares.iter().sum::<f64>() / NUM_SAMPLES as f64;

    // Calculate the critical value for the Chi-square test
    let degrees_of_freedom = NUM_BINS - 1;
    let significance_level = 0.05;
    let critical_value = ChiSquared::new(degrees_of_freedom as f64)
        .unwrap()
        .inverse_cdf(1.0 - significance_level);
    let percentage_uniform = chi_squares
        .iter()
        .filter(|&chi_square| *chi_square < critical_value)
        .count() as f64
        / NUM_SAMPLES as f64
        * 100.0;

    // Print the results
    println!("Chi-square statistic: {}", avg_chi_square);
    println!("Critical value: {}", critical_value);
    println!(
        "Percentage of samples with uniformly distributed ciphertext: {:.2}%",
        percentage_uniform
    );

    export_to_file(&chi_squares, "data/uniformity/test_1/data.txt");
    export_to_file(
        &[critical_value],
        "data/uniformity/test_1/critical_value.txt",
    );

    println!("Uniformity data has been written to data/uniformity/test_1")
}
