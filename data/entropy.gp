set title "Entropy Comparison: Plaintext vs Ciphertext" font ",18" textcolor rgbcolor "#333333"
set xlabel "Sample Index" font ",14" textcolor rgbcolor "#333333"
set ylabel "Entropy (bits per byte)" font ",14" textcolor rgbcolor "#333333"
set grid linecolor rgbcolor "#cccccc" linetype 1 linewidth 0.5
set border linecolor rgbcolor "#999999" linewidth 1.5
set key top right font ",12" box linecolor rgbcolor "#999999" linewidth 1.5
set xtics font ",12" textcolor rgbcolor "#333333"
set ytics font ",12" textcolor rgbcolor "#333333"
set key at graph 0.8, 0.5 center font ",12" box linecolor rgbcolor "#999999" linewidth 1.5
set terminal pngcairo enhanced font "Arial,12" size 800,600
set output "entropy_plot.png"
set style line 1 linecolor rgbcolor "#FF5733" linewidth 2 pt 7 ps 1.5
set style line 2 linecolor rgbcolor "#3498DB" linewidth 2 pt 9 ps 1.5
plot "entropy_values.txt" using 1 with points linestyle 1 title "Plaintext Entropy", "entropy_values.txt" using 2 with points linestyle 2 title "Ciphertext Entropy"