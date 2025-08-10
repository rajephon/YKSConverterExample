/*!
 * Complete MML to MP3 Conversion Pipeline
 * 
 * Integrates MML→MIDI→WAV→MP3 conversion pipeline using:
 * - yks_converter for MML→MIDI conversion
 * - FluidSynth for MIDI→WAV synthesis  
 * - LAME for WAV→MP3 encoding
 */

use crate::mml_converter::MmlConverter;
use crate::midi_converter::MidiConverter;
use crate::mp3_encoder::Mp3Encoder;
use std::fs;
use std::path::Path;

/// Complete MML to MP3 conversion pipeline
/// 
/// This pipeline handles the entire conversion process from Mabinogi MML files
/// to high-quality MP3 output through multiple stages.
/// 
/// # Pipeline Stages
/// 
/// 1. **MML → MIDI**: Parse MML and generate MIDI data
/// 2. **MIDI → WAV**: Synthesize audio using FluidSynth with SoundFont
/// 3. **WAV → MP3**: Encode to MP3 using LAME at 192kbps
/// 
/// # Example
/// 
/// ```no_run
/// use yks_converter_example::pipeline::ConversionPipeline;
/// 
/// let pipeline = ConversionPipeline::new()?;
/// pipeline.load_soundfont("piano.sf2")?;
/// pipeline.convert_mml_to_mp3("song.mml", "output.mp3")?;
/// # Ok::<(), String>(())
/// ```
pub struct ConversionPipeline {
    mml_converter: MmlConverter,
    midi_converter: MidiConverter,
}

