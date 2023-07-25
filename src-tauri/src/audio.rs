use std::{
    fs::File,
    io::BufReader,
    time::Duration,
};

use rodio::{Decoder, OutputStream, Source};
use tauri::AppHandle;

use crate::error::Error;

pub async fn paly_audio(_app: &AppHandle, path: &str) -> Result<(), Error> {
    tracing::info!("开始播放音频: {:?}",path);
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path)?);
    // Decode that sound file into a source
    let source = Decoder::new(file)?;
    let total_duration = source.total_duration().unwrap_or(Duration::from_secs(0));
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples())?;
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(total_duration);
    Ok(())
}
