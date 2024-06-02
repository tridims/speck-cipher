Explaining [../example/entropy.rs](../examples/entropy.rs)

# Introduction

This Rust code evaluates the entropy of plaintext and its corresponding ciphertext after encryption using the Speck cipher in CBC (Cipher Block Chaining) mode. Entropy is a measure of randomness, indicating how unpredictable the data is.

# Test Conducted

The code conducts an entropy analysis of the encryption process by comparing the entropy of random plaintexts with their corresponding ciphertexts.

# Steps for the Test

1. Generate Random Key and IV
    - Generate a 32-byte random key.
    - Generate a 16-byte random initialization vector (IV).

2. Generate Random Plaintexts
    - Generate 500 random plaintexts. Each plaintext is a 12-word mnemonic phrase converted to bytes.

3. Encrypt Plaintexts
    - Encrypt each plaintext using the Speck cipher in CBC mode with the generated key and IV.

4. Calculate Entropy
    - Calculate the entropy of each plaintext.
    - Calculate the entropy of each corresponding ciphertext.

5. Record Results
    - Write the entropy values of plaintexts and ciphertexts to a file for further analysis.

# Summary

This test helps in understanding the randomness of the Speck cipher's output by comparing the entropy of the input plaintexts and the resulting ciphertexts. Higher entropy in ciphertexts indicates better encryption quality.