impl ConversionPipeline {
    /// Creates a new conversion pipeline
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(ConversionPipeline)` on success, or `Err(String)` with error message.
    pub fn new() -> Result<Self, String> {
        let mml_converter = MmlConverter::new();
        let midi_converter = MidiConverter::new()?;
        
        Ok(ConversionPipeline {
            mml_converter,
            midi_converter,
        })
    }

    /// Loads a SoundFont file for MIDI synthesis
    /// 
    /// # Arguments
    /// 
    /// * `soundfont_path` - Path to the SoundFont (.sf2) file
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    pub fn load_soundfont(&mut self, soundfont_path: &str) -> Result<(), String> {
        self.midi_converter.load_soundfont(soundfont_path)
    }

    /// Sets the instrument for MML conversion
    /// 
    /// # Arguments
    /// 
    /// * `program` - MIDI program number (0-127)
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    pub fn set_instrument(&mut self, program: u8) -> Result<(), String> {
        // Set instrument for MML conversion only
        // The MIDI file generated will contain the instrument information
        self.mml_converter.set_instrument(program);
        Ok(())
    }

    /// Converts MML file directly to MP3
    /// 
    /// This is the main pipeline function that performs the complete conversion:
    /// MML → MIDI → WAV → MP3
    /// 
    /// # Arguments
    /// 
    /// * `mml_file_path` - Path to input MML file
    /// * `mp3_output_path` - Path for output MP3 file
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    pub fn convert_mml_to_mp3(&mut self, mml_file_path: &str, mp3_output_path: &str) -> Result<(), String> {
        // Generate temporary file names
        let temp_midi_path = "temp_conversion.mid";
        let temp_wav_path = "temp_conversion.wav";

        // Step 1: MML → MIDI
        println!("🎼 Converting MML to MIDI...");
        self.mml_converter.convert_mml_file_to_midi(mml_file_path, temp_midi_path)?;
        println!("✅ MIDI file generated");

        // Step 2: MIDI → WAV
        println!("🎹 Synthesizing MIDI to WAV...");
        self.midi_converter.convert_midi_to_wav(temp_midi_path, temp_wav_path)?;
        println!("✅ WAV file generated");

        // Step 3: WAV → MP3
        println!("🎵 Encoding WAV to MP3...");
        Mp3Encoder::convert_wav_to_mp3(temp_wav_path, mp3_output_path)?;
        println!("✅ MP3 encoding completed");

        // Clean up temporary files
        self.cleanup_temp_files(&[temp_midi_path, temp_wav_path]);

        Ok(())
    }

    /// Converts MML text directly to MP3
    /// 
    /// # Arguments
    /// 
    /// * `mml_text` - MML code as string
    /// * `mp3_output_path` - Path for output MP3 file
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    pub fn convert_mml_text_to_mp3(&mut self, mml_text: &str, mp3_output_path: &str) -> Result<(), String> {
        // Validate MML content first
        self.mml_converter.validate_mml(mml_text)?;

        let temp_midi_path = "temp_conversion.mid";
        let temp_wav_path = "temp_conversion.wav";

        // Step 1: MML → MIDI
        println!("🎼 Converting MML to MIDI...");
        self.mml_converter.convert_mml_to_midi(mml_text, temp_midi_path)?;
        println!("✅ MIDI file generated");

        // Step 2: MIDI → WAV
        println!("🎹 Synthesizing MIDI to WAV...");
        self.midi_converter.convert_midi_to_wav(temp_midi_path, temp_wav_path)?;
        println!("✅ WAV file generated");

        // Step 3: WAV → MP3
        println!("🎵 Encoding WAV to MP3...");
        Mp3Encoder::convert_wav_to_mp3(temp_wav_path, mp3_output_path)?;
        println!("✅ MP3 encoding completed");

        // Clean up temporary files
        self.cleanup_temp_files(&[temp_midi_path, temp_wav_path]);

        Ok(())
    }

    /// Validates an MML file before conversion
    /// 
    /// # Arguments
    /// 
    /// * `mml_file_path` - Path to MML file to validate
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if valid, or `Err(String)` with validation error.
    pub fn validate_mml_file(&self, mml_file_path: &str) -> Result<(), String> {
        if !Path::new(mml_file_path).exists() {
            return Err(format!("MML file not found: {}", mml_file_path));
        }

        let mml_content = fs::read_to_string(mml_file_path)
            .map_err(|e| format!("Failed to read MML file: {}", e))?;

        self.mml_converter.validate_mml(&mml_content)
    }

    /// Cleans up temporary files created during conversion
    /// 
    /// # Arguments
    /// 
    /// * `file_paths` - Array of file paths to clean up
    fn cleanup_temp_files(&self, file_paths: &[&str]) {
        for &path in file_paths {
            if Path::new(path).exists() {
                if let Err(e) = fs::remove_file(path) {
                    eprintln!("⚠️  Warning: Failed to remove temporary file '{}': {}", path, e);
                } else {
                    println!("🧹 Cleaned up temporary file: {}", path);
                }
            }
        }
    }

    /// Gets conversion statistics and info
    /// 
    /// # Arguments
    /// 
    /// * `mml_file_path` - Path to MML file
    /// 
    /// # Returns
    /// 
    /// Returns file size and basic info about the MML file.
    pub fn get_conversion_info(&self, mml_file_path: &str) -> Result<String, String> {
        if !Path::new(mml_file_path).exists() {
            return Err(format!("MML file not found: {}", mml_file_path));
        }

        let metadata = fs::metadata(mml_file_path)
            .map_err(|e| format!("Failed to read file metadata: {}", e))?;

        let file_size = metadata.len();
        let mml_content = fs::read_to_string(mml_file_path)
            .map_err(|e| format!("Failed to read MML file: {}", e))?;

        let line_count = mml_content.lines().count();
        let char_count = mml_content.chars().count();

        Ok(format!(
            "📊 MML File Info:\n\
             • File size: {} bytes\n\
             • Lines: {}\n\
             • Characters: {}\n\
             • Estimated complexity: {}",
            file_size,
            line_count,
            char_count,
            if char_count > 1000 { "High" } else if char_count > 500 { "Medium" } else { "Low" }
        ))
    }
}

impl Default for ConversionPipeline {
    fn default() -> Self {
        Self::new().expect("Failed to create ConversionPipeline")
    }
}