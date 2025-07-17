use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::{tempdir, NamedTempFile};
use std::fs;
use std::path::PathBuf;
use audio_extractor::{Args, AudioFormat, AudioExtractor};

mod common;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A tool for extracting and saving audio files from video files"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("audio_extractor"));
}

#[test]
fn test_cli_missing_arguments() {
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_cli_successful_extraction() {
    // Create a temporary video file
    let temp_input = common::create_test_video_file();
    
    // Create output directory
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.mp3");
    
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("--input")
        .arg(temp_input.path())
        .arg("--output")
        .arg(&output_path);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Audio extraction completed successfully"));
    
    // Verify output file was created
    assert!(output_path.exists());
}

#[test]
fn test_cli_with_format_option() {
    let temp_input = common::create_test_video_file();
    
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.wav");
    
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("--input")
        .arg(temp_input.path())
        .arg("--output")
        .arg(&output_path)
        .arg("--format")
        .arg("wav");
    
    cmd.assert().success();
    assert!(output_path.exists());
}

#[test]
fn test_cli_with_quality_option() {
    let temp_input = common::create_test_video_file();
    
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.mp3");
    
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("--input")
        .arg(temp_input.path())
        .arg("--output")
        .arg(&output_path)
        .arg("--quality")
        .arg("192");
    
    cmd.assert().success();
    assert!(output_path.exists());
}

#[test]
fn test_cli_nonexistent_input() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.mp3");
    
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("--input")
        .arg("/nonexistent/file.mp4")
        .arg("--output")
        .arg(&output_path);
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Input file does not exist"));
}

#[test]
fn test_cli_invalid_format() {
    let temp_input = NamedTempFile::with_suffix(".txt").unwrap();
    fs::write(temp_input.path(), b"not a video file").unwrap();
    
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.mp3");
    
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("--input")
        .arg(temp_input.path())
        .arg("--output")
        .arg(&output_path);
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not a supported video format"));
}

#[test]
fn test_cli_short_flags() {
    let temp_input = common::create_test_video_file();
    
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.aac");
    
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("-i")
        .arg(temp_input.path())
        .arg("-o")
        .arg(&output_path)
        .arg("-f")
        .arg("aac")
        .arg("-q")
        .arg("256");
    
    cmd.assert().success();
    assert!(output_path.exists());
}

#[test]
fn test_cli_all_supported_formats() {
    let formats = vec!["mp3", "wav", "flac", "aac"];
    
    for format in formats {
        let temp_input = common::create_test_video_file();
        
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join(format!("output.{}", format));
        
        let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
        cmd.arg("--input")
            .arg(temp_input.path())
            .arg("--output")
            .arg(&output_path)
            .arg("--format")
            .arg(format);
        
        cmd.assert().success();
        assert!(output_path.exists());
    }
}

#[test]
fn test_cli_various_quality_settings() {
    let qualities = vec!["64", "128", "192", "256", "320"];
    
    for quality in qualities {
        let temp_input = common::create_test_video_file();
        
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join(format!("output_{}.mp3", quality));
        
        let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
        cmd.arg("--input")
            .arg(temp_input.path())
            .arg("--output")
            .arg(&output_path)
            .arg("--quality")
            .arg(quality);
        
        cmd.assert().success();
        assert!(output_path.exists());
    }
}

#[test]
fn test_cli_with_verify_option() {
    // Create a temporary video file
    let temp_input = common::create_test_video_file();
    
    // Create output directory
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.mp3");
    
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("--input")
        .arg(temp_input.path())
        .arg("--output")
        .arg(&output_path)
        .arg("--verify");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Verification: enabled"))
        .stdout(predicate::str::contains("Verifying audio file"))
        .stdout(predicate::str::contains("Basic file validation passed"));
}

#[test]
fn test_cli_help_shows_verify_option() {
    let mut cmd = Command::cargo_bin("audio_extractor").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--verify"))
        .stdout(predicate::str::contains("Verify the output audio file after extraction"));
}

#[test]
fn test_full_workflow() {
    // Create a temporary video file
    let temp_input = common::create_test_video_file();
    
    // Create output directory
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("extracted_audio.mp3");
    
    // Create args
    let args = Args {
        input: temp_input.path().to_path_buf(),
        output: output_path.clone(),
        format: AudioFormat::Mp3,
        quality: 192,
        verify: false,
    };
    
    // Create extractor and run full workflow
    let extractor = AudioExtractor::new(args);
    let result = extractor.extract();
    
    // Verify success
    assert!(result.is_ok());
    assert!(output_path.exists());
    
    // Verify the file is not empty
    let metadata = fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0);
}

#[test]
fn test_multiple_extractions() {
    let temp_input = common::create_test_video_file();
    
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
        let mut args = common::create_test_args(temp_input.path().to_path_buf(), output_path.clone());
        args.format = format;
        args.quality = quality;
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
        verify: false,
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
        verify: false,
    };
    
    let extractor2 = AudioExtractor::new(args2);
    assert!(extractor2.extract().is_err());
}