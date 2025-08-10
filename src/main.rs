/*!
 * YKS Converter Example - MML/MIDI to MP3 converter
 * 
 * This tool converts MML (Music Macro Language) and MIDI files to MP3 format 
 * using yks_converter, FluidSynth, and LAME encoder.
 * It supports SoundFont (.sf2) files for high-quality synthesis.
 */

use yks_converter_example::pipeline::ConversionPipeline;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check command line arguments
    if args.len() != 4 && args.len() != 5 {
        eprintln!("YKS Converter Example - MML/MIDI to MP3 Converter");
        eprintln!("Usage: {} <input_file> <sf2_file> <output_mp3> [instrument_number]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  input_file        - Input MML file (.mml) or MIDI file (.mid, .midi)");
        eprintln!("  sf2_file          - SoundFont file (.sf2)");
        eprintln!("  output_mp3        - Output MP3 file");
        eprintln!("  instrument_number - Optional: MIDI instrument number (0-127, default: 0)");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} song.mml soundfont.sf2 output.mp3", args[0]);
        eprintln!("  {} song.mml soundfont.sf2 output.mp3 1    # Use instrument 1", args[0]);
        eprintln!("  {} song.mml soundfont.sf2 output.mp3 25   # Use instrument 25", args[0]);
        eprintln!("  {} song.mid soundfont.sf2 output.mp3 40   # Use instrument 40", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let sf2_path = &args[2];
    let mp3_path = &args[3];
    let instrument_number = if args.len() == 5 {
        match args[4].parse::<u8>() {
            Ok(num) if num <= 127 => num,
            Ok(_) => {
                eprintln!("❌ Instrument number must be between 0-127");
                std::process::exit(1);
            },
            Err(_) => {
                eprintln!("❌ Invalid instrument number: {}", args[4]);
                std::process::exit(1);
            }
        }
    } else {
        0 // Default to instrument 0 (Grand Piano)
    };

    // Detect input file type
    let input_extension = Path::new(input_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let is_mml = input_extension == "mml";
    let is_midi = matches!(input_extension.as_str(), "mid" | "midi");

    if !is_mml && !is_midi {
        eprintln!("❌ Unsupported file format: {}", input_extension);
        eprintln!("   Supported formats: .mml, .mid, .midi");
        std::process::exit(1);
    }

    println!("🎵 YKS Converter Example - Starting {} to MP3 conversion...", 
             if is_mml { "MML" } else { "MIDI" });
    println!("📂 Input file: {}", input_path);
    println!("🎹 SoundFont: {}", sf2_path);
    if is_mml {
        println!("🎼 Instrument: {}", instrument_number);
    }
    println!("🎧 Output: {}", mp3_path);
    println!();
    
    // Initialize conversion pipeline
    let mut pipeline = match ConversionPipeline::new() {
        Ok(pipeline) => {
            println!("✅ Conversion pipeline initialized");
            pipeline
        },
        Err(e) => {
            eprintln!("❌ Failed to create conversion pipeline: {}", e);
            std::process::exit(1);
        }
    };

    // Load SoundFont file
    if let Err(e) = pipeline.load_soundfont(sf2_path) {
        eprintln!("❌ Failed to load soundfont: {}", e);
        std::process::exit(1);
    }
    println!("✅ SoundFont loaded");

    // Set instrument (only needed for MML files)
    if is_mml {
        if let Err(e) = pipeline.set_instrument(instrument_number) {
            eprintln!("❌ Failed to set instrument: {}", e);
            std::process::exit(1);
        }
        if instrument_number != 0 {
            println!("✅ Instrument {} set", instrument_number);
        }
    }

    // Show file information for MML files
    if is_mml {
        match pipeline.get_conversion_info(input_path) {
            Ok(info) => println!("{}", info),
            Err(e) => eprintln!("⚠️  Warning: Could not get file info: {}", e),
        }
        println!();
    }

    // Convert file to MP3
    let result = if is_mml {
        pipeline.convert_mml_to_mp3(input_path, mp3_path)
    } else {
        // For MIDI files, use the existing pipeline but skip MML conversion step
        use yks_converter_example::midi_converter::MidiConverter;
        use yks_converter_example::mp3_encoder::Mp3Encoder;
        
        let temp_wav_path = "temp_conversion.wav";
        
        println!("🎹 Synthesizing MIDI to WAV...");
        match MidiConverter::new() {
            Ok(mut midi_converter) => {
                match midi_converter.load_soundfont(sf2_path) {
                    Ok(_) => {
                        // MIDI files already contain instrument information
                        // The instrument_number parameter is ignored for MIDI files
                        match midi_converter.convert_midi_to_wav(input_path, temp_wav_path) {
                            Ok(_) => {
                                println!("✅ WAV file generated");

                                println!("🎵 Encoding WAV to MP3...");
                                match Mp3Encoder::convert_wav_to_mp3(temp_wav_path, mp3_path) {
                                    Ok(_) => {
                                        println!("✅ MP3 encoding completed");

                                        // Clean up temporary file
                                        if std::fs::remove_file(temp_wav_path).is_ok() {
                                            println!("🧹 Cleaned up temporary file: {}", temp_wav_path);
                                        }
                                        
                                        Ok(())
                                    },
                                    Err(e) => Err(format!("WAV to MP3 error: {}", e))
                                }
                            },
                            Err(e) => Err(format!("MIDI to WAV error: {}", e))
                        }
                    },
                    Err(e) => Err(format!("SoundFont error: {}", e))
                }
            },
            Err(e) => Err(format!("MIDI converter error: {}", e))
        }
    };

    match result {
        Ok(_) => {
            println!("🎉 Conversion completed successfully!");
            println!("📁 Output saved to: {}", mp3_path);
        },
        Err(e) => {
            eprintln!("❌ Conversion failed: {}", e);
            std::process::exit(1);
        }
    }
}
