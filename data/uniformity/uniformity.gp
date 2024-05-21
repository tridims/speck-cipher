set terminal pngcairo enhanced font "Arial,12" size 800,600
set output "uniformity.png"
set title "Chi-Square Test for Encrypted Mnemonic Phrase" font ",18" textcolor rgbcolor "#333333"
set xlabel "Key Index" font ",14" textcolor rgbcolor "#333333"
set ylabel "Chi-Square Value" font ",14" textcolor rgbcolor "#333333"
set grid linecolor rgbcolor "#cccccc" linetype 1 linewidth 0.5
set border linecolor rgbcolor "#999999" linewidth 1.5
set key top right font ",12" box linecolor rgbcolor "#999999" linewidth 1.5 opaque
set xtics font ",12" textcolor rgbcolor "#333333"
set ytics font ",12" textcolor rgbcolor "#333333"
set yrange [240:300]  # Adjust this range as needed
set style line 1 linecolor rgbcolor "#FF5733" pt 7 ps 1.5
set style line 2 linecolor rgbcolor "#3498DB" linetype 1 linewidth 2
critical_value = 293.24647521972656
plot "uniformity_data.txt" with points linestyle 1 title "Chi-Square Values", critical_value with lines linestyle 2 title "Critical Value"