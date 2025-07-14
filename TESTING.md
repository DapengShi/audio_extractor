# Audio Extractor Tests

This document describes the comprehensive test suite for the audio extractor tool.

## Test Structure

### Unit Tests (`src/lib.rs`)
Located in the main library file, these tests cover:

#### Core Functionality Tests
- **AudioFormat Tests**: 
  - `test_audio_format_display()` - Tests string representation of audio formats
  - `test_audio_format_debug()` - Tests debug formatting
  - `test_audio_format_equality()` - Tests format comparison
  - `test_audio_format_clone()` - Tests format cloning

#### AudioExtractor Tests
- **Creation**: `test_audio_extractor_creation()` - Tests extractor initialization
- **Validation**: 
  - `test_validate_input_existing_file()` - Tests validation with valid input
  - `test_validate_input_nonexistent_file()` - Tests validation with missing input
  - `test_validate_input_unsupported_format()` - Tests validation with invalid format
- **Format Detection**:
  - `test_is_video_file_supported_formats()` - Tests supported video formats
  - `test_is_video_file_unsupported_formats()` - Tests unsupported formats
- **Directory Creation**: `test_create_output_directory()` - Tests output directory creation
- **Extraction**:
  - `test_extract_audio_success()` - Tests successful extraction
  - `test_extract_with_different_formats()` - Tests all supported output formats
  - `test_extract_with_different_qualities()` - Tests different quality settings
  - `test_extract_nonexistent_input()` - Tests error handling for missing input
  - `test_extract_invalid_input_format()` - Tests error handling for invalid format

#### Static Method Tests
- `test_get_supported_video_formats()` - Tests video format list
- `test_get_supported_audio_formats()` - Tests audio format list
- `test_args_structure()` - Tests argument structure

### Integration Tests (`src/lib.rs`)
These tests verify complete workflows:
- `test_full_workflow()` - Tests complete extraction process
- `test_multiple_extractions()` - Tests multiple format extractions
- `test_error_handling_chain()` - Tests error propagation

### CLI Tests (`tests/cli_tests.rs`)
Command-line interface tests using `assert_cmd`:
- `test_cli_help()` - Tests help output
- `test_cli_version()` - Tests version output
- `test_cli_missing_arguments()` - Tests missing argument handling
- `test_cli_successful_extraction()` - Tests successful CLI extraction
- `test_cli_with_format_option()` - Tests format option
- `test_cli_with_quality_option()` - Tests quality option
- `test_cli_nonexistent_input()` - Tests CLI error handling
- `test_cli_invalid_format()` - Tests CLI format validation
- `test_cli_short_flags()` - Tests short command flags
- `test_cli_all_supported_formats()` - Tests all formats via CLI
- `test_cli_various_quality_settings()` - Tests quality settings via CLI

### Benchmark Tests (`benches/audio_extraction_bench.rs`)
Performance tests using Criterion:
- `benchmark_audio_extraction()` - Tests extraction performance with different file sizes
- `benchmark_validation()` - Tests input validation performance
- `benchmark_format_detection()` - Tests format detection performance
- `benchmark_different_formats()` - Tests performance across formats
- `benchmark_different_qualities()` - Tests performance across quality settings

## Running Tests

### Unit and Integration Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_audio_format_display

# Run tests in specific module
cargo test tests::

# Run integration tests only
cargo test --test cli_tests
```

### Benchmark Tests
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench benchmark_audio_extraction

# Generate HTML reports
cargo bench --features html_reports
```

### Test Coverage
```bash
# Install cargo-tarpaulin for coverage
cargo install cargo-tarpaulin

# Run coverage analysis
cargo tarpaulin --out Html
```

## Test Data

Tests use temporary files and directories created with the `tempfile` crate to ensure:
- Clean test environment
- No side effects between tests
- Automatic cleanup
- Cross-platform compatibility

## Supported Test Scenarios

### Input Formats Tested
- MP4, AVI, MKV, MOV, WMV, FLV, WebM
- Case insensitive extensions
- Invalid formats (TXT, JPG, etc.)
- Missing files

### Output Formats Tested
- MP3, WAV, FLAC, AAC
- Different quality settings (64-320 kbps)
- Various output paths

### Error Conditions Tested
- Non-existent input files
- Unsupported input formats
- Invalid command-line arguments
- Directory creation failures

## Continuous Integration

These tests are designed to run in CI environments and include:
- Cross-platform compatibility
- No external dependencies
- Deterministic results
- Fast execution
- Comprehensive coverage

## Test Best Practices

The test suite follows Rust testing best practices:
- Descriptive test names
- Isolated test cases
- Proper setup and teardown
- Clear assertions
- Edge case coverage
- Performance testing
- Integration testing
