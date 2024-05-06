set title "Avalanche Effect" font ",18" textcolor rgbcolor "#333333"
set xlabel "Number of Flipped Bits" font ",14" textcolor rgbcolor "#333333"
set ylabel "Avalanche Effect" font ",14" textcolor rgbcolor "#333333"

set grid linecolor rgbcolor "#cccccc" linetype 1 linewidth 0.5
set border linecolor rgbcolor "#999999" linewidth 1.5

set key top right font ",12" box linecolor rgbcolor "#999999" linewidth 1.5

set xtics font ",12" textcolor rgbcolor "#333333"
set ytics font ",12" textcolor rgbcolor "#333333"

set terminal pngcairo enhanced font "Arial,12" size 800,600
set output "avalanche_plot.png"

set style line 1 linecolor rgbcolor "#FF5733" linewidth 2 pt 7 ps 1
set style line 2 linecolor rgbcolor "#3498DB" linewidth 2 pt 9 ps 1

plot "avalanche_data_plaintext.txt" with linespoints linestyle 1 title "Avalanche Effect - Plaintext", \
     "avalanche_data_key.txt" with linespoints linestyle 2 title "Avalanche Effect - Key"