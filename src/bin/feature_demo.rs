use audio_extractor::{AudioExtractor, AudioFormat, Args};
use std::path::Path;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🎵 Audio Extractor - Complete Feature Demo");
    println!("===========================================\n");
    
    // Ensure test video file exists
    let test_video = "sample-15s.mp4";
    if !Path::new(test_video).exists() {
        println!("❌ Test video file not found: {}", test_video);
        println!("💡 Please ensure {} exists in the current directory", test_video);
        return Ok(());
    }
    
    println!("📹 Test video: {}", test_video);
    println!("📁 Creating output directory...\n");
    
    // Create output directory
    let output_dir = Path::new("demo_output");
    std::fs::create_dir_all(output_dir)?;
    
    // Demo 1: Basic Audio Extraction
    println!("🎯 Demo 1: Basic Audio Extraction");
    println!("==================================");
    
    let mp3_output = output_dir.join("basic_extract.mp3");
    let args = Args {
        input: Path::new(test_video).to_path_buf(),
        output: mp3_output.clone(),
        format: Some(AudioFormat::Mp3),
        quality: Some(128),
        verify: true,
    };
    
    let extractor = AudioExtractor::new(args);
    match extractor.extract_with_progress(|msg| println!("  📄 {}", msg)) {
        Ok(()) => println!("  ✅ Basic extraction completed!\n"),
        Err(e) => println!("  ❌ Error: {}\n", e),
    }
    
    // Demo 2: Different Audio Formats
    println!("🎯 Demo 2: Different Audio Formats");
    println!("===================================");
    
    let formats = vec![
        (AudioFormat::Mp3, 128, "Standard MP3"),
        (AudioFormat::Wav, 0, "Lossless WAV"),
        (AudioFormat::Flac, 0, "Lossless FLAC"),
        (AudioFormat::Aac, 192, "High Quality AAC"),
    ];
    
    for (format, quality, description) in formats {
        println!("  🎵 Extracting {} ({})", description, format);
        
        let output_file = output_dir.join(format!("format_demo.{}", format));
        let args = Args {
            input: Path::new(test_video).to_path_buf(),
            output: output_file.clone(),
            format: Some(format),
            quality: Some(quality),
            verify: false, // Skip verification to speed up demo
        };
        
        let extractor = AudioExtractor::new(args);
        match extractor.extract() {
            Ok(()) => {
                let size = std::fs::metadata(&output_file)?.len();
                println!("    ✅ Success! Size: {} bytes", size);
            }
            Err(e) => println!("    ❌ Error: {}", e),
        }
    }
    
    println!();
    
    // Demo 3: Different Quality Settings
    println!("🎯 Demo 3: Different Quality Settings");
    println!("=====================================");
    
    let qualities = vec![64, 128, 192, 256, 320];
    for quality in qualities {
        println!("  🎵 Extracting MP3 at {} kbps", quality);
        
        let output_file = output_dir.join(format!("quality_{}k.mp3", quality));
        let args = Args {
            input: Path::new(test_video).to_path_buf(),
            output: output_file.clone(),
            format: Some(AudioFormat::Mp3),
            quality: Some(quality),
            verify: false,
        };
        
        let extractor = AudioExtractor::new(args);
        match extractor.extract() {
            Ok(()) => {
                let size = std::fs::metadata(&output_file)?.len();
                println!("    ✅ Success! Size: {} bytes", size);
            }
            Err(e) => println!("    ❌ Error: {}", e),
        }
    }
    
    println!();
    
    // Demo 4: Verification Feature
    println!("🎯 Demo 4: Audio Verification");
    println!("=============================");
    
    let verify_file = output_dir.join("verify_demo.mp3");
    let args = Args {
        input: Path::new(test_video).to_path_buf(),
        output: verify_file.clone(),
        format: Some(AudioFormat::Mp3),
        quality: Some(192),
        verify: true,
    };
    
    let extractor = AudioExtractor::new(args);
    match extractor.extract() {
        Ok(()) => println!("  ✅ Verification demo completed!\n"),
        Err(e) => println!("  ❌ Error: {}\n", e),
    }
    
    // Demo 5: Standalone File Verification
    println!("🎯 Demo 5: Standalone File Verification");
    println!("=======================================");
    
    if verify_file.exists() {
        match AudioExtractor::verify_standalone(&verify_file) {
            Ok(info) => {
                println!("  ✅ File verification successful!");
                println!("    📊 Format: {}", info.format);
                if let Some(duration) = info.duration {
                    println!("    ⏱️ Duration: {:.2} seconds", duration);
                }
                if let Some(channels) = info.channels {
                    println!("    🔊 Channels: {}", channels);
                }
                if let Some(sample_rate) = info.sample_rate {
                    println!("    📻 Sample Rate: {} Hz", sample_rate);
                }
            }
            Err(e) => println!("  ❌ Verification failed: {}", e),
        }
    }
    
    println!();
    
    // Demo 6: Supported Formats Info
    println!("🎯 Demo 6: Supported Formats");
    println!("============================");
    
    let video_formats = AudioExtractor::get_supported_video_formats();
    println!("  📹 Supported video formats: {:?}", video_formats);
    
    let audio_formats = AudioExtractor::get_supported_audio_formats();
    println!("  🎵 Supported audio formats: {:?}", audio_formats);
    
    println!();
    
    // Display all generated files
    println!("🎯 Generated Files Summary");
    println!("=========================");
    
    if output_dir.exists() {
        let entries = std::fs::read_dir(output_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let size = std::fs::metadata(&path)?.len();
                println!("  📄 {} ({} bytes)", path.file_name().unwrap().to_str().unwrap(), size);
            }
        }
    }
    
    println!("\n🎉 Demo completed successfully!");
    println!("📁 All output files are in: {:?}", output_dir);
    
    Ok(())
}