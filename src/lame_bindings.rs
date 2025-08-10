/*!
 * LAME MP3 Encoder Bindings
 * 
 * Safe Rust bindings for the LAME MP3 encoder library.
 * Provides high-quality MP3 encoding with configurable settings.
 */

use libc::{c_int, c_uchar};

/// LAME global flags structure (opaque)
#[repr(C)]
pub struct lame_global_flags {
    _private: [u8; 0],
}

/// Type alias for LAME global flags pointer
pub type LameT = *mut lame_global_flags;

#[link(name = "mp3lame")]
unsafe extern "C" {
    pub fn lame_init() -> LameT;
    pub fn lame_close(gfp: LameT) -> c_int;
    pub fn lame_init_params(gfp: LameT) -> c_int;
    
    // Set parameters
    pub fn lame_set_in_samplerate(gfp: LameT, sample_rate: c_int) -> c_int;
    pub fn lame_set_num_channels(gfp: LameT, num_channels: c_int) -> c_int;
    pub fn lame_set_out_samplerate(gfp: LameT, sample_rate: c_int) -> c_int;
    pub fn lame_set_brate(gfp: LameT, brate: c_int) -> c_int;
    pub fn lame_set_quality(gfp: LameT, quality: c_int) -> c_int;
    
    // Encoding functions - use short (i16) instead of int
    pub fn lame_encode_buffer_interleaved(
        gfp: LameT,
        pcm: *const i16,
        num_samples: c_int,
        mp3buf: *mut c_uchar,
        mp3buf_size: c_int,
    ) -> c_int;
    
    pub fn lame_encode_buffer(
        gfp: LameT,
        buffer_l: *const i16,
        buffer_r: *const i16,
        nsamples: c_int,
        mp3buf: *mut c_uchar,
        mp3buf_size: c_int,
    ) -> c_int;
    
    pub fn lame_encode_flush(
        gfp: LameT,
        mp3buf: *mut c_uchar,
        size: c_int,
    ) -> c_int;
}

/// High-quality MP3 encoder using LAME
/// 
/// Provides a safe wrapper around the LAME encoder with optimal settings
/// for music production and audio conversion.
pub struct LameEncoder {
    lame: LameT,
}

impl LameEncoder {
    /// Creates a new LAME encoder with specified settings
    /// 
    /// # Arguments
    /// 
    /// * `sample_rate` - Audio sample rate (e.g., 44100 for CD quality)
    /// * `channels` - Number of audio channels (1 for mono, 2 for stereo)
    /// * `bitrate` - MP3 bitrate in kbps (e.g., 192 for high quality)
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(LameEncoder)` on success, or `Err(String)` with error message.
    pub fn new(sample_rate: u32, channels: u16, bitrate: u32) -> Result<Self, String> {
        unsafe {
            let lame = lame_init();
            if lame.is_null() {
                return Err("Failed to initialize LAME encoder".to_string());
            }

            lame_set_in_samplerate(lame, sample_rate as c_int);
            lame_set_num_channels(lame, channels as c_int);
            lame_set_out_samplerate(lame, sample_rate as c_int);
            lame_set_brate(lame, bitrate as c_int);
            lame_set_quality(lame, 0); // Highest quality (0 is best, 9 is worst)

            if lame_init_params(lame) != 0 {
                lame_close(lame);
                return Err("Failed to initialize LAME parameters".to_string());
            }

            Ok(LameEncoder { lame })
        }
    }

    pub fn encode_buffer(
        &mut self,
        left: &[i16],
        right: &[i16],
        mp3_buffer: &mut [u8],
    ) -> Result<usize, String> {
        if left.len() != right.len() {
            return Err("Left and right channel buffers must have the same length".to_string());
        }

        unsafe {
            let result = lame_encode_buffer(
                self.lame,
                left.as_ptr(),
                right.as_ptr(),
                left.len() as c_int,
                mp3_buffer.as_mut_ptr(),
                mp3_buffer.len() as c_int,
            );

            if result < 0 {
                Err("LAME encoding error".to_string())
            } else {
                Ok(result as usize)
            }
        }
    }

    pub fn flush(&mut self, mp3_buffer: &mut [u8]) -> Result<usize, String> {
        unsafe {
            let result = lame_encode_flush(
                self.lame,
                mp3_buffer.as_mut_ptr(),
                mp3_buffer.len() as c_int,
            );

            if result < 0 {
                Err("LAME flush error".to_string())
            } else {
                Ok(result as usize)
            }
        }
    }
}

impl Drop for LameEncoder {
    fn drop(&mut self) {
        unsafe {
            if !self.lame.is_null() {
                lame_close(self.lame);
            }
        }
    }
}