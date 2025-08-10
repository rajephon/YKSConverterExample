fn main() {
    let _fluidsynth = pkg_config::probe_library("fluidsynth")
        .expect("FluidSynth library not found. Please install FluidSynth development package.");
    
    // Add LAME library path and link
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    println!("cargo:rustc-link-lib=mp3lame");
    
    println!("cargo:rerun-if-changed=build.rs");
}