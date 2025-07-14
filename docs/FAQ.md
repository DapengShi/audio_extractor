# Frequently Asked Questions

## 🔧 Installation and Compilation Issues

### Q: "FFmpeg not found" error during compilation
**A**: This usually means FFmpeg is not installed on your system or is not in the system PATH.

**Solution**:
```bash
# macOS
brew install ffmpeg

# Ubuntu/Debian
sudo apt update
sudo apt install ffmpeg

# CentOS/RHEL
sudo yum install ffmpeg
# or
sudo dnf install ffmpeg

# Windows
# Download from https://ffmpeg.org/download.html
# Extract and add bin directory to system PATH
```

### Q: Rust version error during compilation
**A**: The project requires Rust 1.70 or higher.

**Solution**:
```bash
# Update Rust
rustup update

# Check version
rustc --version
```

### Q: Dependency errors during compilation
**A**: This might be due to network issues or dependency version conflicts.

**Solution**:
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Update dependencies
cargo update
```

## 🎵 Audio Extraction Issues

### Q: Extracted audio file has no sound
**A**: The input video might not have an audio track or the audio codec is not supported.

**Solution**:
```bash
# Check if video has audio track
ffmpeg -i input.mp4 2>&1 | grep -i audio

# Use --verify flag to check output
audio_extractor -i input.mp4 -o output.mp3 --verify
```

### Q: Poor audio quality in extracted file
**A**: The bitrate setting might be too low.

**Solution**:
```bash
# Use higher bitrate
audio_extractor -i input.mp4 -o output.mp3 -q 320

# Use lossless format
audio_extractor -i input.mp4 -o output.flac -f flac
```

### Q: Extraction process is very slow
**A**: Processing large files can take considerable time.

**Solution**:
```bash
# Use lower quality settings for faster processing
audio_extractor -i input.mp4 -o output.mp3 -q 128

# Use more efficient formats
audio_extractor -i input.mp4 -o output.aac -f aac
```

## 📁 File and Path Issues

### Q: "Input file does not exist" error
**A**: The input file path is incorrect or the file doesn't exist.

**Solution**:
```bash
# Check if file exists
ls -la /path/to/video.mp4

# Use absolute path
audio_extractor -i /full/path/to/video.mp4 -o output.mp3

# Check file permissions
file /path/to/video.mp4
```

### Q: "Failed to create output directory" error
**A**: Unable to create output directory, possibly due to permission issues.

**Solution**:
```bash
# Create directory manually
mkdir -p /path/to/output/directory

# Check permissions
ls -ld /path/to/output/

# Use current directory
audio_extractor -i input.mp4 -o ./output.mp3
```

### Q: File names with special characters cause errors
**A**: Special characters might cause path parsing issues.

**Solution**:
```bash
# Use quotes around file names
audio_extractor -i "video with spaces.mp4" -o "output file.mp3"

# Or rename the file
mv "video with spaces.mp4" video.mp4
```

## 🔍 Format and Compatibility Issues

### Q: "Input file is not a supported video format" error
**A**: The input file is not a supported video format.

**Supported formats**: MP4, AVI, MKV, MOV, WMV, FLV, WebM

**Solution**:
```bash
# Check file type
file input.unknown

# Use FFmpeg to convert format
ffmpeg -i input.unknown -c copy output.mp4

# Or rename file (if extension is wrong)
mv input.unknown input.mp4
```

### Q: Output audio format not supported by player
**A**: Different players support different audio formats.

**Solution**:
```bash
# Use widely supported MP3 format
audio_extractor -i input.mp4 -o output.mp3 -f mp3

# Or use AAC format (supported by modern players)
audio_extractor -i input.mp4 -o output.aac -f aac
```

## 📊 Performance and Quality Issues

### Q: How to choose appropriate audio quality?
**A**: Select bitrate based on use case:

- **64 kbps**: Voice recordings, small files
- **128 kbps**: Standard quality, suitable for most purposes
- **192 kbps**: High quality, recommended for music
- **256 kbps**: Very high quality, for audiophiles
- **320 kbps**: Maximum quality for lossy formats

### Q: How to balance file size and quality?
**A**: Choose format and quality based on requirements:

```bash
# Small files, general quality
audio_extractor -i input.mp4 -o output.mp3 -q 128

# Balanced file size and quality
audio_extractor -i input.mp4 -o output.aac -f aac -q 192

