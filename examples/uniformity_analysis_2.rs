use crypto_wallet::bip39::mnemonic::*;
use rand::{thread_rng, Rng};
use speck_cipher::test_utils::export_to_file;
use speck_cipher::{speck_cbc_encrypt, test_utils::perform_uniformity_analysis};
use statrs::distribution::{ChiSquared, ContinuousCDF};

// ===============================
// Multiple samples, unique key and iv for each sample
// ===============================

const NUM_SAMPLES: usize = 5000;
const NUM_BINS: usize = 256;

fn main() {
    let mut rng = thread_rng();

    let mut data = Vec::new();
    let mut chi_square_sum = 0.0;

    for _ in 0..NUM_SAMPLES {
        let key: [u8; 32] = rng.gen();
        let iv: [u8; 16] = rng.gen();
        let mnemonic = Mnemonic::random(24).unwrap().to_phrase();
        let ciphertext = speck_cbc_encrypt(&key, &iv, mnemonic.as_bytes());

        let chi_square = perform_uniformity_analysis(&ciphertext, NUM_BINS);
        data.push(chi_square);
        chi_square_sum += chi_square;
    }

    let avg_chi_square = chi_square_sum / NUM_SAMPLES as f64;

    let degrees_of_freedom = NUM_BINS - 1;
    let significance_level = 0.05;
    let critical_value = ChiSquared::new(degrees_of_freedom as f64)
        .unwrap()
        .inverse_cdf(1.0 - significance_level);

    let uniform_count = data
        .iter()
        .filter(|&&chi_square| chi_square < critical_value)
        .count();
    let uniform_percentage = (uniform_count as f64 / NUM_SAMPLES as f64) * 100.0;
    println!(
        "Percentage of samples with uniformly distributed ciphertext: {:.2}%",
        uniform_percentage
    );
    println!("Average Chi-square statistic: {}", avg_chi_square);
    println!("Critical value: {}", critical_value);

    // export
    export_to_file(&data, "data/uniformity/test_2/data.txt");
    export_to_file(
        &[critical_value],
        "data/uniformity/test_2/critical_value.txt",
    );

    println!("Uniformity data has been written to data/uniformity/test_2")
}
