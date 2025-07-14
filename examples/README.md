# Examples

This directory contains example scripts and configuration files for the audio extractor tool.

## Files

### `batch_convert.sh`
A shell script that demonstrates how to batch process multiple video files in a directory.

**Usage:**
```bash
# Make sure you have MP4 files in the current directory
# Then run the script
./examples/batch_convert.sh
```

**Features:**
- Processes all MP4 files in the current directory
- Converts them to MP3 format with 192 kbps quality
- Shows progress and status for each file
- Handles errors gracefully

### `config.toml`
An example configuration file for future versions of the tool that will support configuration files.

**Features:**
- Default settings for format and quality
- Format-specific configurations
- Predefined presets for different use cases
- Quality range definitions

## Usage Examples

### Basic Audio Extraction
```bash
# Simple conversion
./target/release/audio_extractor -i video.mp4 -o audio.mp3

# With custom quality
./target/release/audio_extractor -i video.mp4 -o audio.mp3 --quality 320
```

### Different Formats
```bash
# WAV format (lossless)
./target/release/audio_extractor -i video.mkv -o audio.wav --format wav

# FLAC format (lossless, compressed)
./target/release/audio_extractor -i video.avi -o audio.flac -f flac

# AAC format (efficient compression)
./target/release/audio_extractor -i video.mov -o audio.aac --format aac
```

### Batch Processing
```bash
# Using the provided script
./examples/batch_convert.sh

# Manual batch processing with find
find . -name "*.mp4" -exec ./target/release/audio_extractor -i {} -o {}.mp3 \;

# With GNU parallel (if available)
find . -name "*.mp4" | parallel ./target/release/audio_extractor -i {} -o {.}.mp3 -q 192
```

### Quality Optimization
```bash
# Voice recordings (low quality, small file)
./target/release/audio_extractor -i interview.mp4 -o interview.mp3 -q 64

# Music (high quality)
./target/release/audio_extractor -i concert.mkv -o concert.mp3 -q 256

# Archival quality (lossless)
./target/release/audio_extractor -i recording.avi -o recording.flac -f flac
```

## Integration Examples

### With FFprobe
```bash
# Get video information first
ffprobe -v quiet -print_format json -show_format input.mp4

# Then extract audio
./target/release/audio_extractor -i input.mp4 -o output.mp3
```

### With File Organization
```bash
# Create organized directory structure
mkdir -p extracted_audio/{mp3,wav,flac}

# Extract to specific directories
./target/release/audio_extractor -i video.mp4 -o extracted_audio/mp3/audio.mp3
./target/release/audio_extractor -i video.mp4 -o extracted_audio/wav/audio.wav -f wav
./target/release/audio_extractor -i video.mp4 -o extracted_audio/flac/audio.flac -f flac
```

### Error Handling in Scripts
```bash
#!/bin/bash
if ./target/release/audio_extractor -i "$input" -o "$output" -q 192; then
    echo "✅ Successfully extracted: $input -> $output"
    # Optional: remove original video file
    # rm "$input"
else
    echo "❌ Failed to extract: $input"
    exit 1
fi
```

### `verify_audio.rs`
A standalone audio file verification tool that can validate any audio file independently.

**Usage:**
```bash
# Build the example
cargo build --example verify_audio

# Run the verification tool
./target/debug/examples/verify_audio audio_file.mp3

# Example output:
# Audio File Verification Tool
# ============================
# File: "audio_file.mp3"
# 
# ✓ Audio file verification successful!
#   - Format: mp3
#   - Duration: 180.45 seconds
#   - Channels: 2
#   - Sample rate: 44100 Hz
#   - File size: 1048576 bytes
```

**Features:**
- Validates audio file format and structure
- Extracts and displays metadata
- Provides detailed error messages
- Can be used in scripts and pipelines
