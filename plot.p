#! /usr/bin/gnuplot -persist
data = "./data.txt"
data_ampl = "./amplitudes.txt"

set term wxt 1 size 1280,1024
#set terminal png size 1280,1024
#set output 'plots/d1-d2_pr.png'
set xlabel "sample" 
set ylabel "amplitude"
#set xrange[0:100000]
#set yrange [-1.1:1.1]
#set yrange [-0.02:0.02]
set key left top
set grid
plot data title "Input Signal" lc rgb "red" with linespoint
    

set term wxt 2
#set terminal png size 1280,1024
#set output 'plots/d1-d2_pr.png'
set xlabel "sample" 
set ylabel "amplitude"
#set xrange[0:2896]
#set yrange [-1.1:1.1]
#set yrange [-0.02:0.02]
set key left top
set grid
plot data_ampl title "Transformed Signal" lc rgb "blue" with lines