# Lossless but larger files
audio_extractor -i input.mp4 -o output.flac -f flac
```

### Q: Excessive memory usage
**A**: The program uses streaming processing and typically doesn't consume large amounts of memory.

**Solution**:
```bash
# Check system resources
top -p $(pgrep audio_extractor)

# If problem persists, try restarting system
# or process one file at a time
```

## 🔄 Batch Processing Issues

### Q: How to batch process multiple files?
**A**: Use shell scripts or find command:

```bash
#!/bin/bash
# Batch processing script
for file in *.mp4; do
    if [ -f "$file" ]; then
        audio_extractor -i "$file" -o "${file%.mp4}.mp3" -q 192
    fi
done
```

```bash
# Using find command
find . -name "*.mp4" -exec audio_extractor -i {} -o {}.mp3 -q 192 \;

# Using GNU parallel (if available)
find . -name "*.mp4" | parallel audio_extractor -i {} -o {}.mp3 -q 192
```

### Q: Some files fail during batch processing
**A**: Add error handling:

```bash
#!/bin/bash
for file in *.mp4; do
    if [ -f "$file" ]; then
        echo "Processing: $file"
        if audio_extractor -i "$file" -o "${file%.mp4}.mp3" -q 192; then
            echo "✅ Success: $file"
        else
            echo "❌ Failed: $file"
        fi
    fi
done
```

## 🧪 Testing and Verification Issues

### Q: Audio verification failed
**A**: The output file might be corrupted or in incorrect format.

**Solution**:
```bash
# Try without verification first
audio_extractor -i input.mp4 -o output.mp3

# Check output file
ls -la output.mp3
file output.mp3

# Verify with FFmpeg
ffmpeg -v error -i output.mp3 -f null -
```

### Q: How to manually verify audio files?
**A**: Use multiple methods for verification:

```bash
# Use file command
file output.mp3

# Use FFmpeg to get information
ffmpeg -i output.mp3 2>&1 | grep -i duration

# Use FFprobe (if available)
ffprobe -v quiet -print_format json -show_format output.mp3
```

## 🐛 Debugging and Logging

### Q: How to get more detailed error information?
**A**: Use Rust's logging functionality:

```bash
# Set log level
RUST_LOG=debug audio_extractor -i input.mp4 -o output.mp3

# Or
RUST_LOG=info audio_extractor -i input.mp4 -o output.mp3
```

### Q: How to report bugs?
**A**: Collect the following information:

1. **System information**:
   ```bash
   uname -a
   rustc --version
   ffmpeg -version
   ```

2. **Error messages**: Complete error output
3. **Reproduction steps**: Detailed operation steps
4. **Sample files**: If possible, provide problematic files

## 🔧 Advanced Usage

### Q: How to use the library in code?
**A**: Refer to [API Documentation](API.md):

```rust
use audio_extractor::{Args, AudioExtractor, AudioFormat};
use std::path::PathBuf;

let args = Args {
    input: PathBuf::from("input.mp4"),
    output: PathBuf::from("output.mp3"),
    format: AudioFormat::Mp3,
    quality: 192,
    verify: true,
};

let extractor = AudioExtractor::new(args);
extractor.extract()?;
```

### Q: How to extend supported formats?
**A**: Modify source code:

1. Add new format to `AudioFormat` enum
2. Add FFmpeg parameters in `extract_audio_with_ffmpeg`
3. Update tests and documentation

## 📱 Platform-Specific Issues

### macOS
- Install FFmpeg using Homebrew: `brew install ffmpeg`
- If permission issues occur, check macOS security settings

### Linux
- Install FFmpeg using package manager
- Ensure user has file read/write permissions

### Windows
- Download FFmpeg from official website and add to PATH
- Use PowerShell or Command Prompt
- Backslashes in paths might need escaping

## 🚀 Performance Optimization

### Q: How to improve processing speed?
**A**: Several suggestions:

1. **Use faster storage** (SSD)
2. **Choose appropriate quality settings**
3. **Use efficient formats** (AAC vs MP3)
4. **Ensure FFmpeg is latest version**

### Q: Large file processing timeout
**A**: This is normal; large files require more time:

```bash
# Test with small files first
audio_extractor -i small_test.mp4 -o test.mp3

# For large files, be patient or use lower quality
audio_extractor -i large_file.mp4 -o output.mp3 -q 128
```

---
