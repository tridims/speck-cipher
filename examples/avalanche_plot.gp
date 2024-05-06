set title "Avalanche Effect"
set xlabel "Number of Flipped Bits"
set ylabel "Avalanche Effect"
set grid
set terminal png
set output "avalanche_plot.png"
plot "avalanche_data.txt" with linespoints title "Avalanche Effect"