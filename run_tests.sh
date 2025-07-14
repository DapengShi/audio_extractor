#!/bin/bash

# Audio Extractor Test Runner Script
# This script runs all tests for the audio extractor tool

set -e  # Exit on any error

echo "🎵 Audio Extractor Test Suite 🎵"
echo "================================="

# Function to print section headers
print_section() {
    echo ""
    echo "📋 $1"
    echo "$(printf '%.0s-' {1..50})"
}

# Function to run command with timing
run_timed() {
    echo "⏱️  Running: $1"
    time eval "$1"
    echo "✅ Completed: $1"
}

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo is not installed or not in PATH"
    exit 1
fi

print_section "Unit and Integration Tests"
run_timed "cargo test --lib"

print_section "CLI Integration Tests"
run_timed "cargo test --test cli_tests"

print_section "All Tests with Output"
run_timed "cargo test -- --nocapture"

print_section "Documentation Tests"
run_timed "cargo test --doc"

print_section "Benchmark Tests (if available)"
if cargo bench --help &> /dev/null; then
    run_timed "cargo bench --bench audio_extraction_bench"
else
    echo "⚠️  Benchmark tests skipped (criterion not available)"
fi

print_section "Code Formatting Check"
if command -v rustfmt &> /dev/null; then
    run_timed "cargo fmt -- --check"
else
    echo "⚠️  Formatting check skipped (rustfmt not available)"
fi

print_section "Linting with Clippy"
if command -v cargo-clippy &> /dev/null || cargo clippy --help &> /dev/null; then
    run_timed "cargo clippy -- -D warnings"
else
    echo "⚠️  Clippy check skipped (clippy not available)"
fi

print_section "Build Check"
run_timed "cargo build"
run_timed "cargo build --release"

print_section "Test Coverage (Optional)"
if command -v cargo-tarpaulin &> /dev/null; then
    echo "📊 Generating test coverage report..."
    run_timed "cargo tarpaulin --out Html --output-dir coverage"
    echo "📈 Coverage report generated in coverage/ directory"
else
    echo "⚠️  Coverage analysis skipped (cargo-tarpaulin not available)"
    echo "💡 Install with: cargo install cargo-tarpaulin"
fi

print_section "Test Summary"
echo "🎉 All tests completed successfully!"
echo ""
echo "📊 Test Statistics:"
echo "   - Unit tests: ✅"
echo "   - Integration tests: ✅"
echo "   - CLI tests: ✅"
echo "   - Documentation tests: ✅"
echo "   - Build verification: ✅"
echo ""
echo "🚀 The audio extractor is ready for use!"
