#!/bin/bash
# Example batch processing script for audio extraction
# This script converts all MP4 files in the current directory to MP3

echo "🎵 Batch Audio Extraction Script"
echo "================================"

# Check if audio_extractor is available
if ! command -v ./target/release/audio_extractor &> /dev/null; then
    echo "❌ Error: audio_extractor not found. Please build the project first with 'cargo build --release'"
    exit 1
fi

# Count total files
total_files=$(ls *.mp4 2>/dev/null | wc -l)
if [ "$total_files" -eq 0 ]; then
    echo "ℹ️  No MP4 files found in the current directory."
    exit 0
fi

echo "📁 Found $total_files MP4 file(s) to process"
echo ""

# Process each MP4 file
count=0
for file in *.mp4; do
    if [ -f "$file" ]; then
        count=$((count + 1))
        output="${file%.mp4}.mp3"
        
        echo "🔄 Processing ($count/$total_files): $file"
        
        if ./target/release/audio_extractor -i "$file" -o "$output" -q 192; then
            echo "✅ Success: $file -> $output"
        else
            echo "❌ Failed: $file"
        fi
        echo ""
    fi
done

echo "🎉 Batch processing completed!"
echo "📊 Processed $count out of $total_files files"
