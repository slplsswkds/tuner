#! /usr/bin/gnuplot -persist
data = "./data.txt"

set term wxt 1 size 1280,1024
#set terminal png size 1280,1024
#set output 'plots/d1-d2_pr.png'
set xlabel "sample" 
set ylabel "amplitude"
#set xrange[0:100000]
#set yrange [-1.1:1.1]
set yrange [-0.3:0.3]
set key left top
set grid
set title "Input Signal" font "Helvetica Bold, 14"
plot data title "Input Signal" lc rgb "red"
