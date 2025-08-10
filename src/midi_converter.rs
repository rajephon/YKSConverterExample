/*!
 * MIDI Converter Module
 * 
 * Provides high-quality MIDI to WAV conversion using FluidSynth synthesis.
 * Supports SoundFont files for realistic instrument sounds.
 */

use crate::*;
use hound::{WavSpec, WavWriter};
use std::ffi::CString;

/// High-quality MIDI converter using FluidSynth synthesis
/// 
/// This converter uses FluidSynth to synthesize MIDI files with SoundFont support,
/// producing high-quality WAV output suitable for MP3 encoding.
/// 
/// # Example
/// 
/// ```no_run
/// use yks_converter_example::midi_converter::MidiConverter;
/// 
/// let mut converter = MidiConverter::new().unwrap();
/// converter.load_soundfont("soundfont.sf2").unwrap();
/// converter.convert_midi_to_wav("input.mid", "output.wav").unwrap();
/// ```
pub struct MidiConverter {
    settings: *mut fluid_settings_t,
    synth: *mut fluid_synth_t,
}

impl MidiConverter {
    /// Creates a new MIDI converter with optimized FluidSynth settings
    /// 
    /// Initializes FluidSynth with high-quality settings:
    /// - Sample rate: 44.1 kHz
    /// - Stereo output (2 channels)
    /// - High polyphony (256 voices)
    /// - Reverb and chorus enabled
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(MidiConverter)` on success, or `Err(String)` with error message.
    pub fn new() -> Result<Self, String> {
        unsafe {
            let settings = new_fluid_settings();
            if settings.is_null() {
                return Err("Failed to create FluidSynth settings".to_string());
            }

            // Configure FluidSynth for high quality audio
            fluid_settings_setnum(settings, CString::new("synth.sample-rate").unwrap().as_ptr(), 44100.0);
            fluid_settings_setint(settings, CString::new("synth.audio-channels").unwrap().as_ptr(), 2);
            fluid_settings_setint(settings, CString::new("synth.audio-groups").unwrap().as_ptr(), 2);
            fluid_settings_setnum(settings, CString::new("synth.gain").unwrap().as_ptr(), 1.0);
            fluid_settings_setint(settings, CString::new("synth.polyphony").unwrap().as_ptr(), 256);
            // Enable reverb and chorus with proper integer settings
            fluid_settings_setint(settings, CString::new("synth.reverb.active").unwrap().as_ptr(), 1);
            fluid_settings_setint(settings, CString::new("synth.chorus.active").unwrap().as_ptr(), 1);

            let synth = new_fluid_synth(settings);
            if synth.is_null() {
                delete_fluid_settings(settings);
                return Err("Failed to create FluidSynth".to_string());
            }

            Ok(MidiConverter {
                settings,
                synth,
            })
        }
    }

    /// Loads a SoundFont (.sf2) file for synthesis
    /// 
    /// # Arguments
    /// 
    /// * `sf2_path` - Path to the SoundFont file (.sf2)
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    pub fn load_soundfont(&mut self, sf2_path: &str) -> Result<(), String> {
        unsafe {
            let sf2_cstring = CString::new(sf2_path).map_err(|_| "Invalid SF2 path")?;
            let sfont_id = fluid_synth_sfload(self.synth, sf2_cstring.as_ptr(), 1);
            if sfont_id == -1 {
                return Err("Failed to load soundfont".to_string());
            }
        }
        Ok(())
    }

    /// Sets the instrument for MIDI channel 0
    /// 
    /// # Arguments
    /// 
    /// * `program` - MIDI program number (0-127)
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    pub fn set_instrument(&mut self, program: u8) -> Result<(), String> {
        unsafe {
            let result = fluid_synth_program_change(self.synth, 0, program as i32);
            if result != 0 {
                return Err(format!("Failed to change instrument to program {}", program));
            }
        }
        Ok(())
    }

    /// Converts a MIDI file to WAV format using FluidSynth synthesis
    /// 
    /// # Arguments
    /// 
    /// * `midi_path` - Path to the input MIDI file (.mid, .midi)
    /// * `wav_path` - Path for the output WAV file
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err(String)` with error message.
    /// 
    /// # Quality Settings
    /// 
    /// - 44.1 kHz sample rate
    /// - 16-bit stereo output
    /// - 4096 sample buffer for optimal quality
    pub fn convert_midi_to_wav(&mut self, midi_path: &str, wav_path: &str) -> Result<(), String> {
        unsafe {
            let spec = WavSpec {
                channels: 2,
                sample_rate: 44100,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };

            let mut writer = WavWriter::create(wav_path, spec)
                .map_err(|e| format!("Failed to create WAV writer: {}", e))?;

            let player = new_fluid_player(self.synth);
            if player.is_null() {
                return Err("Failed to create MIDI player".to_string());
            }

            let midi_cstring = CString::new(midi_path).map_err(|_| "Invalid MIDI path")?;
            if fluid_player_add(player, midi_cstring.as_ptr()) != 0 {
                delete_fluid_player(player);
                return Err("Failed to add MIDI file to player".to_string());
            }

            fluid_player_play(player);

            const BUFFER_SIZE: usize = 4096; // Larger buffer for better quality
            let mut left_buffer = vec![0i16; BUFFER_SIZE];
            let mut right_buffer = vec![0i16; BUFFER_SIZE];

            while fluid_player_get_status(player) == FLUID_PLAYER_PLAYING as i32 {
                let result = fluid_synth_write_s16(
                    self.synth,
                    BUFFER_SIZE as i32,
                    left_buffer.as_mut_ptr(),
                    0,
                    1,
                    right_buffer.as_mut_ptr(),
                    0,
                    1,
                );

                if result != 0 {
                    break;
                }

                for i in 0..BUFFER_SIZE {
                    writer.write_sample(left_buffer[i])
                        .map_err(|e| format!("Failed to write left sample: {}", e))?;
                    writer.write_sample(right_buffer[i])
                        .map_err(|e| format!("Failed to write right sample: {}", e))?;
                }
            }

            delete_fluid_player(player);
            writer.finalize().map_err(|e| format!("Failed to finalize WAV: {}", e))?;
        }
        Ok(())
    }
}

impl Drop for MidiConverter {
    fn drop(&mut self) {
        unsafe {
            if !self.synth.is_null() {
                delete_fluid_synth(self.synth);
            }
            if !self.settings.is_null() {
                delete_fluid_settings(self.settings);
            }
        }
    }
}