use rand::Rng;
use speck_cipher::{speck_cbc_decrypt, speck_cbc_encrypt};

fn main() {
    // Create a new instance of the Speck128/256 cipher with a random key and IV
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen();
    let iv: [u8; 16] = rng.gen();

    println!("Speck128/256 Cipher in CBC mode");
    println!("Key: {:?}", key);
    println!("IV: {:?}", iv);

    // Create a longer string of plaintext
    let plaintext = "This is a longer string of plaintext that will be encrypted using the Speck128/256 cipher in CBC mode. It is long enough to span multiple blocks, which will demonstrate the block mode functionality.";
    println!("Plaintext: {}\n", plaintext);

    println!("Encrypting plaintext...");
    // Encrypt the plaintext
    let ciphertext = speck_cbc_encrypt(&key, &iv, plaintext.as_bytes());
    println!("Ciphertext: {:?}\n", ciphertext);

    println!("Decrypting ciphertext...");
    // Decrypt the ciphertext
    let decrypted_data = speck_cbc_decrypt(&key, &iv, &ciphertext);
    let decrypted_text = String::from_utf8(decrypted_data).unwrap();
    println!("Decrypted text: {}\n", decrypted_text);

    // Check that the decrypted text matches the original plaintext
    assert_eq!(plaintext, decrypted_text);
    println!("Decryption successful: original plaintext matches decrypted text");
}
