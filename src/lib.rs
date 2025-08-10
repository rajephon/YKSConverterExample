/*!
 * YKS Converter Example Library
 * 
 * A high-quality MML/MIDI to MP3 converter library using yks_converter, FluidSynth, and LAME.
 * 
 * # Features
 * - MML (Music Macro Language) to MIDI conversion using yks_converter
 * - High-quality MIDI synthesis using FluidSynth
 * - SoundFont (.sf2) support for realistic instrument sounds
 * - MP3 encoding with LAME for optimal compression
 * - Complete conversion pipeline (MML → MIDI → WAV → MP3)
 * - Configurable audio quality settings
 * 
 * # Usage
 * 
 * ## Complete Pipeline
 * ```no_run
 * use yks_converter_example::pipeline::ConversionPipeline;
 * 
 * let mut pipeline = ConversionPipeline::new().unwrap();
 * pipeline.load_soundfont("soundfont.sf2").unwrap();
 * pipeline.convert_mml_to_mp3("song.mml", "output.mp3").unwrap();
 * ```
 * 
 * ## Individual Components
 * ```no_run
 * use yks_converter_example::midi_converter::MidiConverter;
 * use yks_converter_example::mp3_encoder::Mp3Encoder;
 * 
 * let mut converter = MidiConverter::new().unwrap();
 * converter.load_soundfont("soundfont.sf2").unwrap();
 * converter.convert_midi_to_wav("input.mid", "temp.wav").unwrap();
 * Mp3Encoder::convert_wav_to_mp3("temp.wav", "output.mp3").unwrap();
 * ```
 */

use std::os::raw::{c_char, c_int};

// FluidSynth FFI bindings
// These structures are opaque and only accessed through pointers

/// FluidSynth settings structure
#[repr(C)]
pub struct fluid_settings_t {
    _private: [u8; 0],
}

/// FluidSynth synthesizer structure
#[repr(C)]
pub struct fluid_synth_t {
    _private: [u8; 0],
}

/// FluidSynth MIDI player structure
#[repr(C)]
pub struct fluid_player_t {
    _private: [u8; 0],
}


/// FluidSynth player status: currently playing
pub const FLUID_PLAYER_PLAYING: u32 = 1;

#[link(name = "fluidsynth")]
unsafe extern "C" {
    pub fn new_fluid_settings() -> *mut fluid_settings_t;
    pub fn delete_fluid_settings(settings: *mut fluid_settings_t);
    pub fn fluid_settings_setstr(settings: *mut fluid_settings_t, name: *const c_char, str: *const c_char) -> c_int;
    pub fn fluid_settings_setnum(settings: *mut fluid_settings_t, name: *const c_char, val: f64) -> c_int;
    pub fn fluid_settings_setint(settings: *mut fluid_settings_t, name: *const c_char, val: c_int) -> c_int;
    
    pub fn new_fluid_synth(settings: *mut fluid_settings_t) -> *mut fluid_synth_t;
    pub fn delete_fluid_synth(synth: *mut fluid_synth_t);
    pub fn fluid_synth_sfload(synth: *mut fluid_synth_t, filename: *const c_char, reset_presets: c_int) -> c_int;
    
    pub fn new_fluid_player(synth: *mut fluid_synth_t) -> *mut fluid_player_t;
    pub fn delete_fluid_player(player: *mut fluid_player_t);
    pub fn fluid_player_add(player: *mut fluid_player_t, midifile: *const c_char) -> c_int;
    pub fn fluid_player_play(player: *mut fluid_player_t) -> c_int;
    pub fn fluid_player_get_status(player: *mut fluid_player_t) -> c_int;
    
    // Audio synthesis functions
    pub fn fluid_synth_write_s16(synth: *mut fluid_synth_t, len: c_int, lbuf: *mut i16, loff: c_int, lincr: c_int, rbuf: *mut i16, roff: c_int, rincr: c_int) -> c_int;
    
    // Program change function
    pub fn fluid_synth_program_change(synth: *mut fluid_synth_t, chan: c_int, program: c_int) -> c_int;
}

pub mod midi_converter;
pub mod mp3_encoder;
pub mod lame_bindings;
pub mod mml_converter;
pub mod pipeline;