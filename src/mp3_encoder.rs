/*!
 * MP3 Encoder Module
 * 
 * High-quality WAV to MP3 conversion using LAME encoder.
 * Supports both mono and stereo WAV files with optimal quality settings.
 */

use crate::lame_bindings::LameEncoder;
use hound::{WavReader, SampleFormat};
use std::fs::File;
use std::io::{BufWriter, Write};

/// High-quality MP3 encoder using LAME
/// 
/// This encoder converts WAV files to MP3 format using the LAME library
/// with optimized settings for maximum quality.
pub struct Mp3Encoder;

impl Mp3Encoder {
    /// Converts a WAV file to MP3 format using LAME encoder
    /// 
    /// # Arguments
    /// 
    /// * `wav_path` - Path to the input WAV file (16-bit, mono or stereo)
    /// * `mp3_path` - Path for the output MP3 file
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    /// 
    /// # Quality Settings
    /// 
    /// - 192 kbps bitrate for high quality
    /// - Highest quality setting (quality=0)
    /// - Supports both mono and stereo input
    /// - 1152 sample frame processing for optimal compression
    pub fn convert_wav_to_mp3(wav_path: &str, mp3_path: &str) -> Result<(), String> {
        let mut reader = WavReader::open(wav_path)
            .map_err(|e| format!("Failed to open WAV file: {}", e))?;
        
        let spec = reader.spec();
        if spec.sample_format != SampleFormat::Int || spec.bits_per_sample != 16 {
            return Err("Only 16-bit integer WAV files are supported".to_string());
        }

        let mut encoder = LameEncoder::new(spec.sample_rate, spec.channels, 192)?; // Higher bitrate for better quality
        
        let mut mp3_file = BufWriter::new(
            File::create(mp3_path).map_err(|e| format!("Failed to create MP3 file: {}", e))?
        );

        const BUFFER_SIZE: usize = 1152; // MP3 frame size
        let mut mp3_buffer = vec![0u8; 7200]; // 1.25 * BUFFER_SIZE + 7200 for safety
        
        if spec.channels == 1 {
            // Mono processing
            let mut mono_buffer = Vec::new();
            for sample in reader.samples::<i16>() {
                mono_buffer.push(sample.map_err(|e| format!("Failed to read sample: {}", e))?);
                
                if mono_buffer.len() >= BUFFER_SIZE {
                    // Duplicate mono to stereo for LAME
                    let stereo_left = mono_buffer[..BUFFER_SIZE].to_vec();
                    let stereo_right = stereo_left.clone();
                    
                    let encoded_size = encoder.encode_buffer(&stereo_left, &stereo_right, &mut mp3_buffer)?;
                    if encoded_size > 0 {
                        mp3_file.write_all(&mp3_buffer[..encoded_size])
                            .map_err(|e| format!("Failed to write MP3 data: {}", e))?;
                    }
                    
                    mono_buffer.clear();
                }
            }
            
            // Process remaining samples
            if !mono_buffer.is_empty() {
                mono_buffer.resize(BUFFER_SIZE, 0); // Pad with zeros
                let stereo_left = mono_buffer;
                let stereo_right = stereo_left.clone();
                
                let encoded_size = encoder.encode_buffer(&stereo_left, &stereo_right, &mut mp3_buffer)?;
                if encoded_size > 0 {
                    mp3_file.write_all(&mp3_buffer[..encoded_size])
                        .map_err(|e| format!("Failed to write MP3 data: {}", e))?;
                }
            }
            
        } else if spec.channels == 2 {
            // Stereo processing
            let samples: Result<Vec<i16>, _> = reader.samples::<i16>().collect();
            let samples = samples.map_err(|e| format!("Failed to read samples: {}", e))?;
            
            for chunk in samples.chunks(BUFFER_SIZE * 2) {
                let mut left = Vec::new();
                let mut right = Vec::new();
                
                for pair in chunk.chunks_exact(2) {
                    left.push(pair[0]);
                    right.push(pair[1]);
                }
                
                // Pad if necessary
                if left.len() < BUFFER_SIZE {
                    left.resize(BUFFER_SIZE, 0);
                    right.resize(BUFFER_SIZE, 0);
                }
                
                let encoded_size = encoder.encode_buffer(&left, &right, &mut mp3_buffer)?;
                if encoded_size > 0 {
                    mp3_file.write_all(&mp3_buffer[..encoded_size])
                        .map_err(|e| format!("Failed to write MP3 data: {}", e))?;
                }
            }
        } else {
            return Err("Only mono and stereo WAV files are supported".to_string());
        }
        
        // Flush encoder
        let encoded_size = encoder.flush(&mut mp3_buffer)?;
        if encoded_size > 0 {
            mp3_file.write_all(&mp3_buffer[..encoded_size])
                .map_err(|e| format!("Failed to write final MP3 data: {}", e))?;
        }
        
        mp3_file.flush().map_err(|e| format!("Failed to flush MP3 file: {}", e))?;
        
        Ok(())
    }
}