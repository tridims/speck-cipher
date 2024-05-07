set terminal png
set output "entropy_plot.png"

set title "Entropy Comparison: Plaintext vs Ciphertext"
set xlabel "Sample Index"
set ylabel "Entropy (bits per byte)"

set style data points

plot "entropy_values.txt" using 1 title "Plaintext Entropy", \
     "entropy_values.txt" using 2 title "Ciphertext Entropy"