explaining : [../examples/uniformity_analysis.rs](../examples/uniformity_analysis.rs)

# Introduction

This code conducts a uniformity test on ciphertexts generated using the Speck cipher in CBC mode. The objective is to assess the uniformity of the ciphertext distribution to ensure it approximates a uniform random distribution, which is a desirable property for secure encryption.

# Test Overview

1. Random Key and IV Generation:

   - Generates 100 random 32-byte keys.
   - For each key, a single random 16-byte initialization vector (IV) is generated.

2. Mnemonic Phrase Encryption:

   - For each key, encrypts 10,000 random 12-word mnemonic phrases using the Speck cipher in CBC mode.

3. Uniformity Analysis:

   - For each encrypted mnemonic phrase, the ciphertext is analyzed to calculate the Chi-square statistic, comparing the observed byte frequencies to the expected uniform distribution.
   - The average Chi-square statistic is computed for each key across the 10,000 samples.

4. Significance Testing:

   - The critical value for the Chi-square test is determined based on a 5% significance level with 255 degrees of freedom (256 bins - 1).
   - The percentage of keys that produce ciphertexts with a Chi-square statistic below the critical value (indicating uniformity) is calculated and reported.

5. Data Export:

   - The average Chi-square statistics for each key and the critical value are exported to files for further analysis or record-keeping.

This test provides an evaluation of the uniformity of ciphertexts across multiple keys, ensuring a robust assessment of the encryption scheme's performance.
