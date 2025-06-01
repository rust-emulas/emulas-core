use std::path::Path;

use rust_emulas::sys::interfaces::{INes, ROMFs};
use rust_emulas::sys::rom_file::ROM;

fn write_bytes(values: &[u8]) -> Result<(), rust_emulas::sys::errors::FileErrors> {
    let path = Path::new("./mamaco.nes");
    let rom = ROM::new(&path)?;

    let INes {
        prg_rom,
        chr_rom,
        trainer,
        prg_size,
        chr_size,
        mapper,
        mirroring,
    } = rom.format;

    println!("PRG ROM Size: {}", prg_size);
    println!("CHR ROM Size: {}", chr_size);
    println!("Trainer Size: {}", trainer);
    println!("Mapper: {}", mapper);
    println!("Mirroring: {:?}", mirroring);

    Ok(())
}

fn main() {
    write_bytes(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05]).unwrap();
    println!("Size of ROM: {}", size_of::<ROM>());
    println!("Align of ROM: {}", align_of::<ROM>());
}
