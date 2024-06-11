set title "Avalanche Effect" font ",54" textcolor rgbcolor "#333333"
set xlabel "Number of Flipped Bits" font ",42" textcolor rgbcolor "#333333"
set ylabel "Avalanche Effect" font ",42" textcolor rgbcolor "#333333"
set grid linecolor rgbcolor "#cccccc" linetype 1 linewidth 1.5
set border linecolor rgbcolor "#999999" linewidth 4.5
set key top right font ",36" box linecolor rgbcolor "#999999" linewidth 4.5
set xtics font ",36" textcolor rgbcolor "#333333"
set ytics font ",36" textcolor rgbcolor "#333333"
set terminal pngcairo enhanced font "Arial,36" size 2400,1800

# test 1 plot
set key at graph 0.7, 0.5 center font ",36" box linecolor rgbcolor "#999999" linewidth 4.5
set output "avalanche_plot_1.png"
set style line 1 linecolor rgbcolor "#FF5733" linewidth 6 pt 7 ps 3
set style line 2 linecolor rgbcolor "#3498DB" linewidth 6 pt 9 ps 3
plot "test_1/data_plaintext.txt" with linespoints linestyle 1 title "Avalanche Effect - Plaintext", \
    "test_1/data_key.txt" with linespoints linestyle 2 title "Avalanche Effect - Key"

# test 2 & 3 plot (merged)
set key at graph 0.5, 0.5 center font ",36" box linecolor rgbcolor "#999999" linewidth 4.5
set output "avalanche_plot_2_3.png"
set style line 1 linecolor rgbcolor "#FF5733" linewidth 6 pt 7 ps 3
set style line 2 linecolor rgbcolor "#3498DB" linewidth 6 pt 9 ps 3
set style line 3 linecolor rgbcolor "#2ECC71" linewidth 6 pt 5 ps 3  # Changed color, linewidth, point type, point size, and line type
plot "test_2/data.txt" using 1:2 with linespoints linestyle 1 title "Avalanche Effect - BackMod", \
    "test_2/data.txt" using 1:3 with linespoints linestyle 2 title "Avalanche Effect - FrontMod", \
    "test_3/data.txt" using 1:2 with linespoints linestyle 3 title "Avalanche Effect - BackMod + Unique Key & IV"