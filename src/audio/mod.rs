use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host, Stream, StreamConfig,
};

pub struct AudioPlayer {
    stream: Option<Stream>,
    config: StreamConfig,
    device: Device,
    host: Host,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("Uh oh, no speaker xd");
        let mut supported_config_range = device.supported_output_configs().expect("wtf");
        let config = supported_config_range
            .next()
            .expect("???")
            .with_max_sample_rate()
            .into();
        Self {
            stream: None,
            config,
            device,
            host,
        }
    }
    pub fn test_run() {

    }
}
