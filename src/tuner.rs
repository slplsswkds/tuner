use std::fs::File;
use cpal::traits::{DeviceTrait, HostTrait};
// use ringbuf::HeapRb;
use std::io::Write;

pub struct Tuner {
    host: cpal::Host,
    input_device: cpal::Device,
    stream_config: cpal::StreamConfig,
    pub stream: cpal::Stream,
}

impl Default for Tuner {
    fn default() -> Self {
        let host = cpal::default_host();
        let input_device = match host.default_input_device() {
            Some(dev) => dev,
            None => panic!("noone input device is available"),
        };
        let mut stream_config: cpal::StreamConfig = match input_device.default_input_config() {
            Ok(config) => config.into(),
            Err(err) => panic!("{}", err),
        };
        stream_config.channels = 1u16;
        stream_config.sample_rate = cpal::SampleRate(1000);

        let mut f = File::create("./data.txt").expect("Unable to create file");
        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            for &sample in data {
                writeln!(f, "{}", &sample.to_string());
                //println!("{}", &sample);
            }
        };

        let stream = match input_device.build_input_stream(&stream_config, input_data_fn, err_fn) {
            Ok(stream) => stream,
            Err(err) => panic!("{}", err),
        };
        Self {
            host: host,
            input_device: input_device,
            stream_config: stream_config,
            stream: stream,
        }
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}