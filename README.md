# Introduction
This repository is a tool for extracting and saving audio file from a video file. It is designed to be simple and efficient, allowing users to quickly convert video files into audio formats. The code is written in rust to ensure performance and reliability.

# Features
- Extract audio from various video formats
- Save audio in multiple formats (e.g., MP3, WAV)
- Easy to use command-line interface
- High performance due to Rust's efficiency

# Installation

## Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

## Install from Source
```bash
# Clone the repository
git clone https://github.com/your-username/audio_extractor.git
cd audio_extractor

# Build the project
cargo build --release

# The binary will be available at target/release/audio_extractor
```

## Install using Cargo
```bash
# Install from crates.io (when published)
cargo install audio_extractor

# Or install from local directory
cargo install --path .
```

# Usage

## Basic Usage
```bash
audio_extractor --input <VIDEO_FILE> --output <AUDIO_FILE>
```

## Command Line Options

### Required Arguments
- `-i, --input <INPUT>`: Input video file path
- `-o, --output <OUTPUT>`: Output audio file path

### Optional Arguments
- `-f, --format <FORMAT>`: Output audio format [default: mp3]
  - Supported formats: `mp3`, `wav`, `flac`, `aac`
- `-q, --quality <QUALITY>`: Audio quality (bitrate in kbps) [default: 128]
  - Common values: 64, 128, 192, 256, 320
- `-h, --help`: Print help information
- `-V, --version`: Print version information

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

### Running Tests
```bash
# Run all tests
./run_tests.sh

# Run specific test types
cargo test --lib          # Unit tests
cargo test --test cli_tests   # CLI tests
cargo bench               # Performance tests
```

# License
This project is licensed under the Apache License 2.0. You can freely use, modify, and distribute the code as long as you comply with the terms of the license.

# Acknowledgments
This project is helped by the Copilot agent

