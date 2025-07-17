# 🎵 Audio Extractor

> **Note:** This project is an experiment to test the capabilities of the Gemini CLI and other AI-powered development tools. The code is not intended for production use.

A high-performance audio extraction tool that uses FFmpeg to extract audio from video files. Supports multiple output formats with batch processing, progress display, and audio verification features.

![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)

## ✨ Key Features

- 🎯 **Real Audio Extraction**: Uses FFmpeg to extract high-quality audio from video files
- 🎵 **Multi-format Support**: MP3, WAV, FLAC, AAC and other output formats
- 🔧 **Quality Control**: Configurable bitrate (64-320 kbps)
- 📊 **Progress Display**: Real-time extraction progress and status display
- 🔍 **Audio Verification**: Automatic audio file integrity verification after extraction
- 🚀 **Batch Processing**: Support for batch conversion of multiple video files
- 🛡️ **Error Handling**: Comprehensive error messages and recovery mechanisms
- 🎨 **User Friendly**: Clear output and status messages

## 🚀 Quick Start

### 📋 System Requirements

- Rust 1.70 or higher
- FFmpeg (for actual audio extraction)

### 🛠️ Installation

#### 1. Install FFmpeg
```bash
# macOS
brew install ffmpeg

# Ubuntu/Debian
sudo apt install ffmpeg

# Windows
# Download and install from https://ffmpeg.org/download.html
```

#### 2. Build Project
```bash
# Clone the project
git clone https://github.com/your-username/audio_extractor.git
cd audio_extractor

# Build release version
cargo build --release
```

### 💡 Basic Usage

```bash
# Extract MP3 audio from MP4 video
./target/release/audio_extractor -i video.mp4 -o audio.mp3 -f mp3 -q 128

# Extract high-quality FLAC audio and verify
./target/release/audio_extractor -i video.mp4 -o audio.flac -f flac -q 320 --verify

# Extract WAV audio
./target/release/audio_extractor -i video.mp4 -o audio.wav -f wav

# Extract AAC audio
./target/release/audio_extractor -i video.mp4 -o audio.aac -f aac -q 192
```

### 🎨 Demo Programs

Experience the full functionality:
```bash
# Complete feature demonstration
./target/release/feature_demo

# Batch processing demonstration
./target/release/batch_demo

# Audio verification tool
./target/release/verify_audio audio.mp3
```

## 🎯 Supported Formats

### Input Video Formats
- MP4 (.mp4)
- AVI (.avi)
- MKV (.mkv)
- MOV (.mov)
- WMV (.wmv)
- FLV (.flv)
- WebM (.webm)

### Output Audio Formats
- **MP3** - Lossy compression, widely compatible
- **WAV** - Lossless format, larger file size
- **FLAC** - Lossless compression, balanced file size and quality
- **AAC** - Modern lossy compression, high efficiency

## 🔧 Command Line Arguments

```bash
audio_extractor [OPTIONS] --input <PATH> --output <PATH> --format <FORMAT>
```

### Required Arguments
- `-i, --input <PATH>`: Input video file path
- `-o, --output <PATH>`: Output audio file path
- `-f, --format <FORMAT>`: Output audio format (mp3, wav, flac, aac)

### Optional Arguments
- `-q, --quality <BITRATE>`: Audio quality (bitrate in kbps)
- `--verify`: Verify audio file after extraction
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## 🚀 Performance Features

- **Quality Control**: Supports 64-320 kbps bitrate settings
- **Format Efficiency**: 
  - MP3: Balanced compression and compatibility
  - AAC: More efficient compression algorithm
  - FLAC: Lossless compression, approximately 50% file size
  - WAV: Lossless original format
- **Processing Speed**: Direct FFmpeg calls, fully utilizing system performance
- **Memory Efficiency**: Streaming processing, doesn't load entire file into memory

## 📊 Example Output

```
🎵 Audio Extractor - Complete Feature Demo
===========================================

📹 Test video: sample-15s.mp4
📁 Creating output directory...

🎯 Demo 1: Basic Audio Extraction
==================================
  📄 Starting audio extraction...
  📄 Input validation completed
  📄 Output directory prepared
  📄 Video duration: 15.63 seconds
  📄 Audio extraction completed
  📄 Starting verification...
  ✅ Basic extraction completed!

🎯 Demo 2: Different Audio Formats
===================================
  🎵 Extracting Standard MP3 (mp3) - ✅ Success! Size: 251339 bytes
  🎵 Extracting Lossless WAV (wav) - ✅ Success! Size: 2756686 bytes
  🎵 Extracting Lossless FLAC (flac) - ✅ Success! Size: 2273348 bytes
  🎵 Extracting High Quality AAC (aac) - ✅ Success! Size: 368307 bytes
```

