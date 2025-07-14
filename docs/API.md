# API Documentation

## 🏗️ Core Structures

### Args Structure
```rust
#[derive(Parser)]
pub struct Args {
    /// Input video file path
    pub input: PathBuf,
    
    /// Output audio file path
    pub output: PathBuf,
    
    /// Output audio format
    pub format: AudioFormat,
    
    /// Audio quality (bitrate in kbps)
    pub quality: u32,
    
    /// Whether to verify output file
    pub verify: bool,
}
```

### AudioFormat Enum
```rust
#[derive(Clone, ValueEnum, Debug, PartialEq)]
pub enum AudioFormat {
    Mp3,
    Wav,
    Flac,
    Aac,
}
```

### AudioFileInfo Structure
```rust
#[derive(Debug, Clone)]
pub struct AudioFileInfo {
    pub format: String,
    pub duration: Option<f64>,
    pub channels: Option<usize>,
    pub sample_rate: Option<u32>,
}
```

### VideoInfo Structure
```rust
#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub duration: f64,
    pub has_audio: bool,
}
```

## 🔧 Core API

### AudioExtractor Structure

#### Constructor
```rust
impl AudioExtractor {
    pub fn new(args: Args) -> Self
}
```

#### Basic Extraction
```rust
pub fn extract(&self) -> Result<()>
```
Performs basic audio extraction operations.

#### Extraction with Progress
```rust
pub fn extract_with_progress<F>(&self, progress_callback: F) -> Result<()>
where
    F: Fn(&str) + Send + Sync,
```
Performs audio extraction and reports progress through a callback function.

**Parameters**:
- `progress_callback`: Progress callback function that receives status messages

**Example**:
```rust
let extractor = AudioExtractor::new(args);
extractor.extract_with_progress(|msg| {
    println!("Progress: {}", msg);
})?;
```

#### Batch Processing
```rust
pub fn extract_batch<P: AsRef<std::path::Path>>(
    inputs: Vec<P>,
    output_dir: P,
    format: AudioFormat,
    quality: u32,
    verify: bool,
) -> Result<Vec<Result<PathBuf>>>
```
Processes multiple video files in batch.

**Parameters**:
- `inputs`: List of input file paths
- `output_dir`: Output directory path
- `format`: Output audio format
- `quality`: Audio quality (bitrate)
- `verify`: Whether to verify output files

**Return Value**:
- `Vec<Result<PathBuf>>`: Processing result for each file

**Example**:
```rust
let inputs = vec!["video1.mp4", "video2.mp4"];
let results = AudioExtractor::extract_batch(
    inputs,
    "output/",
    AudioFormat::Mp3,
    128,
    false
)?;
```

#### Standalone File Verification
```rust
pub fn verify_audio_file_standalone(file_path: &PathBuf) -> Result<AudioFileInfo>
```
Verifies audio files and returns detailed information.

**Parameters**:
- `file_path`: Audio file path

**Return Value**:
- `AudioFileInfo`: Audio file information

**Example**:
```rust
let info = AudioExtractor::verify_audio_file_standalone(&PathBuf::from("audio.mp3"))?;
println!("Format: {}", info.format);
```

#### Utility Functions
```rust
pub fn get_supported_video_formats() -> Vec<&'static str>
pub fn get_supported_audio_formats() -> Vec<AudioFormat>
```

## 🔄 Internal Methods

### Private Methods Overview
```rust
impl AudioExtractor {
    fn validate_input(&self) -> Result<()>
    fn is_video_file(&self, path: &PathBuf) -> bool
    fn create_output_directory(&self) -> Result<()>
    fn extract_audio(&self) -> Result<()>
    fn is_ffmpeg_available(&self) -> bool
    fn extract_audio_with_ffmpeg(&self) -> Result<()>
    fn extract_audio_fallback(&self) -> Result<()>
    fn get_video_info(&self) -> Result<VideoInfo>
    fn verify_audio_file(&self) -> Result<()>
    fn verify_audio_format(&self) -> Result<AudioFileInfo>
}
```

### Method Details

#### validate_input
Validates that the input file exists and is a supported video format.

#### is_video_file
Checks if the file extension is a supported video format.

