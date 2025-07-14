use clap::{Parser, ValueEnum};
use anyhow::{Result, Context};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input video file path
    #[arg(short, long)]
    pub input: PathBuf,
    
    /// Output audio file path
    #[arg(short, long)]
    pub output: PathBuf,
    
    /// Output audio format
    #[arg(short, long, default_value = "mp3")]
    pub format: AudioFormat,
    
    /// Audio quality (bitrate in kbps)
    #[arg(short, long, default_value = "128")]
    pub quality: u32,
}

#[derive(Clone, ValueEnum, Debug, PartialEq)]
pub enum AudioFormat {
    Mp3,
    Wav,
    Flac,
    Aac,
}

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioFormat::Mp3 => write!(f, "mp3"),
            AudioFormat::Wav => write!(f, "wav"),
            AudioFormat::Flac => write!(f, "flac"),
            AudioFormat::Aac => write!(f, "aac"),
        }
    }
}

pub struct AudioExtractor {
    args: Args,
}

impl AudioExtractor {
    pub fn new(args: Args) -> Self {
        Self { args }
    }
    
    pub fn extract(&self) -> Result<()> {
        self.validate_input()?;
        self.create_output_directory()?;
        self.extract_audio()?;
        Ok(())
    }
    
    fn validate_input(&self) -> Result<()> {
        if !self.args.input.exists() {
            anyhow::bail!("Input file does not exist: {:?}", self.args.input);
        }
        
        if !self.is_video_file(&self.args.input) {
            anyhow::bail!("Input file is not a supported video format: {:?}", self.args.input);
        }
        
        Ok(())
    }
    
    fn is_video_file(&self, path: &PathBuf) -> bool {
        if let Some(extension) = path.extension() {
            matches!(
                extension.to_str().unwrap_or("").to_lowercase().as_str(),
                "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm"
            )
        } else {
            false
        }
    }
    
