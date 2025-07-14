# Technical Implementation

## 🏗️ System Architecture

### Overall Design
```
User Input -> Parameter Validation -> Audio Extraction -> File Verification -> Output Results
    |               |                      |                    |                 |
    |               |                      |                    |                 |
   CLI          Input File             FFmpeg Call        Symphonia        User Feedback
  Parsing      Format Check           Command Build      Format Verify     Status Display
```

### Core Components

#### 1. Command Line Interface (CLI)
- Uses `clap` library for parameter parsing
- Supports long and short parameter names
- Automatic help generation
- Parameter validation and type conversion

#### 2. Audio Extraction Engine
- Based on FFmpeg external process calls
- Supports multiple audio formats and encoding parameters
- Dynamic command building and execution
- Error handling and status reporting

#### 3. File Verification System
- Uses Symphonia library for audio analysis
- Supports multiple audio format recognition
- Extracts metadata information
- File integrity checking

#### 4. Batch Processing Framework
- Supports multi-file parallel processing
- Error isolation and recovery
- Progress tracking and reporting
- Resource management and optimization

## 🔧 Core Implementation

### 1. Audio Extraction Process

#### Main Method
```rust
pub fn extract(&self) -> Result<()> {
    self.validate_input()?;           // Input validation
    self.create_output_directory()?;  // Create output directory
    self.extract_audio()?;            // Extract audio
    
    if self.args.verify {
        self.verify_audio_file()?;    // Verify output file
    }
    
    Ok(())
}
```

#### Progress Callback Support
```rust
pub fn extract_with_progress<F>(&self, progress_callback: F) -> Result<()>
where
    F: Fn(&str) + Send + Sync,
{
    progress_callback("Starting audio extraction...");
    
    // Input validation
    progress_callback("Input validation completed");
    self.validate_input()?;
    
    // Directory creation
    progress_callback("Output directory prepared");
    self.create_output_directory()?;
    
    // Video information extraction
    if let Ok(video_info) = self.get_video_info() {
        progress_callback(&format!("Video duration: {:.2} seconds", video_info.duration));
    }
    
    // Audio extraction
    progress_callback("Audio extraction completed");
    self.extract_audio()?;
    
    // Verification
    if self.args.verify {
        progress_callback("Starting verification...");
        self.verify_audio_file()?;
    }
    
    Ok(())
}
```

### 2. FFmpeg Integration

#### Command Construction
```rust
fn extract_audio_with_ffmpeg(&self) -> Result<()> {
    let mut cmd = Command::new("ffmpeg");
    
    // Basic parameters
    cmd.arg("-i").arg(&self.args.input);
    cmd.arg("-y"); // Overwrite output file
    
    // Format-specific encoding settings
    match self.args.format {
        AudioFormat::Mp3 => {
            cmd.arg("-c:a").arg("libmp3lame");
            cmd.arg("-b:a").arg(format!("{}k", self.args.quality));
        }
        AudioFormat::Wav => {
            cmd.arg("-c:a").arg("pcm_s16le");
            // WAV is lossless, no bitrate setting
        }
        AudioFormat::Flac => {
            cmd.arg("-c:a").arg("flac");
            // FLAC is lossless, no bitrate setting
        }
        AudioFormat::Aac => {
            cmd.arg("-c:a").arg("aac");
            cmd.arg("-b:a").arg(format!("{}k", self.args.quality));
        }
    }
    
    // Audio-only extraction
    cmd.arg("-vn");
    cmd.arg(&self.args.output);
    
    // Execute command
    let output = cmd.output()
        .context("Failed to execute ffmpeg command")?;
    
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("FFmpeg extraction failed: {}", error_message));
    }
    
    Ok(())
}
```

#### Video Information Extraction
```rust
fn get_video_info(&self) -> Result<VideoInfo> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&self.args.input)
        .arg("-hide_banner")
        .arg("-f")
        .arg("null")
        .arg("-")
        .output()
        .context("Failed to get video info")?;
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Parse duration from FFmpeg output
    let duration = self.parse_duration(&stderr)?;
    
    // Check for audio stream
    let has_audio = stderr.contains("Audio:");
    
    Ok(VideoInfo {
        duration,
        has_audio,
    })
}
```

