use crypto_wallet::bip39::mnemonic::*;
use rand::{thread_rng, Rng};
use speck_cipher::speck_cbc_encrypt;
use std::collections::HashMap;

const NUM_SAMPLES: usize = 1_000_000;

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

// fn main() {
//     // Generate a random key and IV
//     let key = generate_random_key();
//     let iv = generate_random_iv();

//     // Generate random plaintext samples
//     let plaintexts: Vec<Vec<u8>> = (0..NUM_SAMPLES)
//         .map(|_| generate_random_mnemonic_phrase().as_bytes().to_vec())
//         .collect();

//     // Encrypt the plaintexts using Speck cipher in CBC mode
//     let ciphertexts: Vec<Vec<u8>> = plaintexts
//         .iter()
//         .map(|plaintext| speck_cbc_encrypt(&key, &iv, plaintext))
//         .collect();

//     // Concatenate the ciphertexts into a single byte array
//     let encrypted_data: Vec<u8> = ciphertexts.into_iter().flatten().collect();

//     // Calculate the entropy of the encrypted data
//     let entropy = calculate_entropy(&encrypted_data);

//     println!("Entropy: {:.4} bits per byte", entropy);
// }

fn main() {
    // Generate a random key and IV
    let key = generate_random_key();
    let iv = generate_random_iv();

    // Generate random plaintext samples
    let plaintexts: Vec<Vec<u8>> = (0..NUM_SAMPLES)
        .map(|_| generate_random_mnemonic_phrase().as_bytes().to_vec())
        .collect();

    // Encrypt the plaintexts using Speck cipher in CBC mode and calculate the entropy
    for plaintext in &plaintexts {
        let ciphertext = speck_cbc_encrypt(&key, &iv, plaintext);

        // Calculate the entropy of the plaintext and ciphertext
        let plaintext_entropy = calculate_entropy(plaintext);
        let ciphertext_entropy = calculate_entropy(&ciphertext);

        println!("Plaintext entropy: {:.4} bits per byte", plaintext_entropy);
        println!(
            "Ciphertext entropy: {:.4} bits per byte",
            ciphertext_entropy
        );
    }
}
