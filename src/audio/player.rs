use std::path::Path;

use super::FFmpegSampleRateConversion;
use cpal::{
    traits::*, Device, Host, Sample as CpalSample, SampleFormat as CpalSampleFormat, StreamConfig,
    SupportedStreamConfig,
};

use ffmpeg_next::{
    format::{context::Input, input},
    software::resampling::Context as FFmpegResamplingContext,
};

struct AudioContext {
    input_context: Input,
    resampling_context: Option<FFmpegResamplingContext>,
}

impl AudioContext {
    pub fn new_file<P: AsRef<Path>>(file: P) -> anyhow::Result<Self> {
        Ok(Self {
            input_context: input(&file)?,
            resampling_context: None,
        })
    }
    pub fn set_resampling_context(mut self, resample: FFmpegResamplingContext) -> Self {
        self.resampling_context = Some(resample);
        self
    }
}

pub struct AudioPlayer {
    host: Host,
    device: Device,
    config: StreamConfig,
    sample_format: CpalSampleFormat,
    sample_rate: u32,
    channels: u16,
    contexts: Vec<AudioContext>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let mut supported_config_range = device.supported_output_configs().unwrap();
        let supported_config = supported_config_range
            .next()
            .unwrap()
            .with_max_sample_rate();
        let config = supported_config.config();
        let sample_format = supported_config.sample_format();
        let sample_rate = supported_config.sample_rate().0;
        let channels = supported_config.channels();
        Self {
            host,
            device,
            config,
            sample_format,
            sample_rate,
            channels,
            contexts: vec![],
        }
    }
    pub fn play_file<P: AsRef<Path>>(&mut self, file: P) {
        if let Ok(file) = AudioContext::new_file(file) {
            self.contexts.push(file)
        }
    }
}
