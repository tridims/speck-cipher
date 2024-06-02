set xlabel "Data Index" font ",14" textcolor rgbcolor "#333333"
set ylabel "Avalanche Effect" font ",14" textcolor rgbcolor "#333333"

set grid linecolor rgbcolor "#cccccc" linetype 1 linewidth 0.5
set border linecolor rgbcolor "#999999" linewidth 1.5

set key top right font ",12" box linecolor rgbcolor "#999999" linewidth 1.5

set xtics font ",12" textcolor rgbcolor "#333333"
set ytics font ",12" textcolor rgbcolor "#333333"

set terminal pngcairo enhanced font "Arial,12" size 800,600
set output "avalanche_plot.png"

set style line 1 linecolor rgbcolor "#FF5733" linewidth 2 pt 7 ps 1

plot "avalanche_2.txt" using 1:2 with linespoints linestyle 1 title "Avalanche Effect"