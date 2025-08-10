# YKS Converter Example

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)

Example implementation of a high-quality MML/MIDI to MP3 converter, demonstrating the use of yks_converter, FluidSynth, and LAME libraries in Rust. Supports Mabinogi MML (Music Macro Language) files and standard MIDI files.

## ✨ Features

- 🎼 **Mabinogi MML support** using yks_converter for MML to MIDI conversion
- 🎹 **High-quality MIDI synthesis** using FluidSynth with SoundFont support
- 🎵 **Professional MP3 encoding** with LAME at 192kbps
- 🔊 **Optimized audio processing** with 44.1kHz stereo output
- 📦 **Simple command-line interface** for both MML and MIDI files
- ⚡ **Fast and efficient** conversion pipeline (MML → MIDI → WAV → MP3)
- 🎛️ **Configurable effects** including reverb and chorus

## 📋 Requirements

### System Dependencies

Before building, you need to install the following system libraries:

**macOS (Homebrew):**
```bash
brew install fluid-synth lame pkg-config
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install libfluidsynth-dev libmp3lame-dev pkg-config build-essential
```

**CentOS/RHEL/Fedora:**
```bash
# Fedora
sudo dnf install fluidsynth-devel lame-devel pkgconf-devel

# CentOS/RHEL (with EPEL)
sudo yum install fluidsynth-devel lame-devel pkgconfig
```

### Rust

You'll need Rust 1.70 or later. Install from [rustup.rs](https://rustup.rs/).

## 🚀 Installation

### From Source

```bash
git clone https://github.com/yourusername/yks-converter-example.git
cd yks-converter-example
cargo build --release
```

The binary will be available at `target/release/yks-converter-example`.

### Using Cargo

```bash
cargo install yks-converter-example
```

## 💫 Usage

### Basic Usage

```bash
yks-converter-example <input_file> <soundfont_file> <output_mp3> [instrument_number]
```

### Examples

```bash
# Convert MML file to MP3 (default instrument 0)
yks-converter-example song.mml piano.sf2 output.mp3

# Convert MML file to MP3 with specific instrument
yks-converter-example song.mml piano.sf2 output.mp3 0    # Lute
yks-converter-example song.mml piano.sf2 output.mp3 1    # Ukulele
yks-converter-example song.mml piano.sf2 output.mp3 2    # Mandolin

# Convert MIDI file to MP3 with instrument selection
yks-converter-example song.mid piano.sf2 output.mp3 54   # Flute
```

### Arguments

- `input_file` - Input MML file (.mml) or MIDI file (.mid, .midi)
- `soundfont_file` - SoundFont file (.sf2) for realistic instrument sounds
- `output_mp3` - Output MP3 file path
- `instrument_number` - Optional: MIDI instrument number (0-127, default: 0)

### Sample Output

```
🎵 YKS Converter Example - Starting MML to MP3 conversion...
📂 Input file: song.mml
🎹 SoundFont: piano.sf2
🎼 Instrument: 40
🎧 Output: output.mp3

📊 MML File Info:
• File size: 156 bytes
• Lines: 3
• Characters: 142
• Estimated complexity: Low

✅ Conversion pipeline initialized
✅ SoundFont loaded
✅ Instrument 40 set
🎼 Converting MML to MIDI...
✅ MIDI file generated
🎹 Synthesizing MIDI to WAV...
✅ WAV file generated
🎵 Encoding WAV to MP3...
✅ MP3 encoding completed
🧹 Cleaned up temporary file: temp_conversion.mid
🧹 Cleaned up temporary file: temp_conversion.wav
🎉 Conversion completed successfully!
📁 Output saved to: output.mp3
```

## 🎼 MML Files

This converter supports MML (Music Macro Language) files from Mabinogi online game. MML uses simple text commands to represent musical notes, tempo, and other musical elements.

### Example MML Format

```
MML@t120l4cdefgab>c,l4<cdefgab>c,l4>cdefgab>c;
```

- `t120` - Set tempo to 120 BPM
- `l4` - Set default note length to quarter notes
- `cdefgab` - Play notes C, D, E, F, G, A, B
- `>` - Move up one octave
- `<` - Move down one octave
- `,` - Separate tracks (up to 3 tracks supported)
- `;` - End of MML

