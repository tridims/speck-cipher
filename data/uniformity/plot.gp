set terminal pngcairo enhanced font "Arial,36" size 2400,1800
set title "Chi-Square Test for Encrypted Mnemonic Phrase" font ",54" textcolor rgbcolor "#333333"
set xlabel "Key Index" font ",42" textcolor rgbcolor "#333333"
set ylabel "Chi-Square Value" font ",42" textcolor rgbcolor "#333333"
set grid linecolor rgbcolor "#cccccc" linetype 1 linewidth 1.5
set border linecolor rgbcolor "#999999" linewidth 4.5
set key top right font ",36" box linecolor rgbcolor "#999999" linewidth 4.5 opaque
set xtics font ",36" textcolor rgbcolor "#333333"
set ytics font ",36" textcolor rgbcolor "#333333"
set style line 1 linecolor rgbcolor "#FF5733" pt 7 ps 1.5
set style line 2 linecolor rgbcolor "#3498DB" linetype 1 linewidth 6

critical_value = 293.24647521972656

do for [i=1:2] {
    set output sprintf("uniformity_test_%d.png", i)
    plot sprintf("./test_%d/data.txt", i) with points linestyle 1 title "Chi-Square Values", critical_value with lines linestyle 2 title "Critical Value"
}

set style line 1 linecolor rgbcolor "#FF5733" pt 7 ps 3
set output "uniformity_test_3.png"
plot "./test_3/data.txt" with points linestyle 1 title "Chi-Square Values", critical_value with lines linestyle 2 title "Critical Value"