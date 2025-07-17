use audio_extractor::{Args, AudioExtractor};
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    
    let extractor = AudioExtractor::new(args);

    // Show what we're about to do
    println!("Audio Extractor v{}", env!("CARGO_PKG_VERSION"));
    println!("Input: {:?}", extractor.args.input);
    println!("Output: {:?}", extractor.args.output);
    println!("Format: {}", extractor.args.format.as_ref().unwrap());
    println!("Quality: {} kbps", extractor.args.quality.unwrap());
    if extractor.args.verify {
        println!("Verification: enabled");
    }
    println!();
    
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
