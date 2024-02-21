use std::path::Path;

use super::FFmpegSampleFormatConversion;
use cpal::{
    traits::*, Device, Host, Sample as CpalSample, SampleFormat as CpalSampleFormat, StreamConfig,
    SupportedStreamConfig,
};

use ffmpeg_next::{
    codec::Context as FFMpegCodecContext,
    decoder::Audio as FFMpegAudio,
    format::{context::Input, input as FFMpegInput},
    media::Type as FFMpegMediaType,
    util::{
        error::Error as FFMpegError,
        format::{sample::Buffer as FFMpegBuffer, Sample as FFMpegSample},
    },
};
use ringbuf::HeapRb;

const AUDIO_DATA_BUFFER_SIZE: usize = 15600;

#[derive(Debug)]
enum AudioContextError {
    FFMpegInputError(FFMpegError),
    NoAudioStream,
    FFMpegCodecError(FFMpegError),
    FFMpegAudioDecoder(FFMpegError),
}

impl std::fmt::Display for AudioContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FFMpegInputError(e) => {
                write!(f, "FFMpegInputError: {}", e)
            }
            Self::FFMpegCodecError(e) => {
                write!(f, "FFMpegCodecError: {}", e)
            }
            Self::FFMpegAudioDecoder(e) => {
                write!(f, "FFMpegAudioDecoder: {}", e)
            }
            Self::NoAudioStream => write!(f, "NoAudioStream"),
            _ => write!(f, ""),
        }
    }
}

impl std::error::Error for AudioContextError {}

struct AudioContext {
    input_context: Input,
    index: usize,
    decoder: FFMpegAudio,
}

impl AudioContext {
    pub fn new_file<P: AsRef<Path>>(
        file: P,
        sample_format: FFMpegSample,
    ) -> Result<AudioContext, AudioContextError> {
        let input_context = FFMpegInput(&file).map_err(AudioContextError::FFMpegInputError)?;
        let stream = input_context
            .streams()
            .best(FFMpegMediaType::Audio)
            .ok_or(AudioContextError::NoAudioStream)?;
        let index = stream.index();
        let codec = FFMpegCodecContext::from_parameters(stream.parameters())
            .map_err(AudioContextError::FFMpegCodecError)?;
        let decoder = codec
            .decoder()
            .audio()
            .map_err(AudioContextError::FFMpegAudioDecoder)?;
        Ok(Self {
            input_context,
            index,
            decoder,
        })
    }
}

pub struct AudioPlayer<T>
where
    T: num::Num,
{
    host: Host,
    device: Device,
    config: StreamConfig,
    sample_format: CpalSampleFormat,
    sample_rate: u32,
    channels: u16,
    data_buffer: HeapRb<T>,
    contexts: Vec<AudioContext>,
}

impl<T> AudioPlayer<T>
where
    T: num::Num,
{
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
        let data_buffer: HeapRb<T> = HeapRb::new(AUDIO_DATA_BUFFER_SIZE);
        Self {
            host,
            device,
            config,
            sample_format,
            sample_rate,
            channels,
            data_buffer,
            contexts: vec![],
        }
    }
    pub fn play_file<P: AsRef<Path>>(&mut self, file: P) {
        if let Ok(file) = AudioContext::new_file(file, self.sample_format.as_ffmpeg_sample_format())
        {
            self.contexts.push(file)
        }
    }
}
