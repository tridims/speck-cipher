set title "Avalanche Effect"
set xlabel "Number of Flipped Bits"
set ylabel "Avalanche Effect"
set grid
set terminal png

# Plot for plaintext
set output "avalanche_plot_plaintext.png"
plot "avalanche_data_plaintext.txt" with linespoints title "Avalanche Effect - Plaintext"

# Plot for key
set output "avalanche_plot_key.png"
plot "avalanche_data_key.txt" with linespoints title "Avalanche Effect - Key"