## 🔧 Parameter Reference

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `-i, --input` | Required | Input video file path | `-i video.mp4` |
| `-o, --output` | Required | Output audio file path | `-o audio.mp3` |
| `-f, --format` | Optional | Output audio format | `-f mp3` (default) |
| `-q, --quality` | Optional | Audio quality (bitrate) | `-q 128` (default) |
| `--verify` | Optional | Verify audio file after extraction | `--verify` |

## 🎮 Demo Programs

### 1. Complete Feature Demo
```bash
./target/release/feature_demo
```
Demonstrates all features including different formats, quality settings, and verification.

### 2. Batch Processing Demo
```bash
./target/release/batch_demo
```
Shows how to process multiple video files in batch.

### 3. Audio Verification Tool
```bash
./target/release/examples/verify_audio audio.mp3
```
Verifies audio files and displays detailed information.

## 📊 Performance Characteristics

- **Efficient Processing**: Direct FFmpeg calls, fully utilizing system performance
- **Memory Optimization**: Streaming processing, doesn't load entire file into memory
- **Parallel Support**: Supports parallel operations during batch processing
- **Quality Control**: Supports 64-320 kbps bitrate settings

## 📄 License

This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.

## 🆘 Support

If you encounter problems or have suggestions, please:
1. Check the [FAQ](docs/FAQ.md)

## Examples

### Basic Audio Extraction
```bash
# Extract audio to MP3 with default settings
audio_extractor -i video.mp4 -o audio.mp3
```

### Specify Output Format
```bash
# Extract to WAV format
audio_extractor -i video.mkv -o audio.wav --format wav

# Extract to FLAC format
audio_extractor -i video.avi -o audio.flac -f flac

# Extract to AAC format
audio_extractor -i video.mov -o audio.aac --format aac
```

### Custom Quality Settings
```bash
# High quality MP3 (320 kbps)
audio_extractor -i video.mp4 -o audio.mp3 --quality 320

# Low quality for smaller file size (64 kbps)
audio_extractor -i video.webm -o audio.mp3 -q 64

# CD quality (192 kbps)
audio_extractor -i video.flv -o audio.mp3 --quality 192
```

### Audio File Verification
```bash
# Extract audio and verify the output file
audio_extractor -i video.mp4 -o audio.mp3 --verify

# Extract to high quality and verify
audio_extractor -i video.mkv -o audio.flac -f flac -q 320 --verify

# Verify shows detailed information about the audio file
audio_extractor -i input.avi -o output.wav --format wav --verify
```

### Batch Processing with Shell Scripts
```bash
#!/bin/bash
# Convert all MP4 files in current directory to MP3
for file in *.mp4; do
    if [ -f "$file" ]; then
        output="${file%.mp4}.mp3"
        audio_extractor -i "$file" -o "$output" -q 192
        echo "Converted: $file -> $output"
    fi
done
```

## Supported Formats

### Input Video Formats
- **MP4** (`.mp4`) - MPEG-4 Video
- **AVI** (`.avi`) - Audio Video Interleave
- **MKV** (`.mkv`) - Matroska Video
- **MOV** (`.mov`) - QuickTime Movie
- **WMV** (`.wmv`) - Windows Media Video
- **FLV** (`.flv`) - Flash Video
- **WebM** (`.webm`) - WebM Video

### Output Audio Formats
| Format | Extension | Description | Quality Range |
|--------|-----------|-------------|---------------|
| **MP3** | `.mp3` | MPEG Audio Layer III | 64-320 kbps |
| **WAV** | `.wav` | Waveform Audio File | Lossless |
| **FLAC** | `.flac` | Free Lossless Audio Codec | Lossless |
| **AAC** | `.aac` | Advanced Audio Coding | 64-320 kbps |

## Quality Guidelines

### Bitrate Recommendations
- **64 kbps**: Very low quality, suitable for voice recordings
- **128 kbps**: Standard quality, good for most purposes
- **192 kbps**: High quality, recommended for music
- **256 kbps**: Very high quality, excellent for audiophiles
- **320 kbps**: Maximum quality for lossy formats

### Format Selection Guide
- **MP3**: Universal compatibility, good compression
- **WAV**: Uncompressed, largest file size, best compatibility
- **FLAC**: Lossless compression, smaller than WAV
- **AAC**: Better compression than MP3, good quality

## Performance Tips

### For Large Files
```bash
# Use lower quality for faster processing
audio_extractor -i large_video.mkv -o audio.mp3 -q 128

# Use efficient formats
audio_extractor -i video.mp4 -o audio.aac -f aac -q 192
```

### For Batch Processing
```bash
# Process multiple files in parallel (using GNU parallel)
find . -name "*.mp4" | parallel audio_extractor -i {} -o {}.mp3 -q 192
```

## Troubleshooting

### Common Issues