    fn create_output_directory(&self) -> Result<()> {
        if let Some(parent) = self.args.output.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create output directory")?;
        }
        Ok(())
    }
    
    fn extract_audio(&self) -> Result<()> {
        // This is a simplified implementation
        // In a real implementation, you would use FFmpeg bindings
        println!("Extracting audio from {:?} to {:?}", self.args.input, self.args.output);
        println!("Format: {}, Quality: {} kbps", self.args.format, self.args.quality);
        
        // Simulate audio extraction
        std::fs::write(&self.args.output, b"fake audio data")
            .context("Failed to write output file")?;
        
        Ok(())
    }
    
    pub fn get_supported_video_formats() -> Vec<&'static str> {
        vec!["mp4", "avi", "mkv", "mov", "wmv", "flv", "webm"]
    }
    
    pub fn get_supported_audio_formats() -> Vec<AudioFormat> {
        vec![AudioFormat::Mp3, AudioFormat::Wav, AudioFormat::Flac, AudioFormat::Aac]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::{tempdir, NamedTempFile};
    use std::fs;

    fn create_test_args(input: PathBuf, output: PathBuf) -> Args {
        Args {
            input,
            output,
            format: AudioFormat::Mp3,
            quality: 128,
        }
    }

    fn create_test_video_file() -> NamedTempFile {
        let file = NamedTempFile::with_suffix(".mp4").unwrap();
        // Write some dummy video data
        fs::write(file.path(), b"fake video data").unwrap();
        file
    }

    #[test]
    fn test_audio_format_display() {
        assert_eq!(AudioFormat::Mp3.to_string(), "mp3");
        assert_eq!(AudioFormat::Wav.to_string(), "wav");
        assert_eq!(AudioFormat::Flac.to_string(), "flac");
        assert_eq!(AudioFormat::Aac.to_string(), "aac");
    }

    #[test]
    fn test_audio_format_debug() {
        assert_eq!(format!("{:?}", AudioFormat::Mp3), "Mp3");
        assert_eq!(format!("{:?}", AudioFormat::Wav), "Wav");
    }

    #[test]
    fn test_audio_format_equality() {
        assert_eq!(AudioFormat::Mp3, AudioFormat::Mp3);
        assert_ne!(AudioFormat::Mp3, AudioFormat::Wav);
    }

    #[test]
    fn test_audio_extractor_creation() {
        let temp_input = create_test_video_file();
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.mp3");
        
        let args = create_test_args(temp_input.path().to_path_buf(), output_path);
        let extractor = AudioExtractor::new(args);
        
        // Test that the extractor is created successfully
        assert_eq!(extractor.args.format, AudioFormat::Mp3);
        assert_eq!(extractor.args.quality, 128);
    }

    #[test]
    fn test_validate_input_existing_file() {
        let temp_input = create_test_video_file();
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.mp3");
        
        let args = create_test_args(temp_input.path().to_path_buf(), output_path);
        let extractor = AudioExtractor::new(args);
        
        // Should succeed for existing video file
        assert!(extractor.validate_input().is_ok());
    }

    #[test]
    fn test_validate_input_nonexistent_file() {
        let input_path = PathBuf::from("/nonexistent/file.mp4");
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.mp3");
        
        let args = create_test_args(input_path, output_path);
        let extractor = AudioExtractor::new(args);
        
        // Should fail for non-existent file
        assert!(extractor.validate_input().is_err());
    }

    #[test]
    fn test_validate_input_unsupported_format() {
        let temp_file = NamedTempFile::with_suffix(".txt").unwrap();
        fs::write(temp_file.path(), b"not a video file").unwrap();
        
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.mp3");
        
        let args = create_test_args(temp_file.path().to_path_buf(), output_path);
        let extractor = AudioExtractor::new(args);
        
        // Should fail for unsupported format
        assert!(extractor.validate_input().is_err());
    }

    #[test]
    fn test_is_video_file_supported_formats() {
        let temp_dir = tempdir().unwrap();
        let args = create_test_args(
            temp_dir.path().join("test.mp4"),
            temp_dir.path().join("output.mp3")
        );
        let extractor = AudioExtractor::new(args);
        
        // Test supported video formats
        assert!(extractor.is_video_file(&PathBuf::from("test.mp4")));
        assert!(extractor.is_video_file(&PathBuf::from("test.avi")));
        assert!(extractor.is_video_file(&PathBuf::from("test.mkv")));
        assert!(extractor.is_video_file(&PathBuf::from("test.mov")));
        assert!(extractor.is_video_file(&PathBuf::from("test.wmv")));
        assert!(extractor.is_video_file(&PathBuf::from("test.flv")));
        assert!(extractor.is_video_file(&PathBuf::from("test.webm")));
        
        // Test case insensitivity
        assert!(extractor.is_video_file(&PathBuf::from("test.MP4")));
        assert!(extractor.is_video_file(&PathBuf::from("test.AVI")));
    }

    #[test]
    fn test_is_video_file_unsupported_formats() {
        let temp_dir = tempdir().unwrap();
        let args = create_test_args(
            temp_dir.path().join("test.mp4"),
            temp_dir.path().join("output.mp3")
        );
        let extractor = AudioExtractor::new(args);
        
        // Test unsupported formats
        assert!(!extractor.is_video_file(&PathBuf::from("test.txt")));
        assert!(!extractor.is_video_file(&PathBuf::from("test.mp3")));
        assert!(!extractor.is_video_file(&PathBuf::from("test.jpg")));
        assert!(!extractor.is_video_file(&PathBuf::from("test")));
    }

    #[test]
    fn test_create_output_directory() {
        let temp_input = create_test_video_file();
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("nested").join("directory").join("output.mp3");
        
        let args = create_test_args(temp_input.path().to_path_buf(), output_path.clone());
        let extractor = AudioExtractor::new(args);
        
        // Directory should not exist initially
        assert!(!output_path.parent().unwrap().exists());
        
        // Should create directory successfully
        assert!(extractor.create_output_directory().is_ok());
        assert!(output_path.parent().unwrap().exists());
    }

    #[test]
    fn test_extract_audio_success() {
        let temp_input = create_test_video_file();
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.mp3");
        
        let args = create_test_args(temp_input.path().to_path_buf(), output_path.clone());
        let extractor = AudioExtractor::new(args);
        
        // Should extract audio successfully
        assert!(extractor.extract().is_ok());
        
        // Output file should exist
        assert!(output_path.exists());
        
        // Output file should contain the fake audio data
        let content = fs::read_to_string(&output_path).unwrap();
        assert_eq!(content, "fake audio data");
    }

    #[test]
    fn test_extract_with_different_formats() {
        let temp_input = create_test_video_file();
        let temp_dir = tempdir().unwrap();
        
        let formats = vec![
            (AudioFormat::Mp3, "output.mp3"),
            (AudioFormat::Wav, "output.wav"),
            (AudioFormat::Flac, "output.flac"),
            (AudioFormat::Aac, "output.aac"),
        ];
        
        for (format, filename) in formats {
            let output_path = temp_dir.path().join(filename);
            let mut args = create_test_args(temp_input.path().to_path_buf(), output_path.clone());
            args.format = format;
            
            let extractor = AudioExtractor::new(args);
            assert!(extractor.extract().is_ok());
            assert!(output_path.exists());
        }
    }

    #[test]
    fn test_extract_with_different_qualities() {
        let temp_input = create_test_video_file();
        let temp_dir = tempdir().unwrap();
        
        let qualities = vec![64, 128, 192, 256, 320];
        
        for quality in qualities {
            let output_path = temp_dir.path().join(format!("output_{}.mp3", quality));
            let mut args = create_test_args(temp_input.path().to_path_buf(), output_path.clone());
            args.quality = quality;
            
            let extractor = AudioExtractor::new(args);
            assert!(extractor.extract().is_ok());
            assert!(output_path.exists());
        }
    }

    #[test]
    fn test_get_supported_video_formats() {
        let formats = AudioExtractor::get_supported_video_formats();
        assert_eq!(formats.len(), 7);
        assert!(formats.contains(&"mp4"));
        assert!(formats.contains(&"avi"));
        assert!(formats.contains(&"mkv"));
        assert!(formats.contains(&"mov"));
        assert!(formats.contains(&"wmv"));
        assert!(formats.contains(&"flv"));
        assert!(formats.contains(&"webm"));
    }

    #[test]
    fn test_get_supported_audio_formats() {
        let formats = AudioExtractor::get_supported_audio_formats();
        assert_eq!(formats.len(), 4);
        assert!(formats.contains(&AudioFormat::Mp3));
        assert!(formats.contains(&AudioFormat::Wav));
        assert!(formats.contains(&AudioFormat::Flac));
        assert!(formats.contains(&AudioFormat::Aac));
    }

    #[test]
    fn test_extract_nonexistent_input() {
        let input_path = PathBuf::from("/nonexistent/video.mp4");
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.mp3");
        
        let args = create_test_args(input_path, output_path);
        let extractor = AudioExtractor::new(args);
        
        // Should fail when input file doesn't exist
        assert!(extractor.extract().is_err());
    }

    #[test]
    fn test_extract_invalid_input_format() {
        let temp_file = NamedTempFile::with_suffix(".txt").unwrap();
        fs::write(temp_file.path(), b"not a video").unwrap();
        
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.mp3");
        
        let args = create_test_args(temp_file.path().to_path_buf(), output_path);
        let extractor = AudioExtractor::new(args);
        
        // Should fail for invalid input format
        assert!(extractor.extract().is_err());
    }

    #[test]
    fn test_audio_format_clone() {
        let format1 = AudioFormat::Mp3;
        let format2 = format1.clone();
        assert_eq!(format1, format2);
    }

    #[test]
    fn test_args_structure() {
        let temp_input = create_test_video_file();
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.mp3");
        
        let args = Args {
            input: temp_input.path().to_path_buf(),
            output: output_path.clone(),
            format: AudioFormat::Wav,
            quality: 256,
        };
        
        assert_eq!(args.input, temp_input.path());
        assert_eq!(args.output, output_path);
        assert_eq!(args.format, AudioFormat::Wav);
        assert_eq!(args.quality, 256);
    }
}