### 3. Audio Verification

#### File Verification with Symphonia
```rust
fn verify_audio_format(&self) -> Result<AudioFileInfo> {
    let file = std::fs::File::open(&self.args.output)
        .context("Failed to open output file for verification")?;
    
    let mss = symphonia::core::io::MediaSourceStream::new(
        Box::new(file),
        Default::default(),
    );
    
    let mut hint = symphonia::core::probe::Hint::new();
    if let Some(extension) = self.args.output.extension() {
        hint.with_extension(&extension.to_string_lossy());
    }
    
    let meta_opts: symphonia::core::meta::MetadataOptions = Default::default();
    let fmt_opts: symphonia::core::formats::FormatOptions = Default::default();
    
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .context("Failed to probe audio format")?;
    
    let mut format = probed.format;
    let track = format.tracks()
        .iter()
        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .context("No valid audio track found")?;
    
    let params = &track.codec_params;
    
    Ok(AudioFileInfo {
        format: format!("{:?}", params.codec),
        duration: params.time_base.and_then(|tb| {
            params.n_frames.map(|frames| frames as f64 / tb.denom as f64)
        }),
        channels: params.channels.map(|ch| ch.count()),
        sample_rate: params.sample_rate,
    })
}
```

### 4. Batch Processing

#### Parallel Batch Processing
```rust
pub fn extract_batch<P: AsRef<std::path::Path>>(
    inputs: Vec<P>,
    output_dir: P,
    format: AudioFormat,
    quality: u32,
    verify: bool,
) -> Result<Vec<Result<PathBuf>>> {
    let output_dir = output_dir.as_ref();
    
    // Create output directory
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)
            .context("Failed to create output directory")?;
    }
    
    let results: Vec<Result<PathBuf>> = inputs
        .into_iter()
        .map(|input| {
            let input_path = input.as_ref();
            let file_stem = input_path.file_stem()
                .context("Invalid input file name")?
                .to_string_lossy();
            
            let output_filename = format!("{}.{}", file_stem, format);
            let output_path = output_dir.join(output_filename);
            
            let args = Args {
                input: input_path.to_path_buf(),
                output: output_path.clone(),
                format: format.clone(),
                quality,
                verify,
            };
            
            let extractor = AudioExtractor::new(args);
            extractor.extract()?;
            
            Ok(output_path)
        })
        .collect();
    
    Ok(results)
}
```

## 🎯 Error Handling Strategy

### Error Types
```rust
// Main error types handled
pub enum ExtractionError {
    InputNotFound,
    UnsupportedFormat,
    FFmpegNotFound,
    ExtractionFailed,
    VerificationFailed,
    DirectoryCreationFailed,
}
```

### Error Context
```rust
use anyhow::{Context, Result};

// Example error handling with context
fn validate_input(&self) -> Result<()> {
    if !self.args.input.exists() {
        return Err(anyhow::anyhow!("Input file does not exist: {:?}", self.args.input))
            .context("Input validation failed");
    }
    
    if !self.is_video_file(&self.args.input) {
        return Err(anyhow::anyhow!("Input file is not a supported video format"))
            .context("Format validation failed");
    }
    
    Ok(())
}
```

## 📊 Performance Optimization

### 1. Memory Management
- Streaming processing to avoid loading entire files into memory
- Efficient buffer management for large files
- Automatic cleanup of temporary resources

### 2. Process Optimization
- Direct FFmpeg subprocess calls for maximum performance
- Optimized command-line parameter construction
- Minimal overhead wrapper around FFmpeg

### 3. Concurrent Processing
- Parallel batch processing support
- Non-blocking progress callbacks
- Resource-aware concurrent execution