#### "Input file does not exist"
```bash
# Check file path
ls -la /path/to/video.mp4

# Use absolute path
audio_extractor -i /full/path/to/video.mp4 -o output.mp3
```

#### "Input file is not a supported video format"
```bash
# Check file extension
file video.unknown

# Rename file with correct extension
mv video.unknown video.mp4
```

#### "Failed to create output directory"
```bash
# Create output directory manually
mkdir -p /path/to/output/directory

# Check permissions
ls -ld /path/to/output/
```

#### "Audio file verification failed"
```bash
# Check if output file exists
ls -la output.mp3

# Try without verification first
audio_extractor -i video.mp4 -o audio.mp3

# Then verify manually if needed
audio_extractor -i video.mp4 -o audio.mp3 --verify
```

### Error Codes
- **Exit code 0**: Success
- **Exit code 1**: General error (invalid arguments, file not found, etc.)
- **Exit code 101**: Compilation or dependency error

### Getting Help
```bash
# Show help
audio_extractor --help

# Show version
audio_extractor --version

# Run tests to verify installation
cargo test
```

## Advanced Usage

## Advanced Usage

### Audio File Verification

The `--verify` flag enables comprehensive validation of the extracted audio file:

```bash
# Extract and verify in one command
audio_extractor -i video.mp4 -o audio.mp3 --verify

# The verification process includes:
# 1. File existence check
# 2. File size validation  
# 3. Audio format validation
# 4. Metadata extraction (when possible)
```

### Verification Output Example
When verification is enabled, you'll see output like:
```
Audio Extractor v0.1.0
Input: "video.mp4"
Output: "audio.mp3"
Format: mp3
Quality: 128 kbps
Verification: enabled

Extracting audio from "video.mp4" to "audio.mp3"
Format: mp3, Quality: 128 kbps
Verifying audio file: "audio.mp3"
✓ Basic file validation passed!
  - File exists: "audio.mp3"
  - File size: 1048576 bytes
✓ Audio format validation successful!
  - Format: mp3
  - Duration: 180.45 seconds
  - Channels: 2
  - Sample rate: 44100 Hz
Audio extraction completed successfully!
```

### Configuration File (Future Feature)
```toml
# config.toml
[defaults]
format = "mp3"
quality = 192
output_dir = "./extracted_audio"

[formats.mp3]
quality_range = [64, 320]
default_quality = 192

[formats.flac]
compression_level = 5
```

### Integration with Other Tools
```bash
# Combine with ffprobe to get video info
ffprobe -v quiet -print_format json -show_format input.mp4
audio_extractor -i input.mp4 -o output.mp3

# Use with find for recursive processing
find /media/videos -name "*.mp4" -exec audio_extractor -i {} -o {}.mp3 \;
```

## Development

### Building from Source
```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code formatting
cargo fmt --check

# Run linting
cargo clippy
```

## 🧪 Testing

This project includes a comprehensive test suite to ensure code quality and correctness.

### Running Tests

To run all tests, use the following command:

```bash
cargo test
```

This will run unit tests, integration tests, and documentation tests.

### Test Structure

The tests are organized into the following directories:

*   `tests/`: Contains integration tests that test the public API of the `audio_extractor` crate.
*   `src/`: Contains unit tests that test individual functions and modules.

### Shared Test Utilities

The `tests/common.rs` file contains shared test utilities, such as the `create_test_video_file` function, which is used by both unit and integration tests. This helps to reduce code duplication and makes the tests easier to maintain.


## 🙏 Acknowledgments

This project was developed with the assistance of AI tools.


## Audio File Verification

The audio extractor includes a built-in verification feature that can validate the integrity and format of extracted audio files.

### Verification Features
- **File Existence Check**: Ensures the output file was created
- **File Size Validation**: Checks that the file is not empty
- **Format Validation**: Verifies the audio format is correct and readable
- **Metadata Extraction**: Displays detailed information about the audio file:
  - Audio format and codec
  - Duration (if available)
  - Number of channels
  - Sample rate

### Using Verification
```bash
# Enable verification with --verify flag
audio_extractor -i video.mp4 -o audio.mp3 --verify

# Example output:
# Extracting audio from "video.mp4" to "audio.mp3"
# Format: mp3, Quality: 128 kbps
# Verifying audio file: "audio.mp3"
# ✓ Audio file verification successful!
#   - File size: 1024567 bytes
#   - Format: mp3
#   - Duration: 180.45 seconds
#   - Channels: 2
#   - Sample rate: 44100 Hz
```

### Verification Error Handling
If verification fails, the tool will display helpful error messages:
```bash
# Empty file error
⚠ Audio file verification failed: Output audio file is empty

# Invalid format error
⚠ Audio file verification failed: Failed to probe audio file format

# Corrupted file error
⚠ Audio file verification failed: No default audio track found
```

