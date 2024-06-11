use rand::Rng;
use speck_cipher::speck_cbc_encrypt;
use speck_cipher::test_utils::{
    hamming_distance, MNEMONIC_DATA, MNEMONIC_DATA_MODIFIED_BACK, MNEMONIC_DATA_MODIFIED_FRONT,
};
use std::fs::File;
use std::io::Write;

// Test avalanche effect on two sets of data
// Using the same key and iv
fn benchmark_avalanche_effect_on_data(data: &[&str], modified_data: &[&str]) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut avalanche_effects = vec![0.0; data.len()];

    for (i, (orig, modif)) in data.iter().zip(modified_data.iter()).enumerate() {
        let plaintext = orig.as_bytes();
        let modified_plaintext = modif.as_bytes();
        let key: [u8; 32] = rng.gen();
        let iv: [u8; 16] = rng.gen();

        let ciphertext = speck_cbc_encrypt(&key, &iv, plaintext);
        let modified_ciphertext = speck_cbc_encrypt(&key, &iv, modified_plaintext);
        let distance = hamming_distance(&ciphertext, &modified_ciphertext);
        avalanche_effects[i] = distance as f64 / (ciphertext.len() * 8) as f64;
    }

    avalanche_effects
}

fn main() {
    // Perform avalanche effect test on the two sets of data
    let avalanche_backmod =
        benchmark_avalanche_effect_on_data(&MNEMONIC_DATA, &MNEMONIC_DATA_MODIFIED_BACK);
    let avalanche_frontmod =
        benchmark_avalanche_effect_on_data(&MNEMONIC_DATA, &MNEMONIC_DATA_MODIFIED_FRONT);

    // Write the avalanche effect data to a file
    let dir_path = "data/avalanche/test_2";
    let mut data_file = File::create(format!("{}/data.txt", dir_path)).unwrap();

    for ((index, effect_back), effect_front) in avalanche_backmod
        .iter()
        .enumerate()
        .zip(&avalanche_frontmod)
    {
        writeln!(data_file, "{} {} {}", index, effect_back, effect_front).unwrap();
    }

    println!("Avalanche effect data has been written to {}", dir_path);
}
