use crate::speck::Speck128_256;

pub fn xor_bytes(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect()
}

pub fn speck_cbc_encrypt(key: &[u8; 32], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let padded_plaintext = crate::padding::pad_pkcs7(plaintext, 16);
    let iv = iv.to_vec();

    let cipher = Speck128_256::new(key);
    let mut encrypted_blocks: Vec<Vec<u8>> = Vec::new();

    (0..padded_plaintext.len()).step_by(16).for_each(|x| {
        // Take last encrypted block or IV for first block iteration
        let last = encrypted_blocks.last().unwrap_or(&iv);

        // XOR last encrypted block with current msg block & encrypt result
        let mut block: [u8; 16] = xor_bytes(last, &padded_plaintext[x..x + 16])
            .try_into()
            .expect("Invalid block size");
        cipher.encrypt(&mut block);

        encrypted_blocks.push(block.into_iter().collect::<Vec<u8>>());
    });

    encrypted_blocks.into_iter().flatten().collect::<Vec<u8>>()
}

pub fn speck_cbc_decrypt(key: &[u8; 32], iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let iv = iv.to_vec();

    let cipher = Speck128_256::new(key);
    let mut decrypted_blocks: Vec<Vec<u8>> = Vec::new();

    let mut previous_ciphertext_block = iv.to_vec();
    (0..ciphertext.len()).step_by(16).for_each(|x| {
        // Decrypt current block & XOR with last encrypted block
        let mut block: [u8; 16] = ciphertext[x..x + 16]
            .try_into()
            .expect("Invalid block size");
        cipher.decrypt(&mut block);
        block = xor_bytes(&previous_ciphertext_block, &block)
            .try_into()
            .expect("Invalid block size");

        previous_ciphertext_block = ciphertext[x..x + 16].to_vec();
        decrypted_blocks.push(block.into_iter().collect::<Vec<u8>>());
    });

    crate::padding::unpad_pkcs7(&decrypted_blocks.into_iter().flatten().collect::<Vec<u8>>())
}
