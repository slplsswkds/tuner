mod tuner;
mod fourier;

use tuner::Tuner;
use cpal::traits::{StreamTrait};

fn main() {
    let mut tuner = Tuner::default();
    tuner.stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(3000));
    drop(tuner);

    let mut signal = fourier::Signal::default();
    signal.load_src_data();
    signal.dft();
}