set terminal png
set output "uniformity.png"
set title "Chi-Square Test for Encrypted Mnemonic Phrase"
set xlabel "Key Index"
set ylabel "Chi-Square Value"
plot "uniformity_data.txt" with points