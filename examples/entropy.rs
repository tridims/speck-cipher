use crypto_wallet::bip39::mnemonic::*;
use rand::{thread_rng, Rng};
use speck_cipher::speck_cbc_encrypt;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// const NUM_SAMPLES: usize = 1_000;
const NUM_SAMPLES: usize = 500;

fn calculate_entropy(data: &[u8]) -> f64 {
    let mut frequency_map = HashMap::new();
    let data_len = data.len() as f64;

    // Calculate the frequency of each byte value
    for &byte in data {
        *frequency_map.entry(byte).or_insert(0) += 1;
    }

    // Calculate the entropy
    let mut entropy = 0.0;
    for &count in frequency_map.values() {
        let probability = count as f64 / data_len;
        entropy -= probability * probability.log2();
    }

    entropy
}

fn main() {
    // Generate a random key and IV
    let mut rng = thread_rng();
    let key: [u8; 32] = rng.gen();
    let iv: [u8; 16] = rng.gen();

    // Generate random plaintext samples
    let plaintexts: Vec<Vec<u8>> = (0..NUM_SAMPLES)
        .map(|_| {
            Mnemonic::random(12)
                .unwrap()
                .to_phrase()
                .as_bytes()
                .to_vec()
        })
        .collect();

    let mut file = File::create("data/entropy/data.txt").expect("Unable to create file");

    // Encrypt the plaintexts using Speck cipher in CBC mode and calculate the entropy
    for plaintext in &plaintexts {
        let ciphertext = speck_cbc_encrypt(&key, &iv, plaintext);

        // Calculate the entropy of the plaintext and ciphertext
        let plaintext_entropy = calculate_entropy(plaintext);
        let ciphertext_entropy = calculate_entropy(&ciphertext);

        writeln!(file, "{:.4} {:.4}", plaintext_entropy, ciphertext_entropy)
            .expect("Unable to write to file");
    }

    println!("Entropy data has been written to data/entropy");
}
