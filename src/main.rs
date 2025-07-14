use audio_extractor::{Args, AudioExtractor};
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Show what we're about to do
    println!("Audio Extractor v{}", env!("CARGO_PKG_VERSION"));
    println!("Input: {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!("Format: {}", args.format);
    println!("Quality: {} kbps", args.quality);
    if args.verify {
        println!("Verification: enabled");
    }
    println!();
    
    let extractor = AudioExtractor::new(args);
    
    match extractor.extract_with_progress(|msg| println!("📄 {}", msg)) {
        Ok(()) => {
            println!("✅ Audio extraction completed successfully!");
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