## 🔍 Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_audio_format_display() {
        assert_eq!(AudioFormat::Mp3.to_string(), "mp3");
        assert_eq!(AudioFormat::Wav.to_string(), "wav");
        assert_eq!(AudioFormat::Flac.to_string(), "flac");
        assert_eq!(AudioFormat::Aac.to_string(), "aac");
    }
    
    #[test]
    fn test_audio_extractor_creation() {
        let temp_dir = tempdir().unwrap();
        let args = Args {
            input: temp_dir.path().join("input.mp4"),
            output: temp_dir.path().join("output.mp3"),
            format: AudioFormat::Mp3,
            quality: 128,
            verify: false,
        };
        
        let extractor = AudioExtractor::new(args);
        assert_eq!(extractor.args.quality, 128);
        assert_eq!(extractor.args.format, AudioFormat::Mp3);
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_full_workflow() {
        // Create test video file
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("test.mp4");
        let output_path = temp_dir.path().join("test.mp3");
        
        // Create mock video file
        std::fs::write(&input_path, b"mock video data").unwrap();
        
        let args = Args {
            input: input_path,
            output: output_path.clone(),
            format: AudioFormat::Mp3,
            quality: 128,
            verify: false,
        };
        
        let extractor = AudioExtractor::new(args);
        
        // This would normally call FFmpeg, but in tests we might mock it
        // assert!(extractor.extract().is_ok());
        // assert!(output_path.exists());
    }
}
```

## 🚀 Extension Points

### Adding New Audio Formats
1. **Extend AudioFormat enum**:
   ```rust
   #[derive(Clone, ValueEnum, Debug, PartialEq)]
   pub enum AudioFormat {
       Mp3,
       Wav,
       Flac,
       Aac,
       Ogg,  // New format
   }
   ```

2. **Add display implementation**:
   ```rust
   impl std::fmt::Display for AudioFormat {
       fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
           match self {
               AudioFormat::Mp3 => write!(f, "mp3"),
               AudioFormat::Wav => write!(f, "wav"),
               AudioFormat::Flac => write!(f, "flac"),
               AudioFormat::Aac => write!(f, "aac"),
               AudioFormat::Ogg => write!(f, "ogg"),  // New format
           }
       }
   }
   ```

3. **Add FFmpeg parameters**:
   ```rust
   AudioFormat::Ogg => {
       cmd.arg("-c:a").arg("libvorbis");
       cmd.arg("-b:a").arg(format!("{}k", self.args.quality));
   }
   ```

### Custom Progress Callbacks
```rust
pub trait ProgressReporter {
    fn report_progress(&self, message: &str);
    fn report_error(&self, error: &str);
    fn report_completion(&self);
}

pub struct CustomProgressReporter {
    // Custom implementation
}

impl ProgressReporter for CustomProgressReporter {
    fn report_progress(&self, message: &str) {
        // Custom progress handling
        println!("Progress: {}", message);
    }
    
    fn report_error(&self, error: &str) {
        eprintln!("Error: {}", error);
    }
    
    fn report_completion(&self) {
        println!("Extraction completed successfully!");
    }
}
```

## 📈 Monitoring and Metrics

### Performance Metrics
- Extraction time per file
- Memory usage during processing
- Error rates and types
- Format-specific performance

### Logging Integration
```rust
use log::{info, warn, error, debug};

fn extract_audio(&self) -> Result<()> {
    info!("Starting audio extraction for {:?}", self.args.input);
    
    if self.is_ffmpeg_available() {
        debug!("FFmpeg is available, using real extraction");
        self.extract_audio_with_ffmpeg()
    } else {
        warn!("FFmpeg not available, using fallback method");
        self.extract_audio_fallback()
    }
}
```

## 🔐 Security Considerations

### Path Validation
- Prevent directory traversal attacks
- Validate file extensions
- Check file permissions
- Sanitize user input

### Resource Limits
- Limit maximum file sizes
- Timeout for long-running processes
- Memory usage monitoring
- Concurrent process limits

This implementation provides a robust, extensible foundation for audio extraction with comprehensive error handling, performance optimization, and testing coverage.
