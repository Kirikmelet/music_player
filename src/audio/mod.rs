use cpal::{traits::*, Sample, SampleFormat, StreamConfig};
use ffmpeg_next::{
    self,
    codec::Context as FFMPEGCodecContext,
    format::{
        self,
        sample::{Buffer, Type as FFmpegSampleType},
        stream::Stream,
        Sample as FFmpegSample,
    },
    frame::Audio as FFmpegAudio,
    media::Type as FFmpegMediaType,
    software::resampling::{delay, Context as FFmpegResamplingContext},
    ChannelLayout,
};
use ringbuf::{HeapRb, SharedRb};
use std::{f32::consts::PI, mem::MaybeUninit, path::Path, sync::Arc, time::Duration};

pub mod player;

/* WHY THIS MAGIC NUMBER
 * 12 is the LCM (least common multiple) of 1,2,3,4
 */

pub trait FFmpegSampleRateConversion {
    fn as_ffmpeg_sample_rate(&self) -> FFmpegSample;
}

impl FFmpegSampleRateConversion for SampleFormat {
    fn as_ffmpeg_sample_rate(&self) -> FFmpegSample {
        match self {
            Self::F32 => FFmpegSample::F32(FFmpegSampleType::Packed),
            Self::F64 => FFmpegSample::F64(FFmpegSampleType::Packed),
            Self::I16 => FFmpegSample::I16(FFmpegSampleType::Packed),
            Self::I32 => FFmpegSample::I32(FFmpegSampleType::Packed),
            Self::I64 => FFmpegSample::I64(FFmpegSampleType::Packed),
            Self::U8 => FFmpegSample::U8(FFmpegSampleType::Packed),
            _ => FFmpegSample::None,
        }
    }
}

pub fn _audio_play_test_file<P: AsRef<Path>>(file: P) {
    let err_fn = |err| eprintln!("Error: {}", err);
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let mut supported_configs_range = device.supported_output_configs().unwrap();
    let supported_config = supported_configs_range
        .next()
        .unwrap()
        .with_max_sample_rate();
    //let stream = &device.build_output_stream(
    //    &supported_config.config(),
    //    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {},
    //    |err| {},
    //    None,
    //);
    let config: &StreamConfig = &supported_config.config();
    let sample_format = supported_config.sample_format();
    let sample_rate = supported_config.sample_rate();
    /* {{{START FFMPEG */
    let mut ffmpeg_input_context = format::input(&file).unwrap();
    let ffmpeg_audio_stream: Stream = ffmpeg_input_context
        .streams()
        .best(FFmpegMediaType::Audio)
        .unwrap();
    let ffmpeg_audio_stream_index: usize = ffmpeg_audio_stream.index();
    let codec_context =
        FFMPEGCodecContext::from_parameters(ffmpeg_audio_stream.parameters()).unwrap();
    let mut ffmpeg_audio_decoder = codec_context.decoder().audio().unwrap();
    eprintln!(
        "{:?}",
        ffmpeg_audio_decoder
            .codec()
            .and_then(|f| Some(f.name().to_string()))
    );
    if ffmpeg_audio_decoder.channel_layout().is_empty() {
        // ffmpeg_audio_decoder.set_channel_layout(ChannelLayout::default(
        //     ffmpeg_audio_decoder.channels() as i32,
        // ));
        ffmpeg_audio_decoder.set_channel_layout(ChannelLayout::MONO);
    }
    let audio_buffer_size = Buffer::size(
        sample_format.as_ffmpeg_sample_rate(),
        ffmpeg_audio_decoder.channels(),
        4608,
        true,
    );
    eprintln!("audio buffer size: {} bytes", audio_buffer_size);
    /* END FFMPEG}}} */
    /* AUDIO BUFFER */
    let layout_debug = ffmpeg_audio_decoder.channel_layout();
    tracing::info!(?layout_debug);
    let mut resampler = ffmpeg_audio_decoder
        .resampler(
            sample_format.as_ffmpeg_sample_rate().clone(),
            ffmpeg_audio_decoder.channel_layout(),
            sample_rate.0,
        )
        .unwrap();
    // return;
    let audio_buffer: HeapRb<f32> = HeapRb::new(audio_buffer_size as usize);
    let (mut audio_buffer_producer, mut audio_buffer_consumer) = audio_buffer.split();
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(
            config,
            move |data: &mut [f32], _cb: &cpal::OutputCallbackInfo| {
                _play_audio(data, _cb, &mut audio_buffer_consumer)
            },
            err_fn,
            None,
        ),
        _ => unimplemented!("Not yet!"),
    }
    .unwrap();
    let mut decode_and_resample_audio = |decoder: &mut ffmpeg_next::decoder::Audio| {
        let mut decoded = FFmpegAudio::empty();
        while decoder.receive_frame(&mut decoded).is_ok() {
            if decoded.is_corrupt() {
                panic!("Decoded is corrupt!");
            }
            let mut resampled = FFmpegAudio::empty();
            if let Some(delay) = resampler.run(&decoded, &mut resampled).unwrap() {
                tracing::info!("sleeping");
                std::thread::sleep(Duration::from_secs(
                    delay.seconds.try_into().unwrap_or_default(),
                ));
                continue;
            };
            if !resampled.is_packed() {
                panic!("cringe, is not packed");
            }
            let both_channels = _packed(&resampled);
            while audio_buffer_producer.free_len() < both_channels.len() {
                std::thread::sleep(Duration::from_millis(10));
            }
            audio_buffer_producer.push_slice(both_channels);
        }
    };
    stream.play().unwrap();
    for (stream, packet) in ffmpeg_input_context.packets() {
        if stream.index() == ffmpeg_audio_stream_index {
            ffmpeg_audio_decoder.send_packet(&packet).unwrap();
            decode_and_resample_audio(&mut ffmpeg_audio_decoder);
        }
    }
    //stream.pause().unwrap();
}

fn _play_audio<T: Sample>(
    data: &mut [T],
    _: &cpal::OutputCallbackInfo,
    samples: &mut ringbuf::Consumer<T, Arc<SharedRb<T, Vec<MaybeUninit<T>>>>>,
) {
    for sample in data.iter_mut() {
        match samples.pop() {
            Some(x) => *sample = x.to_sample(),
            None => *sample = Sample::EQUILIBRIUM,
        }
    }
}

fn _packed<T: ffmpeg_next::frame::audio::Sample>(frame: &ffmpeg_next::frame::Audio) -> &[T] {
    if !frame.is_packed() {
        panic!("SHOULD'VE been packed!");
    }
    if !<T as ffmpeg_next::frame::audio::Sample>::is_valid(frame.format(), frame.channels()) {
        panic!("oops");
    }
    unsafe {
        std::slice::from_raw_parts(
            (*frame.as_ptr()).data[0] as *const T,
            frame.samples() * frame.channels() as usize,
        )
    }
}
