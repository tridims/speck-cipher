set title "Entropy Comparison: Plaintext vs Ciphertext" font ",54" textcolor rgbcolor "#333333"
set xlabel "Sample Index" font ",42" textcolor rgbcolor "#333333"
set ylabel "Entropy (bits per byte)" font ",42" textcolor rgbcolor "#333333"
set grid linecolor rgbcolor "#cccccc" linetype 1 linewidth 1.5
set border linecolor rgbcolor "#999999" linewidth 4.5
set key top right font ",36" box linecolor rgbcolor "#999999" linewidth 4.5
set xtics font ",36" textcolor rgbcolor "#333333"
set ytics font ",36" textcolor rgbcolor "#333333"
set key at graph 0.8, 0.5 center font ",36" box linecolor rgbcolor "#999999" linewidth 4.5
set terminal pngcairo enhanced font "Arial,36" size 2400,1800
set output "entropy_plot.png"
set style line 1 linecolor rgbcolor "#FF5733" linewidth 6 pt 7 ps 2.4
set style line 2 linecolor rgbcolor "#3498DB" linewidth 6 pt 9 ps 2.4
plot "data.txt" using 1 with points linestyle 1 title "Plaintext Entropy", "data.txt" using 2 with points linestyle 2 title "Ciphertext Entropy"