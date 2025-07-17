use audio_extractor::AudioExtractor;
use std::path::Path;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("🎵 Audio File Verifier");
        println!("Usage: {} <audio_file>", args[0]);
        println!();
        println!("Examples:");
        println!("  {} audio.mp3", args[0]);
        println!("  {} music.flac", args[0]);
        println!("  {} song.wav", args[0]);
        return Ok(());
    }
    
    let audio_file = Path::new(&args[1]);
    
    println!("🎵 Audio File Verifier");
    println!("======================");
    println!("📄 File: {:?}", audio_file);
    println!();
    
    if !audio_file.exists() {
        println!("❌ Error: File does not exist!");
        return Ok(());
    }
    
    println!("🔍 Analyzing audio file...");
    
    match AudioExtractor::verify_standalone(&audio_file.to_path_buf()) {
        Ok(info) => {
            println!("✅ Audio file verification successful!");
            println!();
            println!("📊 File Information:");
            println!("  🎵 Format: {}", info.format);
            
            if let Some(duration) = info.duration {
                let minutes = (duration / 60.0) as u32;
                let seconds = duration % 60.0;
                println!("  ⏱️  Duration: {}:{:05.2}", minutes, seconds);
            }
            
            if let Some(channels) = info.channels {
                let channel_desc = match channels {
                    1 => "Mono",
                    2 => "Stereo",
                    n => &format!("{} channels", n),
                };
                println!("  🔊 Channels: {} ({})", channels, channel_desc);
            }
            
            if let Some(sample_rate) = info.sample_rate {
                println!("  📻 Sample Rate: {} Hz", sample_rate);
            }
            
            let metadata = std::fs::metadata(audio_file)?;
            let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
            println!("  💾 File Size: {} bytes ({:.2} MB)", metadata.len(), size_mb);
            
            if let Some(duration) = info.duration {
                if duration > 0.0 {
                    let bitrate = (metadata.len() as f64 * 8.0) / (duration * 1000.0);
                    println!("  📈 Estimated Bitrate: {:.0} kbps", bitrate);
                }
            }
            
            println!();
            println!("🎉 Analysis completed successfully!");
        }
        Err(e) => {
            println!("❌ Audio file verification failed!");
            println!("Error: {}", e);
            println!();
            println!("💡 Possible reasons:");
            println!("  - File is not a valid audio file");
            println!("  - File is corrupted");
            println!("  - Unsupported audio format");
            println!("  - File is empty or incomplete");
        }
    }
    
    Ok(())
}