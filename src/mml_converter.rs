/*!
 * MML to MIDI Converter Module
 * 
 * Converts MML (Music Macro Language) code from Mabinogi online game 
 * to MIDI format using the yks_converter library.
 */

use yks_converter::YksConverter;
use std::fs;
use std::path::Path;

/// MML to MIDI converter using yks_converter library
/// 
/// This converter handles MML files from Mabinogi online game and converts
/// them to standard MIDI format for further processing.
/// 
/// # Example
/// 
/// ```no_run
/// use yks_converter_example::mml_converter::MmlConverter;
/// 
/// let converter = MmlConverter::new();
/// converter.convert_mml_file_to_midi("song.mml", "output.mid").unwrap();
/// ```
pub struct MmlConverter {
    instrument: u8,
}

impl MmlConverter {
    /// Creates a new MML converter instance with default instrument (0)
    pub fn new() -> Self {
        MmlConverter { instrument: 0 }
    }

    /// Sets the instrument for MML conversion
    /// 
    /// # Arguments
    /// 
    /// * `instrument` - MIDI instrument number (0-127)
    pub fn set_instrument(&mut self, instrument: u8) {
        self.instrument = instrument;
    }

    /// Converts MML text to MIDI format
    /// 
    /// # Arguments
    /// 
    /// * `mml_text` - MML code as string
    /// * `output_path` - Path for the output MIDI file
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    pub fn convert_mml_to_midi(&self, mml_text: &str, output_path: &str) -> Result<(), String> {
        let converter = YksConverter::new(mml_text.to_string(), self.instrument);
        
        let midi_data = converter.to_buffer()
            .ok_or_else(|| "Failed to convert MML to MIDI buffer".to_string())?;
        
        fs::write(output_path, midi_data.as_slice())
            .map_err(|e| format!("Failed to write MIDI file: {}", e))?;
        
        Ok(())
    }

    /// Converts MML file to MIDI file
    /// 
    /// # Arguments
    /// 
    /// * `mml_file_path` - Path to the input MML file
    /// * `midi_file_path` - Path for the output MIDI file
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use yks_converter_example::mml_converter::MmlConverter;
    /// 
    /// let converter = MmlConverter::new();
    /// converter.convert_mml_file_to_midi("song.mml", "output.mid")?;
    /// # Ok::<(), String>(())
    /// ```
    pub fn convert_mml_file_to_midi(&self, mml_file_path: &str, midi_file_path: &str) -> Result<(), String> {
        // Check if MML file exists
        if !Path::new(mml_file_path).exists() {
            return Err(format!("MML file not found: {}", mml_file_path));
        }

        // Read MML file content
        let mml_content = fs::read_to_string(mml_file_path)
            .map_err(|e| format!("Failed to read MML file '{}': {}", mml_file_path, e))?;

        // Convert MML to MIDI
        self.convert_mml_to_midi(&mml_content, midi_file_path)?;

        Ok(())
    }

    /// Validates MML content before conversion
    /// 
    /// # Arguments
    /// 
    /// * `mml_text` - MML code as string
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if valid, or `Err(String)` with validation error.
    pub fn validate_mml(&self, mml_text: &str) -> Result<(), String> {
        if mml_text.trim().is_empty() {
            return Err("MML content is empty".to_string());
        }

        // Basic MML syntax validation - check for common MML patterns
        if !mml_text.chars().any(|c| "ABCDEFGRLTVabcdefgrltvN0123456789".contains(c)) {
            return Err("Invalid MML format: no recognizable MML commands found".to_string());
        }

        Ok(())
    }
}

impl Default for MmlConverter {
    fn default() -> Self {
        Self::new()
    }
}