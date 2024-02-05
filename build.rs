const FFMPEG_DIR: Option<&'static str> = option_env!("FFMPEG_DIR");

fn main() {
    if !cfg!(target_os = "windows") {
        eprintln!("cargo:warning=Sorry not supported (yet)");
        return;
    }
    if FFMPEG_DIR.is_none() {
        eprintln!("cargo:warning=YOU NEED `FFMPEG_DIR` SET");
    }
}
