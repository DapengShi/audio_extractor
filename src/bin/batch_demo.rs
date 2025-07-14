use audio_extractor::{AudioExtractor, AudioFormat};
use std::path::Path;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🎵 Audio Extractor - Batch Processing Demo");
    println!("==========================================");
    
    let input_files = vec![
        "sample-15s.mp4", 
    ];
    
    let existing_files: Vec<_> = input_files.iter()
        .filter(|&file| Path::new(file).exists())
        .collect();
    
    if existing_files.is_empty() {
        println!("❌ No valid input files found!");
        return Ok(());
    }
    
    println!("📁 Found {} input file(s)", existing_files.len());
    
    let output_dir = Path::new("batch_output");
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
        println!("📁 Created output directory: {:?}", output_dir);
    }
    
    let formats = vec![
        (AudioFormat::Mp3, 128),
        (AudioFormat::Wav, 256),
        (AudioFormat::Flac, 320),
        (AudioFormat::Aac, 192),
    ];
    
    for (format, quality) in formats {
        println!("\n🎯 Processing format: {} ({}k)", format, quality);
        println!("{}{}{}",
            "=".repeat(20),
            format!(" {} ", format),
            "=".repeat(20)
        );
        
        let format_dir = output_dir.join(format.to_string());
        if !format_dir.exists() {
            std::fs::create_dir_all(&format_dir)?;
        }
        
        let results = AudioExtractor::extract_batch(
            existing_files.iter().map(|s| Path::new(s)).collect(),
            &format_dir,
            format,
            quality,
            false
        )?;
        
        // show results
        for (i, result) in results.iter().enumerate() {
            let input_file = existing_files[i];
            match result {
                Ok(output_path) => {
                    println!("✅ {} → {:?}", input_file, output_path);
                }
                Err(e) => {
                    println!("❌ {} → Error: {}", input_file, e);
                }
            }
        }
    }
    
    println!("\n🎉 Batch processing completed!");
    println!("📁 Output directory: {:?}", output_dir);
    
    Ok(())
}