#### create_output_directory
Creates the directory structure needed for the output file.

#### extract_audio
Main audio extraction logic that checks FFmpeg availability and selects appropriate extraction method.

#### is_ffmpeg_available
Checks if FFmpeg is installed on the system.

#### extract_audio_with_ffmpeg
Uses FFmpeg for actual audio extraction.

#### extract_audio_fallback
Fallback method when FFmpeg is not available.

#### get_video_info
Gets video file information (duration, whether it contains audio, etc.).

#### verify_audio_file
Verifies the output audio file.

#### verify_audio_format
Uses Symphonia library to parse audio file format.

## 📝 Usage Examples

### Basic Usage
```rust
use audio_extractor::{Args, AudioExtractor, AudioFormat};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args {
        input: PathBuf::from("video.mp4"),
        output: PathBuf::from("audio.mp3"),
        format: AudioFormat::Mp3,
        quality: 128,
        verify: true,
    };
    
    let extractor = AudioExtractor::new(args);
    extractor.extract()?;
    
    Ok(())
}
```

### With Progress Display
```rust
use audio_extractor::{Args, AudioExtractor, AudioFormat};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args {
        input: PathBuf::from("video.mp4"),
        output: PathBuf::from("audio.mp3"),
        format: AudioFormat::Mp3,
        quality: 192,
        verify: true,
    };
    
    let extractor = AudioExtractor::new(args);
    extractor.extract_with_progress(|msg| {
        println!("📄 {}", msg);
    })?;
    
    Ok(())
}
```

### Batch Processing
```rust
use audio_extractor::{AudioExtractor, AudioFormat};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = vec![
        Path::new("video1.mp4"),
        Path::new("video2.mp4"),
        Path::new("video3.mp4"),
    ];
    
    let results = AudioExtractor::extract_batch(
        inputs,
        Path::new("output/"),
        AudioFormat::Mp3,
        128,
        false
    )?;
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(path) => println!("✅ File {} extracted to {:?}", i + 1, path),
            Err(e) => println!("❌ File {} failed: {}", i + 1, e),
        }
    }
    
    Ok(())
}
```

### File Verification
```rust
use audio_extractor::AudioExtractor;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_file = PathBuf::from("audio.mp3");
    
    match AudioExtractor::verify_audio_file_standalone(&audio_file) {
        Ok(info) => {
            println!("✅ Audio file verified successfully!");
            println!("Format: {}", info.format);
            if let Some(duration) = info.duration {
                println!("Duration: {:.2} seconds", duration);
            }
            if let Some(channels) = info.channels {
                println!("Channels: {}", channels);
            }
            if let Some(sample_rate) = info.sample_rate {
                println!("Sample Rate: {} Hz", sample_rate);
            }
        }
        Err(e) => {
            println!("❌ Verification failed: {}", e);
        }
    }
    
    Ok(())
}
```

## 🚨 Error Handling

### Error Types
All methods return `anyhow::Result<T>`, providing rich error context.

### Common Errors
- `Input file does not exist`
- `Input file is not a supported video format`
- `FFmpeg not found`
- `Failed to create output directory`
- `Audio extraction failed`

### Error Handling Example
```rust
match extractor.extract() {
    Ok(()) => println!("✅ Extraction successful"),
    Err(e) => {
        eprintln!("❌ Extraction failed: {}", e);
        // Handle specific error types
        if e.to_string().contains("FFmpeg not found") {
            eprintln!("Please install FFmpeg first");
        }
    }
}
```

## 🧪 Testing Support

### Unit Tests
The project includes a comprehensive unit test suite:
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration_tests
```

### Benchmark Tests
```bash
cargo bench
```

## 🔧 Developer Guide

### Adding New Audio Formats
1. Add new format to `AudioFormat` enum
2. Add string representation in `Display` implementation
3. Add FFmpeg parameter configuration in `extract_audio_with_ffmpeg` method

### Extending Functionality
- Implement new methods for `AudioExtractor`
- Add corresponding test cases
- Update documentation

### Contributing Code
1. Fork the project
2. Create feature branch
3. Write tests
4. Submit Pull Request
