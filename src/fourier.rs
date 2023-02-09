use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::f32::consts::PI;

use std::io::Write;
use nalgebra::Complex;

#[derive(Default)]
pub struct Signal {
    src_data: Vec<f32>,
    fourier_transform: Vec<Complex<f32>>,
}

impl Signal {
    pub fn load_src_data(&mut self) {
        let file = File::open("./data.txt").expect("file wasn't found.");
        let reader = BufReader::new(file);
        let numbers: Vec<f32> = reader
            .lines()
            .map(|line| line.unwrap().parse::<f32>().unwrap())
            .collect();
        self.src_data = numbers;
    }

    // Discrete Fourier transform in exponential form:
    pub fn dft(&mut self) {
        // the number of readings of the input sequence and the number of frequency readings of the DFT result
        let len = self.src_data.len();

        let mut s = Complex::new(0.0, 0.0);

        for k in 0..len {
            s.re = 0.0;
            s.im = 0.0;
            for n in 0..len {
                s.re += self.src_data.get(n).unwrap() * (2.0 * PI * k as f32 * n as f32 / len as f32).cos();
                s.im -= self.src_data.get(n).unwrap() * (2.0 * PI * k as f32 * n as f32 / len as f32).cos();
            }
            self.fourier_transform.push(s);
        }

        let mut f = File::create("./amplitudes.txt").expect("Unable to create file");
        let mut counter = 0;
        let mut max_val = 0.0;
        let mut max_val_pos = 0;
        for k in 0..len/2 {
            counter += 1;
            let re_k = self.fourier_transform.get(k).unwrap().re;
            let im_k = self.fourier_transform.get(k).unwrap().im;
            let a_k = (re_k.powf(2.0).sqrt() + im_k.powf(2.0).sqrt()).sqrt() / (len / 2) as f32;

            if a_k > max_val {
                max_val = a_k;
                max_val_pos = counter;
            }

            //println!("{}", a_k);
            writeln!(f, "{}", a_k);
        }
        println!("frequency is {} Hz", max_val_pos as f32 * 1f32 / (len as f32 / 1000f32)); // max_val_pos * 1 / time of recording
                                                                        // max_val_pos * 1 / (samples_len / sample rate)
    }
}