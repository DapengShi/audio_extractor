use audio_extractor::{Args, AudioFormat};
use std::path::PathBuf;
use tempfile::NamedTempFile;
use std::fs;
use std::process::Command;

pub fn create_test_video_file() -> NamedTempFile {
    let file = NamedTempFile::with_suffix(".mp4").unwrap();
    
    // Create a minimal test video file using FFmpeg
    let output = Command::new("ffmpeg")
        .arg("-f").arg("lavfi")
        .arg("-i").arg("testsrc=duration=1:size=320x240:rate=30")
        .arg("-f").arg("lavfi")
        .arg("-i").arg("sine=frequency=1000:duration=1")
        .arg("-c:v").arg("libx264")
        .arg("-c:a").arg("aac")
        .arg("-t").arg("1")
        .arg("-y") // Overwrite if exists
        .arg(file.path())
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            // Successfully created test video
            file
        }
        _ => {
            // FFmpeg failed or not available, create a placeholder
            // This will cause tests to be skipped or use fallback methods
            fs::write(file.path(), b"fake video data").unwrap();
            file
        }
    }
}

pub fn create_test_args(input: PathBuf, output: PathBuf) -> Args {
    Args {
        input,
        output,
        format: Some(AudioFormat::Mp3),
        quality: Some(128),
        verify: false,
    }
}