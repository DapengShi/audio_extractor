use criterion::{black_box, criterion_group, criterion_main, Criterion};
use audio_extractor::{Args, AudioExtractor, AudioFormat};
use tempfile::{tempdir, NamedTempFile};
use std::fs;
use std::path::PathBuf;

fn create_test_video_file(size_kb: usize) -> NamedTempFile {
    let temp_file = NamedTempFile::with_suffix(".mp4").unwrap();
    let data = vec![0u8; size_kb * 1024]; // Create fake video data
    fs::write(temp_file.path(), data).unwrap();
    temp_file
}

fn benchmark_audio_extraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("audio_extraction");
    
    // Test with different file sizes
    let sizes = vec![1, 10, 100]; // 1KB, 10KB, 100KB
    
    for size in sizes {
        group.bench_function(&format!("extract_{}kb", size), |b| {
            b.iter(|| {
                let temp_input = create_test_video_file(size);
                let temp_dir = tempdir().unwrap();
                let output_path = temp_dir.path().join("output.mp3");
                
                let args = Args {
                    input: temp_input.path().to_path_buf(),
                    output: output_path,
                    format: AudioFormat::Mp3,
                    quality: 128,
                };
                
                let extractor = AudioExtractor::new(args);
                black_box(extractor.extract().unwrap());
            });
        });
    }
    
    group.finish();
}

fn benchmark_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation");
    
    let temp_input = create_test_video_file(10);
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.mp3");
    
    let args = Args {
        input: temp_input.path().to_path_buf(),
        output: output_path,
        format: AudioFormat::Mp3,
        quality: 128,
    };
    
    let extractor = AudioExtractor::new(args);
    
    group.bench_function("validate_input", |b| {
        b.iter(|| {
            black_box(extractor.validate_input().unwrap());
        });
    });
    
    group.finish();
}

fn benchmark_format_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("format_detection");
    
    let temp_dir = tempdir().unwrap();
    let args = Args {
        input: temp_dir.path().join("test.mp4"),
        output: temp_dir.path().join("output.mp3"),
        format: AudioFormat::Mp3,
        quality: 128,
    };
    
    let extractor = AudioExtractor::new(args);
    
    let test_files = vec![
        PathBuf::from("test.mp4"),
        PathBuf::from("test.avi"),
        PathBuf::from("test.mkv"),
        PathBuf::from("test.mov"),
        PathBuf::from("test.txt"),
        PathBuf::from("test.jpg"),
    ];
    
    group.bench_function("is_video_file", |b| {
        b.iter(|| {
            for file in &test_files {
                black_box(extractor.is_video_file(file));
            }
        });
    });
    
    group.finish();
}

fn benchmark_different_formats(c: &mut Criterion) {
    let mut group = c.benchmark_group("different_formats");
    
    let formats = vec![
        AudioFormat::Mp3,
        AudioFormat::Wav,
        AudioFormat::Flac,
        AudioFormat::Aac,
    ];
    
    for format in formats {
        group.bench_function(&format!("extract_{:?}", format), |b| {
            b.iter(|| {
                let temp_input = create_test_video_file(10);
                let temp_dir = tempdir().unwrap();
                let output_path = temp_dir.path().join(format!("output.{}", format));
                
                let args = Args {
                    input: temp_input.path().to_path_buf(),
                    output: output_path,
                    format: format.clone(),
                    quality: 128,
                };
                
                let extractor = AudioExtractor::new(args);
                black_box(extractor.extract().unwrap());
            });
        });
    }
    
    group.finish();
}

fn benchmark_different_qualities(c: &mut Criterion) {
    let mut group = c.benchmark_group("different_qualities");
    
    let qualities = vec![64, 128, 192, 256, 320];
    
    for quality in qualities {
        group.bench_function(&format!("extract_{}kbps", quality), |b| {
            b.iter(|| {
                let temp_input = create_test_video_file(10);
                let temp_dir = tempdir().unwrap();
                let output_path = temp_dir.path().join("output.mp3");
                
                let args = Args {
                    input: temp_input.path().to_path_buf(),
                    output: output_path,
                    format: AudioFormat::Mp3,
                    quality,
                };
                
                let extractor = AudioExtractor::new(args);
                black_box(extractor.extract().unwrap());
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_audio_extraction,
    benchmark_validation,
    benchmark_format_detection,
    benchmark_different_formats,
    benchmark_different_qualities
);
criterion_main!(benches);
