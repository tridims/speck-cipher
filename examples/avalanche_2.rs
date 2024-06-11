use rand::Rng;
use speck_cipher::speck_cbc_encrypt;
use speck_cipher::test_utils::{hamming_distance, MNEMONIC_DATA, MNEMONIC_DATA_MODIFIED_BACK};
use std::fs::File;
use std::io::Write;

fn benchmark_avalanche_effect_on_data(test_data: &[&str], modified_data: &[&str]) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut avalanche_effects = vec![0.0; test_data.len()];

    for (i, (orig, modif)) in test_data.iter().zip(modified_data.iter()).enumerate() {
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
    let avalanche_effects_on_data =
        benchmark_avalanche_effect_on_data(&MNEMONIC_DATA, &MNEMONIC_DATA_MODIFIED_BACK);

    let mut data_file_data = File::create("data/avalanche2/avalanche_2.txt").unwrap();

    for (index, effect) in avalanche_effects_on_data.iter().enumerate() {
        writeln!(data_file_data, "{} {}", index, effect).unwrap();
    }

    println!("Avalanche effect data has been written to data/avalanche2/avalanche_2.txt");
}
