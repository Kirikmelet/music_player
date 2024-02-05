use audio::_audio_play_test_file;
use run::run;

mod audio;
mod config;
mod db;
mod event;
mod page;
mod run;

#[tokio::main]
async fn main() {
    // Init ffmpeg :)
    ffmpeg_next::init().unwrap();
    let log_file = std::fs::File::create("log.log").unwrap();
    let (non_blocking_log_file, _guard) = tracing_appender::non_blocking(log_file);
    tracing_subscriber::fmt()
        .with_writer(non_blocking_log_file)
        .init();
    //run().await.unwrap();
    _audio_play_test_file("./test/beep.wav");
    // _audio_play_test_file("./test/beep.ogg");
    // _audio_play_test_file("./test/wangxian.opus");
    // _audio_play_test_file("./test/futari.mp3");
    // _audio_play_test_file("./test/futari.flac");
}
