explaining : 
1. [../examples/avalanche.rs](../examples/avalanche.rs)
2. [../examples/avalanche_2.rs](../examples/avalanche_2.rs)
3. [../examples/avalanche_3.rs](../examples/avalanche_3.rs)

# Introduction

This Rust code tests the avalanche effect of the Speck block cipher. The avalanche effect measures how small changes in the input (plaintext or key) lead to significant changes in the output (ciphertext).
Test Conducted

# In The Original Test, The code conducts two main tests:

1. Avalanche Effect on Plaintext
2. Avalanche Effect on Key

Steps for Each Test

1.  Generate Random Data

    - Generate a random plaintext.
    - Generate a random key and initialization vector (IV).

2.  Encrypt the Original Plaintext

    - Encrypt the plaintext using the Speck cipher with the original key and IV.
    - Store the resulting ciphertext.

3.  Modify Input and Encrypt Again

    - For a range of flipped bits (from 1 to a specified maximum):
        - For Plaintext Test: Flip bits in the plaintext.
        - For Key Test: Flip bits in the key.
    - Encrypt the modified plaintext or with the modified key using the same IV.

4.  Calculate Hamming Distance

    - Compare the original ciphertext with the modified ciphertext.
    - Calculate the Hamming distance, which counts the differing bits between the two ciphertexts.

5.  Record and Average Results

    - Record the Hamming distance for each number of flipped bits.
    - Average the results over multiple iterations to get the final avalanche effect score for each number of flipped bits.

# Summary

By following these steps, the code evaluates how sensitive the Speck cipher is to small changes in the plaintext and key, providing a measure of its avalanche effect.

# Difference on the 2nd and 3rd Test

- The second test using mnemonic data with a modified data of the last phrase of the mnemonic.
- The third test using mnemonic data with a modified data of the first phrase of the mnemonic and one bit changed on the key.