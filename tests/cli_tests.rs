use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::{tempdir, NamedTempFile};
use std::fs;

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
    let temp_input = NamedTempFile::with_suffix(".mp4").unwrap();
    fs::write(temp_input.path(), b"fake video data").unwrap();
    
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
    let temp_input = NamedTempFile::with_suffix(".avi").unwrap();
    fs::write(temp_input.path(), b"fake video data").unwrap();
    
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
    let temp_input = NamedTempFile::with_suffix(".mkv").unwrap();
    fs::write(temp_input.path(), b"fake video data").unwrap();
    
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
    let temp_input = NamedTempFile::with_suffix(".webm").unwrap();
    fs::write(temp_input.path(), b"fake video data").unwrap();
    
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
        let temp_input = NamedTempFile::with_suffix(".mov").unwrap();
        fs::write(temp_input.path(), b"fake video data").unwrap();
        
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
        let temp_input = NamedTempFile::with_suffix(".flv").unwrap();
        fs::write(temp_input.path(), b"fake video data").unwrap();
        
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
