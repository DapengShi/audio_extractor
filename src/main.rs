use audio_extractor::{Args, AudioExtractor};
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    let extractor = AudioExtractor::new(args);
    extractor.extract()?;
    println!("Audio extraction completed successfully!");
    Ok(())
}