// Integration tests
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::{tempdir, NamedTempFile};
    use std::fs;

    #[test]
    fn test_full_workflow() {
        // Create a temporary video file
        let temp_input = NamedTempFile::with_suffix(".mp4").unwrap();
        fs::write(temp_input.path(), b"fake video content").unwrap();
        
        // Create output directory
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("extracted_audio.mp3");
        
        // Create args
        let args = Args {
            input: temp_input.path().to_path_buf(),
            output: output_path.clone(),
            format: AudioFormat::Mp3,
            quality: 192,
        };
        
        // Create extractor and run full workflow
        let extractor = AudioExtractor::new(args);
        let result = extractor.extract();
        
        // Verify success
        assert!(result.is_ok());
        assert!(output_path.exists());
        
        // Verify content
        let content = fs::read_to_string(&output_path).unwrap();
        assert_eq!(content, "fake audio data");
    }

    #[test]
    fn test_multiple_extractions() {
        let temp_input = NamedTempFile::with_suffix(".mkv").unwrap();
        fs::write(temp_input.path(), b"fake video content").unwrap();
        
        let temp_dir = tempdir().unwrap();
        
        // Test multiple formats
        let test_cases = vec![
            (AudioFormat::Mp3, "test1.mp3", 128),
            (AudioFormat::Wav, "test2.wav", 256),
            (AudioFormat::Flac, "test3.flac", 320),
            (AudioFormat::Aac, "test4.aac", 192),
        ];
        
        for (format, filename, quality) in test_cases {
            let output_path = temp_dir.path().join(filename);
            let args = Args {
                input: temp_input.path().to_path_buf(),
                output: output_path.clone(),
                format,
                quality,
            };
            
            let extractor = AudioExtractor::new(args);
            assert!(extractor.extract().is_ok());
            assert!(output_path.exists());
        }
    }

    #[test]
    fn test_error_handling_chain() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.mp3");
        
        // Test with non-existent input
        let args1 = Args {
            input: PathBuf::from("/definitely/does/not/exist.mp4"),
            output: output_path.clone(),
            format: AudioFormat::Mp3,
            quality: 128,
        };
        
        let extractor1 = AudioExtractor::new(args1);
        assert!(extractor1.extract().is_err());
        
        // Test with unsupported format
        let temp_file = NamedTempFile::with_suffix(".doc").unwrap();
        fs::write(temp_file.path(), b"document content").unwrap();
        
        let args2 = Args {
            input: temp_file.path().to_path_buf(),
            output: output_path,
            format: AudioFormat::Mp3,
            quality: 128,
        };
        
        let extractor2 = AudioExtractor::new(args2);
        assert!(extractor2.extract().is_err());
    }
}
