use std::path::Path;

use rust_emulas::memory::{Bus, BusInterface};
use rust_emulas::sys::interfaces::ROMFs;
use rust_emulas::sys::rom_file::ROM;

fn main() {
    let bus = &mut Bus::new(Vec::new());
    let rom_path: &Path = Path::new("./mamaco.nes");
    let rom = ROM::new(&rom_path).expect("Failed to load ROM");
    rom.write_rom_memory(rom_path, bus)
        .expect("Failed to write ROM to bus");

    // Assuming Bus has a method to get a reference to its memory, e.g., bus.memory()
    println!(
        "ROM loaded successfully! {:?}",
        &bus.prg_rom[bus.prg_rom.len() - 1000..bus.prg_rom.len()]
    );
    println!("Size of ROM: {}", size_of::<ROM>());
    println!("Align of ROM: {}", align_of::<ROM>());
}
