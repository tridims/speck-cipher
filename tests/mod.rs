#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use speck_cipher::{Speck128_128, Speck128_256};

    #[test]
    fn test_speck128_128() {
        let key = hex!("0f0e0d0c0b0a09080706050403020100");
        let plaintext = hex!("6c617669757165207469206564616d20");
        let ciphertext = hex!("a65d9851797832657860fedf5c570d18");

        let cipher = Speck128_128::new(&key);

        let mut block = plaintext;
        cipher.encrypt(&mut block);
        assert_eq!(block, ciphertext);

        cipher.decrypt(&mut block);
        assert_eq!(block, plaintext);
    }

    #[test]
    fn test_speck128_256() {
        let key = hex!("1f1e1d1c1b1a191817161514131211100f0e0d0c0b0a09080706050403020100");
        let plaintext = hex!("65736f6874206e49202e72656e6f6f70");
        let ciphertext = hex!("4109010405c0f53e4eeeb48d9c188f43");

        let cipher = Speck128_256::new(&key);

        let mut block = plaintext;
        cipher.encrypt(&mut block);
        assert_eq!(block, ciphertext);

        cipher.decrypt(&mut block);
        assert_eq!(block, plaintext);
    }

    #[test]
    fn test_cbc_block_mode_with_random_message() {
        let binding = "This is some secret message. Do not reveal. I mean really this is some secret..duh! Why would you want to reveal it anyway."
            .repeat(20);
        let msg = binding.as_bytes();
        println!("length of msg: {}", msg.len());

        let key = b"suuperrsecretkeysuuperrsecretkey";
        let binding = "\x00".repeat(16);
        let iv = binding.as_bytes();

        let encrypted = speck_cipher::speck_cbc_encrypt(key, iv, msg);
        let decrypted = speck_cipher::speck_cbc_decrypt(key, iv, &encrypted);

        assert_eq!(msg, decrypted);
    }
}