## ⚙️ Audio Quality Settings

The converter uses the following optimized settings:

- **Sample Rate:** 44.1 kHz (CD quality)
- **Bit Depth:** 16-bit
- **Channels:** Stereo (2 channels)
- **MP3 Bitrate:** 192 kbps (high quality)
- **LAME Quality:** 0 (highest quality setting)
- **Buffer Size:** 4096 samples (optimal for quality)
- **Effects:** Reverb and chorus enabled

## 📚 Code Structure

This example project demonstrates how to integrate yks_converter, FluidSynth, and LAME libraries in Rust:

- `src/lib.rs` - Library structure and FFI bindings
- `src/mml_converter.rs` - MML to MIDI conversion using yks_converter
- `src/midi_converter.rs` - MIDI to WAV conversion using FluidSynth
- `src/mp3_encoder.rs` - WAV to MP3 encoding using LAME
- `src/lame_bindings.rs` - Safe LAME encoder wrapper
- `src/pipeline.rs` - Complete MML/MIDI to MP3 conversion pipeline
- `src/main.rs` - Command-line interface
- `build.rs` - Build configuration for native libraries

### Using the Complete Pipeline

```rust
use yks_converter_example::pipeline::ConversionPipeline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pipeline = ConversionPipeline::new()?;
    pipeline.load_soundfont("piano.sf2")?;
    pipeline.set_instrument(32)?; // Use piano
    pipeline.convert_mml_to_mp3("song.mml", "output.mp3")?;
    
    println!("MML to MP3 conversion completed!");
    Ok(())
}
```

### Using Individual Components

```rust
use yks_converter_example::midi_converter::MidiConverter;
use yks_converter_example::mp3_encoder::Mp3Encoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut converter = MidiConverter::new()?;
    converter.load_soundfont("piano.sf2")?;
    converter.set_instrument(25)?; // Use steel guitar
    converter.convert_midi_to_wav("song.mid", "temp.wav")?;
    Mp3Encoder::convert_wav_to_mp3("temp.wav", "output.mp3")?;
    
    std::fs::remove_file("temp.wav")?; // Clean up
    println!("Conversion completed!");
    Ok(())
}
```

## 🔧 Building

### Development Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

## 🐛 Troubleshooting

### Common Issues

**"FluidSynth library not found"**
- Make sure FluidSynth development libraries are installed
- Check that `pkg-config` can find fluidsynth: `pkg-config --exists fluidsynth`

**"LAME library not found"**
- Install LAME development libraries
- On macOS: `brew install lame`
- On Ubuntu: `sudo apt install libmp3lame-dev`

**"Failed to load soundfont"**
- Verify the SoundFont file exists and is valid
- Try a different SoundFont file
- Check file permissions

### Verbose Output

For debugging, you can inspect intermediate files by modifying the source to keep temporary WAV files.

## 🤝 Contributing

We welcome contributions! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Install dependencies (see Requirements section)
4. Make your changes
5. Run tests (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Third-Party Dependencies

This project uses the following libraries:

- **FluidSynth**: Licensed under LGPL v2.1 - [FluidSynth License](https://github.com/FluidSynth/fluidsynth/blob/master/LICENSE)
- **LAME**: Licensed under LGPL v2 - [LAME License](https://lame.sourceforge.io/license.txt)  
- **yks_converter**: Check the [yks_converter crate](https://crates.io/crates/yks_converter) for license information

Note: This project dynamically links to FluidSynth and LAME libraries, maintaining license compatibility while keeping the example code under MIT license.

## 🙏 Acknowledgments

- [FluidSynth](https://www.fluidsynth.org/) - Software synthesizer
- [LAME](https://lame.sourceforge.io/) - MP3 encoder
- [hound](https://github.com/ruuda/hound) - WAV file I/O
- Rust community for excellent audio libraries

## 📊 Performance

Typical conversion times on modern hardware:
- Simple MIDI (1-2 minutes): 2-5 seconds
- Complex MIDI (5-10 minutes): 10-30 seconds
- Large orchestral pieces: 30-60 seconds

Output file sizes:
- 192kbps MP3: ~1.4 MB per minute of audio
- Comparable quality to commercial MIDI converters
